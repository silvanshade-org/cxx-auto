#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::allow_attributes_without_reason)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::as_conversions)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::create_dir)]
#![deny(clippy::decimal_literal_representation)]
#![deny(clippy::default_numeric_fallback)]
#![deny(clippy::default_union_representation)]
#![deny(clippy::error_impl_error)]
#![deny(clippy::exhaustive_enums)]
#![deny(clippy::exhaustive_structs)]
#![deny(clippy::filetype_is_file)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::infinite_loop)]
#![deny(clippy::iter_over_hash_type)]
#![deny(clippy::mod_module_files)]
#![deny(clippy::mutex_atomic)]
#![deny(clippy::pattern_type_mismatch)]
#![deny(clippy::shadow_unrelated)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![deny(clippy::wildcard_enum_match_arm)]

extern crate alloc;

use core::ffi::CStr;
use std::path::PathBuf;

use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

fn error_null_pointer() -> BoxError {
    BoxError::from("unexpected NULL pointer")
}

#[allow(clippy::exhaustive_structs)]
#[repr(C, align(32))]
pub struct RawTypeSpecLifetime {
    pub name: *const core::ffi::c_char,
    pub bounds_data: *const *const core::ffi::c_char,
    pub bounds_len: usize,
}

// TODO: profile
// NOTE: consider less aggressive validation depending profile results
impl RawTypeSpecLifetime {
    fn validate(&self) -> BoxResult<TypeSpecLifetime> {
        // NOTE: C++ will use `nullptr` for empty data, but Rust's slice pointers must always be
        // non-null: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety. So when the
        // data pointer is null we just return a pre-constructed empty slice directly, and this also
        // matches the behavior of `cxx`.
        const EMPTY_SLICE: &[*const core::ffi::c_char] = &[];

        let name_ref = unsafe { self.name.as_ref() }.ok_or_else(error_null_pointer)?;
        let name = unsafe { CStr::from_ptr(name_ref) }.to_str()?;
        let bounds_data_ref = unsafe { self.bounds_data.as_ref() };
        let bounds_slice = bounds_data_ref.map_or(EMPTY_SLICE, |bounds_data_opt| unsafe {
            core::slice::from_raw_parts(bounds_data_opt, self.bounds_len)
        });
        // Check if data is null that len was 0
        if bounds_slice.len() != self.bounds_len {
            return Err(format!("Bounds data pointer was NULL but len was {}!", bounds_slice.len()).into());
        }
        let bounds = bounds_slice
            .iter()
            .copied()
            .map(|ptr| {
                let bound_ref = unsafe { ptr.as_ref() }.ok_or_else(error_null_pointer)?;
                let bound = unsafe { CStr::from_ptr(bound_ref) }.to_str()?;
                Ok(bound)
            })
            .collect::<BoxResult<Vec<_>>>()?;
        Ok(TypeSpecLifetime { name, bounds })
    }
}

#[allow(clippy::exhaustive_structs)]
#[repr(C, align(64))]
pub struct RawTypeSpec {
    pub cc_name: *const core::ffi::c_char,
    pub cc_namespace: *const core::ffi::c_char,
    pub rs_name: *const core::ffi::c_char,
    pub rs_namespace: *const core::ffi::c_char,
    pub rs_lifetimes_data: *const RawTypeSpecLifetime,
    pub rs_lifetimes_len: usize,
}

