use alloc::borrow::ToOwned as _;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString as _;
use alloc::vec::Vec;

use object::Object as _;
use object::ObjectSection as _;
use object::ObjectSymbol as _;

use crate::BoxResult;
use crate::cxx_auto_artifact_info::CxxAutoArtifactInfo;

/// Symbol-name prefix of every record `CXX_AUTO_EXPORT` defines.
const SYMBOL_PREFIX: &str = "cxx_auto_type_";
/// `cxx_auto::record_magic`, as the bytes it holds in the object file.
const MAGIC: &[u8; 8] = b"cxx_auto";
/// `cxx_auto::record_version`: the wire format this reader decodes.
const VERSION: u32 = 1;
/// Bytes before the encoded spec: magic, version, spec length, size,
/// alignment, and flags.
const HEADER_LEN: usize = 40;
/// NUL-terminated strings in the encoded spec, in `TypeRecord` order.
const SPEC_FIELDS: usize = 7;

/// Bit positions of `TypeRecord::flags`, mirroring `cxx_auto::record_bit`.
mod bit
{
    /// `CXX` may pass the type by value.
    pub(super) const CXX_EXTERN_TYPE_TRIVIAL: u32 = 0;
    /// Rust may move the type after pinning.
    pub(super) const UNPIN: u32 = 1;
    /// The type opted into `Send`.
    pub(super) const SEND: u32 = 2;
    /// The type opted into `Sync`.
    pub(super) const SYNC: u32 = 3;
    /// Rust must run the C++ destructor.
    pub(super) const DROP: u32 = 4;
    /// Rust may duplicate the type bitwise.
    pub(super) const COPY: u32 = 5;
    /// Rust may default-construct the type.
    pub(super) const DEFAULT_NEW: u32 = 6;
    /// Rust may invoke the C++ copy constructor.
    pub(super) const COPY_NEW: u32 = 7;
    /// Rust may invoke the C++ move constructor.
    pub(super) const MOVE_NEW: u32 = 8;
    /// Rust exposes `Eq`.
    pub(super) const EQ: u32 = 9;
    /// Rust exposes `PartialEq`.
    pub(super) const PARTIAL_EQ: u32 = 10;
    /// Rust exposes `PartialOrd`.
    pub(super) const PARTIAL_ORD: u32 = 11;
    /// Rust exposes `Ord`.
    pub(super) const ORD: u32 = 12;
    /// Rust delegates `Hash` to `std::hash`.
    pub(super) const HASH: u32 = 13;
    /// Rust delegates `Debug` to C++.
    pub(super) const DEBUG: u32 = 14;
    /// Rust delegates `Display` to C++.
    pub(super) const DISPLAY: u32 = 15;
    /// C++ declares its own `operator!=`.
    pub(super) const OPERATOR_NOT_EQUAL: u32 = 16;
    /// Number of defined bits; every higher bit must be clear.
    pub(super) const COUNT: u32 = 17;
}

/// Read every type record defined in one compiled object file.
///
/// # Specification
/// - requires: `path` names an ELF, Mach-O, or COFF object compiled for a
///   little-endian target.
/// - provides: one artifact per defined `cxx_auto_type_*` symbol, in symbol
///   table order, each with an empty descendant list.
/// - fails: the file cannot be read or parsed, targets a big-endian machine, or
///   holds a record that does not decode; the error names the file and the
///   record.
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: a record compiled by the C++ compiler must decode to the
///   layout and capabilities the generated binding relies on.
/// - witness: `tests/dynamic_binding.rs`
///   (`loads_generated_comparison_binding`).
#[cfg(feature = "std")]
pub fn read_object(path: &std::path::Path) -> BoxResult<Vec<CxxAutoArtifactInfo>>
{
    let data = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let file = object::File::parse(data.as_slice())
        .map_err(|error| format!("{}: {error}", path.display()))?;
    if !file.is_little_endian() {
        return Err(format!("{}: big-endian targets are unsupported", path.display()).into());
    }
    let mut infos = Vec::new();
    for symbol in file.symbols() {
        let Some(id) = record_id(symbol.name()?)
        else {
            continue;
        };
        if !symbol.is_definition() {
            continue;
        }
        let bytes = symbol_bytes(&file, &symbol)
            .and_then(decode)
            .map_err(|error| format!("{}: record `{id}`: {error}", path.display()))?;
        infos.push(bytes);
    }
    Ok(infos)
}

