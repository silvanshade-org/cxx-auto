use ::indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TypeSpec<'ctx> {
    pub(crate) cxx_include: &'ctx str,
    pub(crate) cxx_namespace: &'ctx str,
    pub(crate) cxx_name: Option<&'ctx str>,
    pub(crate) rust_name: &'ctx str,
    #[serde(default)]
    pub(crate) rust_lifetimes: IndexMap<&'ctx str, Vec<&'ctx str>>,
}

impl<'ctx> TypeSpec<'ctx> {
    #[must_use]
    pub(crate) fn cxx_name(&self) -> &str {
        self.cxx_name.unwrap_or(self.rust_name)
    }
}
