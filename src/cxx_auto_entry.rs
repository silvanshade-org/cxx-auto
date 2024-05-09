use serde::Deserialize;

#[derive(Deserialize)]
pub struct CxxAutoEntry<'ctx> {
    cxx_include: &'ctx str,
    cxx_proxy_include: Option<&'ctx str>,
    cxx_namespace: &'ctx str,
    cxx_proxy_namespace: Option<&'ctx str>,
    cxx_name: Option<&'ctx str>,
    rust_name: &'ctx str,
    #[serde(default)]
    rust_lifetimes: ::indexmap::IndexMap<&'ctx str, Vec<&'ctx str>>,
}

impl<'ctx> CxxAutoEntry<'ctx> {
    #[must_use]
    pub fn cxx_name(&self) -> &str {
        self.cxx_name.unwrap_or(self.rust_name)
    }

    pub(crate) fn emit_items_write_module_for_file<'a, 'b>(
        &self,
        path_components: impl Iterator<Item = &'a String>,
        path_descendants: impl Iterator<Item = &'b String>,
    ) -> Vec<syn::ItemFn> {
        let cxx_include = self.cxx_include;
        let cxx_namespace = self.cxx_namespace;
        let cxx_name = self.cxx_name();
        let rust_name = self.rust_name;
        let lifetimes = {
            let mut exprs = Vec::<syn::Expr>::new();
            for (lifetime, bounds) in &self.rust_lifetimes {
                exprs.push(syn::parse_quote!((#lifetime, vec![#(#bounds),*])));
            }
            exprs
        };
        vec![
            syn::parse_quote! {
                fn artifact_info() -> ::cxx_auto::CxxAutoArtifactInfo {
                    let path_components = vec![#(#path_components),*];
                    let path_descendants = vec![#(#path_descendants),*];
                    let cxx_include = #cxx_include;
                    let cxx_namespace = #cxx_namespace;
                    let cxx_name = #cxx_name;
                    let rust_name = #rust_name;
                    let lifetimes = ::cxx_auto::indexmap::IndexMap::from_iter([#(#lifetimes),*]);
                    let align = unsafe { self::ffi::CXX_ABI_ALIGN };
                    let size = unsafe { self::ffi::CXX_ABI_SIZE };
                    let cxx_has_operator_equal = unsafe { self::ffi::CXX_HAS_OPERATOR_EQUAL };
                    let cxx_has_operator_not_equal = unsafe { self::ffi::CXX_HAS_OPERATOR_NOT_EQUAL };
                    let cxx_has_operator_less_than = unsafe { self::ffi::CXX_HAS_OPERATOR_LESS_THAN };
                    let cxx_has_operator_less_than_or_equal = unsafe { self::ffi::CXX_HAS_OPERATOR_LESS_THAN_OR_EQUAL };
                    let cxx_has_operator_greater_than = unsafe { self::ffi::CXX_HAS_OPERATOR_GREATER_THAN };
                    let cxx_has_operator_greater_than_or_equal = unsafe { self::ffi::CXX_HAS_OPERATOR_GREATER_THAN_OR_EQUAL };
                    let is_rust_cxx_extern_type_trivial = {
                        let cxx_is_trivially_movable = unsafe { self::ffi::CXX_IS_TRIVIALLY_MOVABLE };
                        let rust_should_impl_cxx_extern_type_trivial = unsafe { self::ffi::RUST_SHOULD_IMPL_CXX_EXTERN_TYPE_TRIVIAL };
                        if cxx_is_trivially_movable == rust_should_impl_cxx_extern_type_trivial {
                            cxx_is_trivially_movable
                        } else {
                            rust_should_impl_cxx_extern_type_trivial
                        }
                    };
                    let is_rust_unpin = unsafe { self::ffi::RUST_SHOULD_IMPL_UNPIN };
                    let is_rust_send = unsafe { self::ffi::RUST_SHOULD_IMPL_SEND };
                    let is_rust_sync = unsafe { self::ffi::RUST_SHOULD_IMPL_SYNC };
                    let is_rust_copy = unsafe { self::ffi::RUST_SHOULD_IMPL_COPY };
                    let is_rust_drop = unsafe { self::ffi::RUST_SHOULD_IMPL_DROP };
                    let is_rust_debug = unsafe { self::ffi::RUST_SHOULD_IMPL_DEBUG };
                    let is_rust_default = unsafe { self::ffi::RUST_SHOULD_IMPL_DEFAULT };
                    let is_rust_display = unsafe { self::ffi::RUST_SHOULD_IMPL_DISPLAY };
                    let is_rust_copy_new = unsafe { self::ffi::RUST_SHOULD_IMPL_MOVEREF_COPY_NEW };
                    let is_rust_move_new = unsafe { self::ffi::RUST_SHOULD_IMPL_MOVEREF_MOVE_NEW };
                    let is_rust_eq = unsafe { self::ffi::RUST_SHOULD_IMPL_EQ };
                    let is_rust_partial_eq = unsafe { self::ffi::RUST_SHOULD_IMPL_PARTIAL_EQ };
                    let is_rust_partial_ord = unsafe { self::ffi::RUST_SHOULD_IMPL_PARTIAL_ORD };
                    let is_rust_ord = unsafe { self::ffi::RUST_SHOULD_IMPL_ORD };
                    let is_rust_hash = unsafe { self::ffi::RUST_SHOULD_IMPL_HASH };
                    ::cxx_auto::CxxAutoArtifactInfo {
                        path_components,
                        path_descendants,
                        cxx_include,
                        cxx_namespace,
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

    pub(crate) fn emit_item_mod_cxx_bridge(&self) -> syn::Item {
        let namespace_with_dollars = self.cxx_namespace.replace("::", "$");
        let link_name = |name: &str| [namespace_with_dollars.as_str(), "$", name].concat();
        let cxx_abi_align = link_name("CXX_ABI_ALIGN");
        let cxx_abi_size = link_name("CXX_ABI_SIZE");
        let cxx_is_copy_constructible = link_name("CXX_IS_COPY_CONSTRUCTIBLE");
        let cxx_is_move_constructible = link_name("CXX_IS_MOVE_CONSTRUCTIBLE");
        let cxx_is_default_constructible = link_name("CXX_IS_DEFAULT_CONSTRUCTIBLE");
        let cxx_is_destructible = link_name("CXX_IS_DESTRUCTIBLE");
        let cxx_is_trivially_copyable = link_name("CXX_IS_TRIVIALLY_COPYABLE");
        let cxx_is_trivially_movable = link_name("CXX_IS_TRIVIALLY_MOVABLE");
        let cxx_is_trivially_destructible = link_name("CXX_IS_TRIVIALLY_DESTRUCTIBLE");
        let cxx_is_equality_comparable = link_name("CXX_IS_EQUALITY_COMPARABLE");
        let cxx_has_operator_equal = link_name("CXX_HAS_OPERATOR_EQUAL");
        let cxx_has_operator_not_equal = link_name("CXX_HAS_OPERATOR_NOT_EQUAL");
        let cxx_has_operator_less_than = link_name("CXX_HAS_OPERATOR_LESS_THAN");
        let cxx_has_operator_less_than_or_equal = link_name("CXX_HAS_OPERATOR_LESS_THAN_OR_EQUAL");
        let cxx_has_operator_greater_than = link_name("CXX_HAS_OPERATOR_GREATER_THAN");
        let cxx_has_operator_greater_than_or_equal = link_name("CXX_HAS_OPERATOR_GREATER_THAN_OR_EQUAL");
        let cxx_is_partially_ordered = link_name("CXX_IS_PARTIALLY_ORDERED");
        let cxx_is_totally_ordered = link_name("CXX_IS_TOTALLY_ORDERED");
        let cxx_is_hashable = link_name("CXX_IS_HASHABLE");
        let rust_should_impl_cxx_extern_type_trivial = link_name("RUST_SHOULD_IMPL_CXX_EXTERN_TYPE_TRIVIAL");
        let rust_should_impl_unpin = link_name("RUST_SHOULD_IMPL_UNPIN");
        let rust_should_impl_send = link_name("RUST_SHOULD_IMPL_SEND");
        let rust_should_impl_sync = link_name("RUST_SHOULD_IMPL_SYNC");
        let rust_should_impl_copy = link_name("RUST_SHOULD_IMPL_COPY");
        let rust_should_impl_debug = link_name("RUST_SHOULD_IMPL_DEBUG");
        let rust_should_impl_default = link_name("RUST_SHOULD_IMPL_DEFAULT");
        let rust_should_impl_display = link_name("RUST_SHOULD_IMPL_DISPLAY");
        let rust_should_impl_drop = link_name("RUST_SHOULD_IMPL_DROP");
        let rust_should_impl_moveref_copy_new = link_name("RUST_SHOULD_IMPL_MOVEREF_COPY_NEW");
        let rust_should_impl_moveref_move_new = link_name("RUST_SHOULD_IMPL_MOVEREF_MOVE_NEW");
        let rust_should_impl_eq = link_name("RUST_SHOULD_IMPL_EQ");
        let rust_should_impl_partial_eq = link_name("RUST_SHOULD_IMPL_PARTIAL_EQ");
        let rust_should_impl_partial_ord = link_name("RUST_SHOULD_IMPL_PARTIAL_ORD");
        let rust_should_impl_ord = link_name("RUST_SHOULD_IMPL_ORD");
        let rust_should_impl_hash = link_name("RUST_SHOULD_IMPL_HASH");
        syn::parse_quote! {
            pub mod ffi {
                extern "C" {
                    #[link_name = #cxx_abi_align]
                    pub static CXX_ABI_ALIGN: usize;
                    #[link_name = #cxx_abi_size]
                    pub static CXX_ABI_SIZE: usize;
                    #[link_name = #cxx_is_copy_constructible]
                    pub static CXX_IS_COPY_CONSTRUCTIBLE: bool;
                    #[link_name = #cxx_is_move_constructible]
                    pub static CXX_IS_MOVE_CONSTRUCTIBLE: bool;
                    #[link_name = #cxx_is_default_constructible]
                    pub static CXX_IS_DEFAULT_CONSTRUCTIBLE: bool;
                    #[link_name = #cxx_is_destructible]
                    pub static CXX_IS_DESTRUCTIBLE: bool;
                    #[link_name = #cxx_is_trivially_copyable]
                    pub static CXX_IS_TRIVIALLY_COPYABLE: bool;
                    #[link_name = #cxx_is_trivially_movable]
                    pub static CXX_IS_TRIVIALLY_MOVABLE: bool;
                    #[link_name = #cxx_is_trivially_destructible]
                    pub static CXX_IS_TRIVIALLY_DESTRUCTIBLE: bool;
                    #[link_name = #cxx_is_equality_comparable]
                    pub static CXX_IS_EQUALITY_COMPARABLE: bool;
                    #[link_name = #cxx_has_operator_equal]
                    pub static CXX_HAS_OPERATOR_EQUAL: bool;
                    #[link_name = #cxx_has_operator_not_equal]
                    pub static CXX_HAS_OPERATOR_NOT_EQUAL: bool;
                    #[link_name = #cxx_has_operator_less_than]
                    pub static CXX_HAS_OPERATOR_LESS_THAN: bool;
                    #[link_name = #cxx_has_operator_less_than_or_equal]
                    pub static CXX_HAS_OPERATOR_LESS_THAN_OR_EQUAL: bool;
                    #[link_name = #cxx_has_operator_greater_than]
                    pub static CXX_HAS_OPERATOR_GREATER_THAN: bool;
                    #[link_name = #cxx_has_operator_greater_than_or_equal]
                    pub static CXX_HAS_OPERATOR_GREATER_THAN_OR_EQUAL: bool;
                    #[link_name = #cxx_is_partially_ordered]
                    pub static CXX_IS_PARTIALLY_ORDERED: bool;
                    #[link_name = #cxx_is_totally_ordered]
                    pub static CXX_IS_TOTALLY_ORDERED: bool;
                    #[link_name = #cxx_is_hashable]
                    pub static CXX_IS_HASHABLE: bool;
                    #[link_name = #rust_should_impl_cxx_extern_type_trivial]
                    pub static RUST_SHOULD_IMPL_CXX_EXTERN_TYPE_TRIVIAL: bool;
                    #[link_name = #rust_should_impl_unpin]
                    pub static RUST_SHOULD_IMPL_UNPIN: bool;
                    #[link_name = #rust_should_impl_send]
                    pub static RUST_SHOULD_IMPL_SEND: bool;
                    #[link_name = #rust_should_impl_sync]
                    pub static RUST_SHOULD_IMPL_SYNC: bool;
                    #[link_name = #rust_should_impl_copy]
                    pub static RUST_SHOULD_IMPL_COPY: bool;
                    #[link_name = #rust_should_impl_debug]
                    pub static RUST_SHOULD_IMPL_DEBUG: bool;
                    #[link_name = #rust_should_impl_default]
                    pub static RUST_SHOULD_IMPL_DEFAULT: bool;
                    #[link_name = #rust_should_impl_display]
                    pub static RUST_SHOULD_IMPL_DISPLAY: bool;
                    #[link_name = #rust_should_impl_drop]
                    pub static RUST_SHOULD_IMPL_DROP: bool;
                    #[link_name = #rust_should_impl_moveref_copy_new]
                    pub static RUST_SHOULD_IMPL_MOVEREF_COPY_NEW: bool;
                    #[link_name = #rust_should_impl_moveref_move_new]
                    pub static RUST_SHOULD_IMPL_MOVEREF_MOVE_NEW: bool;
                    #[link_name = #rust_should_impl_eq]
                    pub static RUST_SHOULD_IMPL_EQ: bool;
                    #[link_name = #rust_should_impl_partial_eq]
                    pub static RUST_SHOULD_IMPL_PARTIAL_EQ: bool;
                    #[link_name = #rust_should_impl_partial_ord]
                    pub static RUST_SHOULD_IMPL_PARTIAL_ORD: bool;
                    #[link_name = #rust_should_impl_ord]
                    pub static RUST_SHOULD_IMPL_ORD: bool;
                    #[link_name = #rust_should_impl_hash]
                    pub static RUST_SHOULD_IMPL_HASH: bool;
                }
            }
        }
    }
}
