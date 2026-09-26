use serde::Deserialize;

/// One C++ type description and the namespaces, include files, and Rust
/// lifetimes needed for its binding.
#[cfg(feature = "alloc")]
#[derive(Deserialize)]
pub struct CxxAutoEntry<'ctx>
{
    /// Header declaring the C++ type.
    cxx_include: &'ctx str,
    /// Optional C++ proxy header when the ABI probe uses a different include.
    cxx_proxy_include: Option<&'ctx str>,
    /// Namespace containing the C++ type.
    cxx_namespace: &'ctx str,
    /// Optional namespace for the C++ proxy.
    cxx_proxy_namespace: Option<&'ctx str>,
    /// C++ type name when it differs from the Rust name.
    cxx_name: Option<&'ctx str>,
    /// Rust type name to generate.
    rust_name: &'ctx str,
    #[serde(default)]
    /// Rust lifetime names and their outlives bounds.
    rust_lifetimes: ::indexmap::IndexMap<&'ctx str, ::alloc::vec::Vec<&'ctx str>>,
}

#[cfg(feature = "alloc")]
impl CxxAutoEntry<'_>
{
    /// Return the C++ type name, falling back to its Rust-facing name.
    #[must_use]
    #[inline]
    pub fn cxx_name(&self) -> &str
    {
        self.cxx_name.unwrap_or(self.rust_name)
    }