// TODO: profile
// NOTE: consider less aggressive validation depending profile results
impl RawTypeSpec {
    unsafe fn validate(&self) -> BoxResult<TypeSpec> {
        let cc_name_ref = unsafe { self.cc_name.as_ref() }.ok_or_else(error_null_pointer)?;
        let cc_name = unsafe { CStr::from_ptr(cc_name_ref) }.to_str()?;
        let cc_namespace_ref = unsafe { self.cc_namespace.as_ref() }.ok_or_else(error_null_pointer)?;
        let cc_namespace = unsafe { CStr::from_ptr(cc_namespace_ref) }.to_str()?;
        let rs_name_ref = unsafe { self.rs_name.as_ref() }.ok_or_else(error_null_pointer)?;
        let rs_name = unsafe { CStr::from_ptr(rs_name_ref) }.to_str()?;
        let rs_namespace_ref = unsafe { self.rs_namespace.as_ref() }.ok_or_else(error_null_pointer)?;
        let rs_namespace = unsafe { CStr::from_ptr(rs_namespace_ref) }.to_str()?;
        let rs_lifetimes_ref = unsafe { self.rs_lifetimes_data.as_ref() }.ok_or_else(error_null_pointer)?;
        let rs_lifetimes = unsafe { core::slice::from_raw_parts(rs_lifetimes_ref, self.rs_lifetimes_len) }
            .iter()
            .map(RawTypeSpecLifetime::validate)
            .collect::<BoxResult<Vec<_>>>()?;
        Ok(TypeSpec {
            cc_name,
            cc_namespace,
            rs_name,
            rs_namespace,
            rs_lifetimes,
        })
    }
}

#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
pub struct TypeSpecLifetime {
    pub name: &'static str,
    pub bounds: Vec<&'static str>,
}

#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
pub struct TypeSpec {
    pub cc_name: &'static str,
    pub cc_namespace: &'static str,
    pub rs_name: &'static str,
    pub rs_namespace: &'static str,
    pub rs_lifetimes: Vec<TypeSpecLifetime>,
}

#[allow(clippy::exhaustive_structs)]
#[repr(C, align(32))]
#[derive(Debug)]
pub struct TypeElab {
    pub cxx_abi_align: usize,
    pub cxx_abi_size: usize,
    pub rust_should_impl_cxx_extern_type_trivial: bool,
    pub rust_should_impl_unpin: bool,
    pub rust_should_impl_send: bool,
    pub rust_should_impl_sync: bool,
    pub rust_should_impl_drop: bool,
    pub rust_should_impl_copy: bool,
    pub rust_should_impl_default: bool,
    pub rust_should_impl_moveref_copy_new: bool,
    pub rust_should_impl_moveref_move_new: bool,
    pub rust_should_impl_eq: bool,
    pub rust_should_impl_partial_eq: bool,
    pub rust_should_impl_partial_ord: bool,
    pub rust_should_impl_ord: bool,
    pub rust_should_impl_hash: bool,
    pub rust_should_impl_debug: bool,
    pub rust_should_impl_display: bool,
}

/// # Errors
///
/// # Panics
///
/// Will return `Err` if auto-generation of the C++ bindings fails.
pub fn generate(builder: &cc::Build, out_dir: &Utf8Path, objects: Vec<PathBuf>) -> crate::BoxResult<()> {
    let cxx_auto_out = Utf8Path::new(&std::env::var("DEP_CXX_AUTO_CXXBRIDGE_DIR0")?).join("../..");
    let compiler = builder.try_get_compiler()?;
    let mut objects = objects
        .into_iter()
        .map(Utf8PathBuf::from_path_buf)
        .filter_ok(|elem| !elem.as_str().ends_with(".rs.o"))
        .map(|res| res.map_err(|path| BoxError::from(format!("Path is not valid UTF-8: {}", path.display()))));
    while let Some(obj) = objects.next().transpose()? {
        let lib = {
            let name = obj
                .file_stem()
                .ok_or_else(|| BoxError::from(format!("Cannot compute file_stem: {obj}")))?;
            out_dir.join(format!("lib{name}.so"))
        };
        compiler
            .to_command()
            .args(["-fvisibility=hidden", "-shared", "-o", lib.as_str()])
            .args([obj])
            .args(["-L", cxx_auto_out.as_str(), "-Wl,--exclude-libs,ALL", "-l", "cxx-auto"])
            .status()?;
        let lib = unsafe { libloading::Library::new(lib)? };
        let type_spec = unsafe { lib.get::<*const RawTypeSpec>(b"type_spec")?.as_ref() }
            .ok_or_else(error_null_pointer)
            .and_then(|r| unsafe { r.validate() })?;
        let type_elab = unsafe { lib.get::<*const TypeElab>(b"type_elab")?.as_ref() }.ok_or_else(error_null_pointer)?;
        lib.close()?;
    }
    Ok(())
}
