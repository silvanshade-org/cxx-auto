#[expect(
    clippy::unsafe_derive_deserialize,
    reason = "Every c_char bit pattern is valid in the transparent C ABI wrapper"
)]
#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_char
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_char,
}

impl ::core::fmt::Debug for c_char
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        let value = ::core::primitive::char::from(u8::from_le_bytes(self.value.to_le_bytes()));
        ::core::fmt::Debug::fmt(&value, f)
    }
}

impl ::core::fmt::Display for c_char
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        let value = ::core::primitive::char::from(u8::from_le_bytes(self.value.to_le_bytes()));
        ::core::fmt::Display::fmt(&value, f)
    }
}

// SAFETY: c_char is transparent over Rust's C char, which matches the C++ char
// alias in cxx-auto.hxx.
unsafe impl cxx::ExternType for c_char
{
    type Id = cxx::type_id!("c_char");
    type Kind = cxx::kind::Trivial;
}

impl From<::core::ffi::c_char> for c_char
{
    #[inline]
    fn from(value: ::core::ffi::c_char) -> Self
    {
        Self { value }
    }
}

impl From<c_char> for ::core::ffi::c_char
{
    #[inline]
    fn from(wrapper: c_char) -> Self
    {
        wrapper.value
    }
}

/// A Unicode scalar cannot be represented in the one-byte C character encoding.
///
/// # Specification
/// - provides: the rejected scalar so callers can distinguish the unsupported
///   input.
/// - panics: none.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CCharEncodingError
{
    /// The scalar does not fit in one unsigned byte.
    OutOfRange(char),
}

impl ::core::fmt::Display for CCharEncodingError
{
    /// Explain which scalar cannot be encoded.
    ///
    /// # Specification
    /// - provides: a diagnostic identifying the rejected scalar.
    /// - panics: none.
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        match *self {
            | Self::OutOfRange(value) => write!(f, "C character cannot encode {value}"),
        }
    }
}

impl ::core::error::Error for CCharEncodingError
{
}

impl ::core::convert::TryFrom<::core::primitive::char> for c_char
{
    type Error = CCharEncodingError;

    /// Encode a Latin-1 codepoint as one C character without truncation.
    ///
    /// # Specification
    /// - provides: the byte corresponding to any codepoint at most 0xFF.
    /// - fails: codepoints above 0xFF return
    ///   `CCharEncodingError::OutOfRange(value)`.
    /// - panics: none.
    ///
    /// # Adequacy
    /// - hypothesis: 0xFF round-trips and 0x100 must reject rather than wrap.
    /// - witness: tests/ctypes.rs
    ///   (`character_conversion_preserves_or_rejects`).
    #[inline]
    fn try_from(value: ::core::primitive::char) -> Result<Self, Self::Error>
    {
        let byte = u8::try_from(u32::from(value))
            .map_err(|_out_of_range| CCharEncodingError::OutOfRange(value))?;
        Ok(Self::from(::core::ffi::c_char::from_le_bytes([byte])))
    }
}

impl From<c_char> for ::core::primitive::char
{
    #[inline]
    fn from(wrapper: c_char) -> Self
    {
        Self::from(u8::from_le_bytes(wrapper.value.to_le_bytes()))
    }
}

impl crate::ffi::ctypes::c_char
{
    #[must_use]
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> &[Self]
    {
        let data = bytes.as_ptr().cast::<Self>();
        let len = bytes.len();
        // SAFETY: both slices are immutable; c_char is transparent over a one-byte
        // scalar with every bit pattern valid.
        unsafe { ::core::slice::from_raw_parts(data, len) }
    }

    #[must_use]
    #[inline]
    pub fn into_bytes(slice: &[Self]) -> &[u8]
    {
        let data = slice.as_ptr().cast::<u8>();
        let len = slice.len();
        // SAFETY: c_char is a transparent one-byte scalar, so its immutable slice has
        // the same extent and alignment.
        unsafe { ::core::slice::from_raw_parts(data, len) }
    }

    #[must_use]
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_path(path: &std::path::Path) -> &[Self]
    {
        use std::os::unix::ffi::OsStrExt as _;
        let bytes = path.as_os_str().as_bytes();
        Self::from_bytes(bytes)
    }

    #[must_use]
    #[expect(
        clippy::should_implement_trait,
        reason = "This returns a borrowed ABI view, not an owned value for FromStr"
    )]
    #[inline]
    pub fn from_str(str: &str) -> &[Self]
    {
        Self::from_bytes(str.as_bytes())
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_int
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_int,
}

impl ::core::fmt::Display for c_int
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_long
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_long,
}

impl ::core::fmt::Display for c_long
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_longlong
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_longlong,
}

impl ::core::fmt::Display for c_longlong
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_schar(::core::ffi::c_schar);

// SAFETY: c_schar is transparent over Rust's signed C char, matching the C++
// signed char alias.
unsafe impl cxx::ExternType for c_schar
{
    type Id = cxx::type_id!("c_schar");
    type Kind = cxx::kind::Trivial;
}

impl From<c_schar> for ::core::ffi::c_schar
{
    #[inline]
    fn from(value: c_schar) -> Self
    {
        value.0
    }
}

impl From<::core::ffi::c_schar> for c_schar
{
    #[inline]
    fn from(value: ::core::ffi::c_schar) -> Self
    {
        Self(value)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_short
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_short,
}

impl ::core::fmt::Display for c_short
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_uchar
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_uchar,
}

impl ::core::fmt::Display for c_uchar
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_uint
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_uint,
}

impl ::core::fmt::Display for c_uint
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_ulong
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_ulong,
}

impl ::core::fmt::Display for c_ulong
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_ulonglong
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_ulonglong,
}

impl ::core::fmt::Display for c_ulonglong
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy,
    Clone,
    Default,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_ushort
{
    /// The wrapped C ABI scalar.
    value: ::core::ffi::c_ushort,
}

impl ::core::fmt::Display for c_ushort
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_void(::core::ffi::c_void);

impl From<::core::ffi::c_void> for c_void
{
    #[inline]
    fn from(value: ::core::ffi::c_void) -> Self
    {
        Self(value)
    }
}

impl From<c_void> for ::core::ffi::c_void
{
    #[inline]
    fn from(value: c_void) -> Self
    {
        value.0
    }
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_off_t
{
    /// The wrapped C ABI scalar.
    value: libc::off_t,
}

impl ::core::fmt::Debug for c_off_t
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Debug::fmt(&self.value, f)
    }
}

impl ::core::fmt::Display for c_off_t
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

// SAFETY: c_off_t wraps libc::off_t transparently, matching the C++ off_t
// alias.
unsafe impl cxx::ExternType for c_off_t
{
    type Id = cxx::type_id!("c_off_t");
    type Kind = cxx::kind::Trivial;
}

#[expect(
    non_camel_case_types,
    reason = "C ABI type names must match their C++ spellings"
)]
#[derive(
    Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct c_time_t
{
    /// The wrapped C ABI scalar.
    value: libc::time_t,
}

impl ::core::fmt::Debug for c_time_t
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Debug::fmt(&self.value, f)
    }
}

impl ::core::fmt::Display for c_time_t
{
    #[inline]
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result
    {
        ::core::fmt::Display::fmt(&self.value, f)
    }
}

// SAFETY: c_time_t wraps libc::time_t transparently, matching the C++ time_t
// alias.
unsafe impl cxx::ExternType for c_time_t
{
    type Id = cxx::type_id!("c_time_t");
    type Kind = cxx::kind::Trivial;
}