/// Recognise a record symbol and return its identifier.
///
/// # Specification
/// - provides: the text after `cxx_auto_type_`, accepting the leading `_`
///   Mach-O adds to C symbols; `None` for any other symbol.
/// - panics: none.
fn record_id(name: &str) -> Option<&str>
{
    name.strip_prefix(SYMBOL_PREFIX)
        .or_else(|| name.strip_prefix('_')?.strip_prefix(SYMBOL_PREFIX))
}

/// Borrow the bytes from a symbol's address to the end of its section.
///
/// # Specification
/// - provides: the section contents starting at the symbol, since Mach-O
///   records no symbol sizes; `decode` bounds the record itself.
/// - fails: the symbol has no section with file contents, or its address lies
///   outside that section.
/// - panics: none.
fn symbol_bytes<'data>(
    file: &object::File<'data>,
    symbol: &object::Symbol<'data, '_>,
) -> BoxResult<&'data [u8]>
{
    let object::SymbolSection::Section(index) = symbol.section()
    else {
        return Err("the symbol is not defined in a section".into());
    };
    let section = file.section_by_index(index)?;
    let offset = symbol
        .address()
        .checked_sub(section.address())
        .ok_or("the symbol lies before its section")?;
    let offset = usize::try_from(offset)?;
    section
        .data()?
        .get(offset ..)
        .ok_or_else(|| "the symbol lies past its section's contents".into())
}

/// Decode one record into the artifact it describes.
///
/// # Specification
/// - requires: `bytes` starts at a record and may continue past it.
/// - provides: the record's names, lifetimes, layout, and capabilities, with
///   `path_descendants` left empty for the caller to fill.
/// - fails: wrong magic or version, truncation, a spec that is not seven UTF-8
///   strings, an invalid Rust identifier or lifetime list, a zero or
///   non-power-of-two alignment, or an unknown flag bit.
/// - panics: none.
fn decode(bytes: &[u8]) -> BoxResult<CxxAutoArtifactInfo>
{
    if bytes.get(.. MAGIC.len()) != Some(MAGIC.as_slice()) {
        return Err("bad magic; not a cxx-auto type record".into());
    }
    let version = read_u32(bytes, 8)?;
    if version != VERSION {
        return Err(format!("record version {version}, expected {VERSION}").into());
    }
    let spec_len = usize::try_from(read_u32(bytes, 12)?)?;
    let size = usize::try_from(read_u64(bytes, 16)?)?;
    let align = usize::try_from(read_u64(bytes, 24)?)?;
    let flags = read_u64(bytes, 32)?;
    if !align.is_power_of_two() {
        return Err(format!("alignment {align} is not a power of two").into());
    }
    if flags.checked_shr(bit::COUNT).is_some_and(|high| high != 0) {
        return Err(format!("unknown flag bits in {flags:#x}").into());
    }
    let spec_end = HEADER_LEN
        .checked_add(spec_len)
        .ok_or("spec length overflows")?;
    let spec = bytes
        .get(HEADER_LEN .. spec_end)
        .ok_or("the record is truncated")?;
    let [
        rust_path,
        rust_name,
        rust_lifetimes,
        cxx_name,
        cxx_namespace,
        cxx_proxy_namespace,
        cxx_proxy_include,
    ] = split_spec(spec)?;
    let path_components = rust_path
        .split("::")
        .map(|component| identifier(component).map(|()| component.to_owned()))
        .collect::<BoxResult<Vec<String>>>()?;
    identifier(rust_name)?;
    let has = |position: u32| {
        flags
            .checked_shr(position)
            .is_some_and(|value| value & 1 == 1)
    };
    Ok(CxxAutoArtifactInfo {
        path_components,
        path_descendants: Vec::new(),
        cxx_proxy_include: cxx_proxy_include.to_owned(),
        cxx_namespace: cxx_namespace.to_owned(),
        cxx_proxy_namespace: cxx_proxy_namespace.to_owned(),
        cxx_name: cxx_name.to_owned(),
        rust_name: rust_name.to_owned(),
        lifetimes: lifetimes(rust_lifetimes)?,
        align,
        size,
        cxx_has_operator_not_equal: has(bit::OPERATOR_NOT_EQUAL),
        is_rust_cxx_extern_type_trivial: has(bit::CXX_EXTERN_TYPE_TRIVIAL),
        is_rust_unpin: has(bit::UNPIN),
        is_rust_send: has(bit::SEND),
        is_rust_sync: has(bit::SYNC),
        is_rust_copy: has(bit::COPY),
        is_rust_debug: has(bit::DEBUG),
        is_rust_default: has(bit::DEFAULT_NEW),
        is_rust_display: has(bit::DISPLAY),
        is_rust_drop: has(bit::DROP),
        is_rust_copy_new: has(bit::COPY_NEW),
        is_rust_move_new: has(bit::MOVE_NEW),
        is_rust_eq: has(bit::EQ),
        is_rust_partial_eq: has(bit::PARTIAL_EQ),
        is_rust_partial_ord: has(bit::PARTIAL_ORD),
        is_rust_ord: has(bit::ORD),
        is_rust_hash: has(bit::HASH),
    })
}