    /// Generate the artifact-information reader and module writer for this
    /// type.
    ///
    /// # Specification
    /// - provides: writers whose configuration and ABI queries reflect this
    ///   type description.
    /// - panics: syn rejects invalid Rust identifiers in the configuration.
    pub(crate) fn emit_items_write_module_for_file<'component, 'descendant>(
        &self,
        path_components: impl Iterator<Item = &'component ::alloc::string::String>,
        path_descendants: impl Iterator<Item = &'descendant ::alloc::string::String>,
    ) -> ::alloc::vec::Vec<syn::ItemFn>
    {
        let cxx_include = self.cxx_include;
        let cxx_proxy_include = self.cxx_proxy_include.unwrap_or(self.cxx_include);
        let cxx_namespace = self.cxx_namespace;
        let cxx_proxy_namespace = self.cxx_proxy_namespace.unwrap_or(self.cxx_namespace);
        let cxx_name = self.cxx_name();
        let rust_name = self.rust_name;
        let lifetimes = {
            let mut exprs = ::alloc::vec::Vec::<syn::Expr>::new();
            for (lifetime, bounds) in &self.rust_lifetimes {
                exprs.push(syn::parse_quote!((#lifetime, vec![#(#bounds),*])));
            }
            exprs
        };
        ::alloc::vec![
            syn::parse_quote! {
                fn artifact_info() -> ::cxx_auto::CxxAutoArtifactInfo {
                    let path_components = vec![#(#path_components),*];
                    let path_descendants = vec![#(#path_descendants),*];
                    let cxx_include = #cxx_include;
                    let cxx_proxy_include = #cxx_proxy_include;
                    let cxx_namespace = #cxx_namespace;
                    let cxx_proxy_namespace = #cxx_proxy_namespace;
                    let cxx_name = #cxx_name;
                    let rust_name = #rust_name;
                    let lifetimes = ::cxx_auto::indexmap::IndexMap::from_iter([#(#lifetimes),*]);
                    let align = self::ffi::cxx_abi_align();
                    let size = self::ffi::cxx_abi_size();
                    let cxx_has_operator_equal = self::ffi::cxx_has_operator_equal();
                    let cxx_has_operator_not_equal = self::ffi::cxx_has_operator_not_equal();
                    let cxx_has_operator_less_than = self::ffi::cxx_has_operator_less_than();
                    let cxx_has_operator_less_than_or_equal = self::ffi::cxx_has_operator_less_than_or_equal();
                    let cxx_has_operator_greater_than = self::ffi::cxx_has_operator_greater_than();
                    let cxx_has_operator_greater_than_or_equal = self::ffi::cxx_has_operator_greater_than_or_equal();
                    let is_rust_cxx_extern_type_trivial = {
                        let cxx_is_trivially_movable = self::ffi::cxx_is_trivially_movable();
                        let rust_should_impl_cxx_extern_type_trivial = self::ffi::rust_should_impl_cxx_extern_type_trivial();
                        if cxx_is_trivially_movable == rust_should_impl_cxx_extern_type_trivial {
                            cxx_is_trivially_movable
                        } else {
                            rust_should_impl_cxx_extern_type_trivial
                        }
                    };
                    let is_rust_unpin = self::ffi::rust_should_impl_unpin();
                    let is_rust_send = self::ffi::rust_should_impl_send();
                    let is_rust_sync = self::ffi::rust_should_impl_sync();
                    let is_rust_copy = self::ffi::rust_should_impl_copy();
                    let is_rust_drop = self::ffi::rust_should_impl_drop();
                    let is_rust_debug = self::ffi::rust_should_impl_debug();
                    let is_rust_default = self::ffi::rust_should_impl_default();
                    let is_rust_display = self::ffi::rust_should_impl_display();
                    let is_rust_copy_new = self::ffi::rust_should_impl_moveref_copy_new();
                    let is_rust_move_new = self::ffi::rust_should_impl_moveref_move_new();
                    let is_rust_eq = self::ffi::rust_should_impl_eq();
                    let is_rust_partial_eq = self::ffi::rust_should_impl_partial_eq();
                    let is_rust_partial_ord = self::ffi::rust_should_impl_partial_ord();
                    let is_rust_ord = self::ffi::rust_should_impl_ord();
                    let is_rust_hash = self::ffi::rust_should_impl_hash();
                    ::cxx_auto::CxxAutoArtifactInfo {
                        path_components,
                        path_descendants,
                        cxx_include,
                        cxx_proxy_include,
                        cxx_namespace,
                        cxx_proxy_namespace,
                        cxx_name,
                        rust_name,
                        lifetimes,
                        align,
                        size,
                        cxx_has_operator_equal,
                        cxx_has_operator_not_equal,
                        cxx_has_operator_less_than,
                        cxx_has_operator_less_than_or_equal,
                        cxx_has_operator_greater_than,
                        cxx_has_operator_greater_than_or_equal,
                        is_rust_cxx_extern_type_trivial,
                        is_rust_unpin,
                        is_rust_send,
                        is_rust_sync,
                        is_rust_copy,
                        is_rust_debug,
                        is_rust_default,
                        is_rust_display,
                        is_rust_drop,
                        is_rust_copy_new,
                        is_rust_move_new,
                        is_rust_eq,
                        is_rust_partial_eq,
                        is_rust_partial_ord,
                        is_rust_ord,
                        is_rust_hash,
                    }
                }
            },
            syn::parse_quote! {
                pub(crate) fn write_module(auto_out_dir_root: &::std::path::Path) -> ::cxx_auto::BoxResult<()> {
                    self::artifact_info().write_module_for_file(auto_out_dir_root)
                }
            },
        ]
    }

    /// Generate the CXX bridge between a C++ ABI probe and this type's Rust
    /// module.
    ///
    /// # Specification
    /// - provides: a bridge to ABI properties and capability predicates for the
    ///   configured type.
    /// - panics: syn rejects invalid Rust identifiers in the configuration.
    pub(crate) fn emit_item_mod_cxx_bridge(&self) -> [syn::Item; 2]
    {
        let namespace: syn::Attribute = {
            let namespace = self.cxx_proxy_namespace.unwrap_or(self.cxx_namespace);
            syn::parse_quote!(#[namespace = #namespace])
        };
        let include = self.cxx_proxy_include.unwrap_or(self.cxx_include);
        [
            syn::parse_quote! {
                #[cxx::bridge]
                mod ffi {
                    #namespace
                    unsafe extern "C++" {
                        include!(#include);
                        #[must_use]
                        fn cxx_abi_align() -> usize;
                        #[must_use]
                        fn cxx_abi_size() -> usize;
                        #[must_use]
                        fn cxx_is_copy_constructible() -> bool;
                        #[must_use]
                        fn cxx_is_move_constructible() -> bool;
                        #[must_use]
                        fn cxx_is_default_constructible() -> bool;
                        #[must_use]
                        fn cxx_is_destructible() -> bool;
                        #[must_use]
                        fn cxx_is_trivially_copyable() -> bool;
                        #[must_use]
                        fn cxx_is_trivially_movable() -> bool;
                        #[must_use]
                        fn cxx_is_trivially_destructible() -> bool;
                        #[must_use]
                        fn cxx_is_equality_comparable() -> bool;
                        #[must_use]
                        fn cxx_has_operator_equal() -> bool;
                        #[must_use]
                        fn cxx_has_operator_not_equal() -> bool;
                        #[must_use]
                        fn cxx_has_operator_less_than() -> bool;
                        #[must_use]
                        fn cxx_has_operator_less_than_or_equal() -> bool;
                        #[must_use]
                        fn cxx_has_operator_greater_than() -> bool;
                        #[must_use]
                        fn cxx_has_operator_greater_than_or_equal() -> bool;
                        #[must_use]
                        fn cxx_is_partially_ordered() -> bool;
                        #[must_use]
                        fn cxx_is_totally_ordered() -> bool;
                        #[must_use]
                        fn cxx_is_hashable() -> bool;
                        #[must_use]
                        fn rust_should_impl_cxx_extern_type_trivial() -> bool;
                        #[must_use]
                        fn rust_should_impl_unpin() -> bool;
                        #[must_use]
                        fn rust_should_impl_send() -> bool;
                        #[must_use]
                        fn rust_should_impl_sync() -> bool;
                        #[must_use]
                        fn rust_should_impl_copy() -> bool;
                        #[must_use]
                        fn rust_should_impl_debug() -> bool;
                        #[must_use]
                        fn rust_should_impl_default() -> bool;
                        #[must_use]
                        fn rust_should_impl_display() -> bool;
                        #[must_use]
                        fn rust_should_impl_drop() -> bool;
                        #[must_use]
                        fn rust_should_impl_moveref_copy_new() -> bool;
                        #[must_use]
                        fn rust_should_impl_moveref_move_new() -> bool;
                        #[must_use]
                        fn rust_should_impl_eq() -> bool;
                        #[must_use]
                        fn rust_should_impl_partial_eq() -> bool;
                        #[must_use]
                        fn rust_should_impl_partial_ord() -> bool;
                        #[must_use]
                        fn rust_should_impl_ord() -> bool;
                        #[must_use]
                        fn rust_should_impl_hash() -> bool;
                    }
                }
            },
            syn::parse_quote! {
                pub use ffi::*;
            },
        ]
    }
}
