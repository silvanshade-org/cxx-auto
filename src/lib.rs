#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::restriction)]
#![allow(clippy::absolute_paths)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::min_ident_chars)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_return)]
#![allow(clippy::pub_use)]
#![allow(clippy::pub_with_shorthand)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::ref_patterns)]
#![allow(clippy::self_named_module_files)]
#![allow(clippy::semicolon_outside_block)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::single_char_lifetime_names)]

extern crate alloc;

mod codegen;
pub mod emit;
mod error;
mod type_elab;
mod type_spec;
mod ffi {
    #[cfg(feature = "ctypes")]
    pub(crate) mod ctypes;
}
mod gen {
    #[cfg(feature = "ctypes")]
    pub(crate) mod ctypes;
}

use std::path::Path;

pub use crate::{
    error::{BoxError, BoxResult},
    type_elab::TypeElab,
    type_spec::TypeSpec,
};
pub use indexmap;
pub use moveref;
pub use static_assertions;

#[cfg(feature = "ctypes")]
pub mod ctypes {
    pub use crate::ffi::ctypes::{
        c_char,
        c_int,
        c_long,
        c_longlong,
        c_off_t,
        c_schar,
        c_short,
        c_time_t,
        c_uchar,
        c_uint,
        c_ulong,
        c_ulonglong,
        c_ushort,
        c_void,
    };
}

/// # Errors
///
/// Will return `Err` if auto-generation of the C++ bindings fails.
pub fn analyze(project_dir: &Path, out_dir: &Path, cfg_dir: &Path) -> crate::BoxResult<()> {
    crate::emit::auto_module(project_dir, out_dir, cfg_dir)?;
    Ok(())
}