/// Split the encoded spec into its seven strings.
///
/// # Specification
/// - provides: the strings in `TypeRecord` order.
/// - fails: the spec does not hold exactly seven NUL-terminated UTF-8 strings.
/// - panics: none.
fn split_spec(spec: &[u8]) -> BoxResult<[&str; SPEC_FIELDS]>
{
    let body = spec
        .strip_suffix(b"\0")
        .ok_or("the spec does not end in NUL")?;
    let fields = body
        .split(|&byte| byte == 0)
        .map(core::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()?;
    let count = fields.len();
    <[&str; SPEC_FIELDS]>::try_from(fields)
        .map_err(|_fields| format!("the spec holds {count} strings, expected {SPEC_FIELDS}").into())
}

/// Require a plain Rust identifier.
///
/// # Specification
/// - fails: `text` does not parse as one identifier.
/// - panics: none.
fn identifier(text: &str) -> BoxResult<()>
{
    syn::parse_str::<syn::Ident>(text)
        .map(drop)
        .map_err(|error| format!("`{text}` is not a Rust identifier: {error}").into())
}

/// Parse a lifetime parameter list such as `'a, 'b: 'a`.
///
/// # Specification
/// - provides: each lifetime name without its `'`, with the names it outlives,
///   in declaration order; empty for empty text.
/// - fails: the text is not a list of lifetime parameters.
/// - panics: none.
fn lifetimes(text: &str) -> BoxResult<Vec<(String, Vec<String>)>>
{
    let generics = syn::parse_str::<syn::Generics>(&format!("<{text}>"))
        .map_err(|error| format!("`{text}` is not a lifetime list: {error}"))?;
    generics
        .params
        .iter()
        .map(|param| {
            let syn::GenericParam::Lifetime(ref lifetime) = *param
            else {
                return Err(format!("`{text}` declares a non-lifetime parameter").into());
            };
            let bounds = lifetime
                .bounds
                .iter()
                .map(|bound| bound.ident.to_string())
                .collect();
            Ok((lifetime.lifetime.ident.to_string(), bounds))
        })
        .collect()
}

/// Read a little-endian `u32` at `at`.
///
/// # Specification
/// - fails: fewer than four bytes remain at `at`.
/// - panics: none.
fn read_u32(
    bytes: &[u8],
    at: usize,
) -> BoxResult<u32>
{
    let end = at.checked_add(4).ok_or("offset overflows")?;
    let field = bytes.get(at .. end).ok_or("the record is truncated")?;
    Ok(u32::from_le_bytes(field.try_into()?))
}

/// Read a little-endian `u64` at `at`.
///
/// # Specification
/// - fails: fewer than eight bytes remain at `at`.
/// - panics: none.
fn read_u64(
    bytes: &[u8],
    at: usize,
) -> BoxResult<u64>
{
    let end = at.checked_add(8).ok_or("offset overflows")?;
    let field = bytes.get(at .. end).ok_or("the record is truncated")?;
    Ok(u64::from_le_bytes(field.try_into()?))
}

#[cfg(test)]
mod tests
{
    use alloc::string::String;
    use alloc::string::ToString as _;
    use alloc::vec::Vec;

    use super::decode;

    /// The seven spec fields of a well-formed nested type with lifetimes.
    const FIELDS: [&str; 7] = [
        "geo::point",
        "Point",
        "'a, 'b, 'c: 'a + 'b",
        "Point",
        "geo",
        "geo::proxy",
        "geo/proxy.hxx",
    ];

    /// Encode a record the way `cxx_auto::type_record` lays it out.
    fn record(
        version: u32,
        flags: u64,
        fields: &[&str],
    ) -> Vec<u8>
    {
        let mut spec = Vec::new();
        for field in fields {
            spec.extend_from_slice(field.as_bytes());
            spec.push(0);
        }
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"cxx_auto");
        bytes.extend_from_slice(&version.to_le_bytes());
        bytes.extend_from_slice(&u32::try_from(spec.len()).unwrap_or(u32::MAX).to_le_bytes());
        bytes.extend_from_slice(&16_u64.to_le_bytes());
        bytes.extend_from_slice(&8_u64.to_le_bytes());
        bytes.extend_from_slice(&flags.to_le_bytes());
        bytes.extend_from_slice(&spec);
        bytes
    }

    #[test]
    fn decodes_names_layout_capabilities_and_lifetime_bounds() -> crate::BoxResult<()>
    {
        // Bits: trivial (0), send (2), eq (9), partial_eq (10), operator!= (16).
        let flags: u64 = 0b1_0000_0110_0000_0101;
        let mut bytes = record(1, flags, &FIELDS);
        bytes.extend_from_slice(b"trailing section bytes");
        let info = decode(&bytes)?;
        assert_eq!(
            info.path_components,
            ["geo", "point"],
            "rust_path splits on `::`"
        );
        assert_eq!(
            (
                info.size,
                info.align,
                info.rust_name.as_str(),
                info.cxx_proxy_namespace.as_str()
            ),
            (16, 8, "Point", "geo::proxy"),
            "layout and names survive decoding",
        );
        let expected: Vec<(String, Vec<String>)> = alloc::vec![
            ("a".into(), alloc::vec![]),
            ("b".into(), alloc::vec![]),
            ("c".into(), alloc::vec!["a".into(), "b".into()]),
        ];
        assert_eq!(info.lifetimes, expected, "every outlives bound is kept");
        assert!(
            info.is_rust_cxx_extern_type_trivial
                && info.is_rust_send
                && !info.is_rust_sync
                && info.is_rust_eq
                && info.is_rust_partial_eq
                && !info.is_rust_ord
                && info.cxx_has_operator_not_equal,
            "each flag bit maps to its own capability",
        );
        let (binder, _) = crate::cxx_auto_artifact_info::emit_generics(&info, false);
        let binder = quote::ToTokens::to_token_stream(&binder).to_string();
        assert_eq!(
            binder, "< 'a , 'b , 'c : 'a + 'b >",
            "every bound is emitted"
        );
        Ok(())
    }

    #[test]
    fn rejects_records_it_cannot_trust()
    {
        let short = FIELDS.get(.. 6).unwrap_or_default();
        let bad_path = ["geo::2d", "Point", "", "Point", "geo", "p", "p.hxx"];
        let bad_lifetime = ["geo", "Point", "T", "Point", "geo", "p", "p.hxx"];
        let mut bad_magic = record(1, 0, &FIELDS);
        if let Some(byte) = bad_magic.first_mut() {
            *byte = b'X';
        }
        let truncated = record(1, 0, &FIELDS)
            .get(.. 50)
            .unwrap_or_default()
            .to_vec();
        let cases: [(&str, Vec<u8>); 7] = [
            ("magic", bad_magic),
            ("version", record(2, 0, &FIELDS)),
            ("flag bit", record(1, 0b10_0000_0000_0000_0000, &FIELDS)),
            ("string count", record(1, 0, short)),
            ("path identifier", record(1, 0, &bad_path)),
            ("lifetime list", record(1, 0, &bad_lifetime)),
            ("length", truncated),
        ];
        for (case, bytes) in cases {
            assert!(
                decode(&bytes).is_err(),
                "a record with a bad {case} is rejected"
            );
        }
    }
}
