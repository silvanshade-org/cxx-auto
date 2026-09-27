//! Generate Rust bindings for C++ types from type records compiled into object
//! files, and expose C ABI wrappers.
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// Emit Rust structs and their CXX bridge implementations.
mod cxx_auto_artifact_info;
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
/// Generate the binding modules for a set of compiled objects.
#[cfg(feature = "std")]
mod generate;
/// Compile C++ module interface units in an explicit order.
#[cfg(feature = "std")]
mod modules;
/// Decode the type records compiled into object files.
#[cfg(feature = "std")]
mod record;

#[cfg(feature = "alloc")]
pub use crate::error::*;
#[cfg(feature = "std")]
pub use crate::generate::generate;
#[cfg(feature = "std")]
pub use crate::modules::ModuleUnit;
#[cfg(feature = "std")]
pub use crate::modules::Modules;
#[cfg(feature = "std")]
pub use crate::modules::compile_modules;

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
