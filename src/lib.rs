//! Generate Rust/C++ bindings from C++ type descriptions and expose C ABI
//! wrappers.
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Emit Rust structs and their CXX bridge implementations.
mod cxx_auto_artifact_info;
/// Parse C++ type descriptions into generated Rust items.
mod cxx_auto_entry;
/// Name the fallible artifact-generation boundary.
mod error;
/// Hold the C ABI wrapper definitions used by the bridge.
mod ffi
{
    /// C primitive wrapper representations.
    pub mod ctypes;
}
/// Compile CXX vector and unique-pointer support for C character wrappers.
mod r#gen
{
    /// CXX glue for the C character wrapper.
    #[expect(
        clippy::missing_inline_in_public_items,
        clippy::panic,
        clippy::renamed_function_params,
        reason = "CXX generates implementations for these external container types"
    )]
    pub mod ctypes;
}
/// Walk type descriptions and write generated modules.
mod processing;

#[cfg(feature = "alloc")]
pub use indexmap;

#[cfg(feature = "alloc")]
pub use crate::cxx_auto_artifact_info::CxxAutoArtifactInfo;
#[cfg(feature = "alloc")]
pub use crate::cxx_auto_entry::CxxAutoEntry;
#[cfg(feature = "alloc")]
pub use crate::error::*;

/// Nominal C ABI primitive wrappers for use in CXX bindings.
pub mod ctypes
{
    pub use crate::ffi::ctypes::CCharEncodingError;
    pub use crate::ffi::ctypes::c_char;
    pub use crate::ffi::ctypes::c_int;
    pub use crate::ffi::ctypes::c_long;
    pub use crate::ffi::ctypes::c_longlong;
    pub use crate::ffi::ctypes::c_off_t;
    pub use crate::ffi::ctypes::c_schar;
    pub use crate::ffi::ctypes::c_short;
    pub use crate::ffi::ctypes::c_time_t;
    pub use crate::ffi::ctypes::c_uchar;
    pub use crate::ffi::ctypes::c_uint;
    pub use crate::ffi::ctypes::c_ulong;
    pub use crate::ffi::ctypes::c_ulonglong;
    pub use crate::ffi::ctypes::c_ushort;
    pub use crate::ffi::ctypes::c_void;
}

/// Generate the Rust modules for C++ type descriptions in `cfg_dir`.
///
/// # Specification
/// - requires: `cfg_dir` contains type descriptions rooted at an auto
///   directory.
/// - ensures: `out_dir/src/auto.rs` and each described Rust bridge module are
///   written.
/// - fails: returns an I/O, input, or formatting error when generation cannot
///   complete.
/// - panics: malformed module identifiers may be rejected by syn.
///
/// # Adequacy
/// - hypothesis: generated modules resolve and compare C++ values even when
///   `cfg_dir` has a separate parent directory.
/// - witness: `tests/dynamic_binding.rs`
///   (`loads_generated_comparison_binding`).
///
/// # Errors
///
/// Will return `Err` if auto-generation of the C++ bindings fails.
#[cfg(feature = "std")]
#[inline]
pub fn process_artifacts(
    out_dir: &std::path::Path,
    cfg_dir: &std::path::Path,
) -> BoxResult<()>
{
    let out_dir = &out_dir.join("src");
    crate::processing::process_src_auto_module(out_dir, cfg_dir)?;
    Ok(())
}
