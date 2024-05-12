use std::path::Path;

use proc_macro2::Span;
use syn::punctuated::Punctuated;

impl crate::TypeElab {
    fn codegen_generics(&self, all_static: bool) -> (syn::Generics, syn::Generics) {
        #![allow(clippy::similar_names)]
        let span = Span::call_site();
        let mut binder_params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
        let mut params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
        for (name, bounds) in &self.rust_lifetimes {
            let name_or_static = if all_static { "static" } else { name };

            let lifetime_param = {
                let lifetime = syn::Lifetime::new(&format!("'{name_or_static}"), span);
                syn::LifetimeParam::new(lifetime)
            };

            let mut lifetime_param_binder = lifetime_param.clone();
            for bound in bounds {
                let lifetime = syn::Lifetime::new(&format!("'{bound}"), span);
                lifetime_param_binder.bounds.push_value(lifetime);
            }

            binder_params.push(syn::GenericParam::from(lifetime_param_binder));
            params.push(syn::GenericParam::from(lifetime_param));
        }
        let lt_token = (!params.is_empty()).then(|| syn::Token![<](span));
        let gt_token = (!params.is_empty()).then(|| syn::Token![>](span));
        let where_clause = None;
        (
            syn::Generics {
                lt_token,
                params: binder_params,
                gt_token,
                where_clause: where_clause.clone(),
            },
            syn::Generics {
                lt_token,
                params,
                gt_token,
                where_clause,
            },
        )
    }

    fn codegen_item_impl_cxx_extern_type(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> syn::ItemImpl {
        let type_id = format!("{}::{}", self.cxx_namespace, self.cxx_name);
        let kind: syn::Type = if self.is_rust_cxx_extern_type_trivial {
            syn::parse_quote!(::cxx::kind::Trivial)
        } else {
            syn::parse_quote!(::cxx::kind::Opaque)
        };
        syn::parse_quote! {
            unsafe impl #generics_binder ::cxx::ExternType for #ident #generics {
                type Id = ::cxx::type_id!(#type_id);
                type Kind = #kind;
            }
        }
    }

    fn codegen_item_impl_drop(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_drop.then(|| {
            syn::parse_quote! {
                impl #generics_binder ::core::ops::Drop for #ident #generics {
                    #[inline]
                    fn drop(&mut self) {
                        unsafe {
                            self::ffi::cxx_destruct(self);
                        }
                    }
                }
            }
        })
    }

    fn codegen_item_impl_debug(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> syn::ItemImpl {
        if self.is_rust_debug {
            syn::parse_quote! {
                impl #generics_binder ::core::fmt::Debug for #ident #generics {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        let string = self::ffi::cxx_debug(self);
                        write!(f, "{string}")
                    }
                }
            }
        } else {
            let name = self.rust_name;
            syn::parse_quote! {
                impl #generics_binder ::core::fmt::Debug for #ident #generics {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct(#name).finish_non_exhaustive()
                    }
                }
            }
        }
    }

    fn codegen_item_impl_default(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_default.then(|| {
            syn::parse_quote! {
                impl #generics_binder #ident #generics {
                    #[inline]
                    pub fn default_new() -> impl ::cxx_auto::moveref::New<Output = #ident #generics> {
                        unsafe {
                            ::cxx_auto::moveref::new::by_raw(move |this| {
                                let this = this.get_unchecked_mut().as_mut_ptr();
                                self::ffi::cxx_default_new(this);
                            })
                        }
                    }
                }
            }
        })
    }

    fn codegen_item_impl_display(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_display.then(|| {
            syn::parse_quote! {
                impl #generics_binder ::core::fmt::Display for #ident #generics {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        let string = self::ffi::cxx_display(self);
                        write!(f, "{string}")
                    }
                }
            }
        })
    }

    fn codegen_item_impl_moveit_copy_new(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_copy_new.then(|| {
            syn::parse_quote! {
                impl #generics_binder ::cxx_auto::moveref::CopyNew for #ident #generics {
                    #[inline]
                    unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
                        let this = this.get_unchecked_mut().as_mut_ptr();
                        self::ffi::cxx_copy_new(this, that);
                    }
                }
            }
        })
    }

    fn codegen_item_impl_moveit_move_new(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_move_new.then(|| {
            syn::parse_quote! {
                impl #generics_binder ::cxx_auto::moveref::MoveNew for #ident #generics {
                    #[inline]
                    unsafe fn move_new(
                        that: ::core::pin::Pin<::cxx_auto::moveref::MoveRef<'_, Self>>,
                        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
                    ) {
                        let this = this.get_unchecked_mut().as_mut_ptr();
                        let that = &mut *::core::pin::Pin::into_inner_unchecked(that);
                        self::ffi::cxx_move_new(this, that);
                    }
                }
            }
        })
    }

    fn codegen_item_impl_partial_eq(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_partial_eq.then(|| {
            let ne: Option<syn::ImplItemFn> = self.cxx_has_operator_not_equal.then(|| {
                syn::parse_quote! {
                    #[allow(clippy::partialeq_ne_impl)]
                    #[inline]
                    fn ne(&self, other: &Self) -> bool {
                        self::ffi::cxx_operator_not_equal(self, other)
                    }
                }
            });
            syn::parse_quote! {
                impl #generics_binder ::core::cmp::PartialEq for #ident #generics {
                    #[inline]
                    fn eq(&self, other: &Self) -> bool {
                        self::ffi::cxx_operator_equal(self, other)
                    }
                    #ne
                }
            }
        })
    }

    fn codegen_item_impl_eq(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_eq.then(|| {
            syn::parse_quote! {
                impl #generics_binder ::core::cmp::Eq for #ident #generics {}
            }
        })
    }

    fn codegen_item_impl_partial_ord(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_partial_ord.then(|| {
            let lt: Option<syn::ImplItemFn> = self.cxx_has_operator_less_than.then(|| {
                syn::parse_quote! {
                    #[inline]
                    fn lt(&self, other: &Self) -> bool {
                        self::ffi::cxx_operator_less_than(self, other)
                    }
                }
            });
            let le: Option<syn::ImplItemFn> = self.cxx_has_operator_less_than_or_equal.then(|| {
                syn::parse_quote! {
                    #[inline]
                    fn le(&self, other: &Self) -> bool {
                        self::ffi::cxx_operator_less_than_or_equal(self, other)
                    }
                }
            });
            let gt: Option<syn::ImplItemFn> = self.cxx_has_operator_greater_than.then(|| {
                syn::parse_quote! {
                    #[inline]
                    fn gt(&self, other: &Self) -> bool {
                        self::ffi::cxx_operator_greater_than(self, other)
                    }
                }
            });
            let ge: Option<syn::ImplItemFn> = self.cxx_has_operator_greater_than_or_equal.then(|| {
                syn::parse_quote! {
                    #[inline]
                    fn ge(&self, other: &Self) -> bool {
                        self::ffi::cxx_operator_greater_than_or_equal(self, other)
                    }
                }
            });
            let partial_cmp: syn::ImplItemFn = if self.is_rust_ord {
                syn::parse_quote! {
                    #[inline]
                    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                        Some(self.cmp(other))
                    }
                }
            } else {
                syn::parse_quote! {
                    #[inline]
                    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                        let res = self::ffi::cxx_operator_three_way_comparison(self, other);
                        if res == -1 {
                            Some(::core::cmp::Ordering::Less)
                        } else if res == 1 {
                            Some(::core::cmp::Ordering::Greater)
                        } else if res == 0 {
                            Some(::core::cmp::Ordering::Equal)
                        } else {
                            ::core::assert_eq!(res, ::core::primitive::i8::MAX);
                            None
                        }
                    }
                }
            };
            syn::parse_quote! {
                impl #generics_binder ::core::cmp::PartialOrd for #ident #generics {
                    #partial_cmp
                    #lt
                    #le
                    #gt
                    #ge
                }
            }
        })
    }

    fn codegen_item_impl_ord(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_ord.then(|| {
            syn::parse_quote! {
                impl #generics_binder ::core::cmp::Ord for #ident #generics {
                    #[inline]
                    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                        let res = self::ffi::cxx_operator_three_way_comparison(self, other);
                        res.cmp(&0)
                    }
                }
            }
        })
    }

    fn codegen_item_impl_hash(
        &self,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> Option<syn::ItemImpl> {
        self.is_rust_hash.then(|| {
            syn::parse_quote! {
                impl #generics_binder ::core::hash::Hash for #ident #generics {
                    #[inline]
                    fn hash<H>(&self, state: &mut H)
                    where
                        H: ::core::hash::Hasher,
                    {
                        let hash = self::ffi::cxx_hash(self);
                        state.write_usize(hash);
                    }
                }
            }
        })
    }

    fn codegen_item_struct(
        &self,
        cxx_abi_align: &proc_macro2::Literal,
        cxx_abi_size: &proc_macro2::Literal,
        ident: &syn::Ident,
        generics_binder: &syn::Generics,
        generics: &syn::Generics,
    ) -> syn::ItemStruct {
        let derive_clone_copy_attr = self
            .is_rust_copy
            .then(|| -> syn::Attribute { syn::parse_quote!(#[derive(Clone, Copy)]) });
        let field_memory_layout = crate::codegen::field_memory_layout(cxx_abi_size);
        let field_neither_send_nor_sync = crate::codegen::field_neither_send_nor_sync(self);
        let field_phantom_pinned = crate::codegen::field_phantom_pinned(self);
        let field_phantom_data_lifetimes = crate::codegen::field_phantom_data_lifetimes(generics);
        let fields = syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: vec![
                Some(field_memory_layout),
                field_neither_send_nor_sync,
                field_phantom_pinned,
                field_phantom_data_lifetimes,
            ]
            .into_iter()
            .flatten()
            .collect::<Punctuated<syn::Field, syn::Token![,]>>(),
        };
        syn::parse_quote! {
            #derive_clone_copy_attr
            #[repr(C, align(#cxx_abi_align))]
            pub struct #ident #generics_binder #fields
        }
    }

    #[allow(clippy::too_many_lines)]
    fn codegen_item_mod_auto_type_ffi(&self, ident: &syn::Ident, generics: &syn::Generics) -> syn::ItemMod {
        let cxx_include = &self.cxx_include;
        let cxx_namespace = &self.cxx_namespace;
        let cxx_name = &self.cxx_name;
        let cxx_copy_new: Option<syn::ForeignItemFn> = self.is_rust_copy_new.then(|| {
            syn::parse_quote! {
                unsafe fn cxx_copy_new #generics (This: *mut #ident #generics, that: &#ident #generics);
            }
        });
        let cxx_move_new: Option<syn::ForeignItemFn> = self.is_rust_move_new.then(|| {
            syn::parse_quote! {
                unsafe fn cxx_move_new #generics (This: *mut #ident #generics, that: *mut #ident #generics);
            }
        });
        let cxx_default_new: Option<syn::ForeignItemFn> = self.is_rust_default.then(|| {
            syn::parse_quote! {
                unsafe fn cxx_default_new #generics (This: *mut #ident #generics);
            }
        });
        let cxx_destruct: Option<syn::ForeignItemFn> = self.is_rust_drop.then(|| {
            syn::parse_quote! {
                unsafe fn cxx_destruct #generics (This: *mut #ident #generics);
            }
        });
        let cxx_operator_equal: Option<syn::ForeignItemFn> = self.is_rust_eq.then(|| {
            syn::parse_quote! {
                fn cxx_operator_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
            }
        });
        let cxx_operator_not_equal: Option<syn::ForeignItemFn> = self.is_rust_eq.then(|| {
            syn::parse_quote! {
                fn cxx_operator_not_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
            }
        });
        let cxx_operator_less_than: Option<syn::ForeignItemFn> = self.cxx_has_operator_less_than.then(|| {
            syn::parse_quote! {
                fn cxx_operator_less_than #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
            }
        });
        let cxx_operator_less_than_or_equal: Option<syn::ForeignItemFn> = self.cxx_has_operator_less_than_or_equal.then(|| {
            syn::parse_quote! {
                fn cxx_operator_less_than_or_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
            }
        });
        let cxx_operator_greater_than: Option<syn::ForeignItemFn> = self.cxx_has_operator_greater_than.then(|| {
            syn::parse_quote! {
                fn cxx_operator_greater_than #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
            }
        });
        let cxx_operator_greater_than_or_equal: Option<syn::ForeignItemFn> = self
            .cxx_has_operator_greater_than_or_equal.then(||
        {
            syn::parse_quote! {
                fn cxx_operator_greater_than_or_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
            }
        });
        let cxx_operator_three_way_comparison: Option<syn::ForeignItemFn> = self.is_rust_partial_ord.then(|| {
            syn::parse_quote! {
                fn cxx_operator_three_way_comparison #generics (This: & #ident #generics, That: & #ident #generics) -> i8;
            }
        });
        let cxx_hash: Option<syn::ForeignItemFn> = self.is_rust_hash.then(|| {
            syn::parse_quote! {
                fn cxx_hash #generics (This: & #ident #generics) -> usize;
            }
        });
        let cxx_debug: Option<syn::ForeignItemFn> = self.is_rust_debug.then(|| {
            syn::parse_quote! {
                fn cxx_debug #generics (This: & #ident #generics) -> String;
            }
        });
        let cxx_display: Option<syn::ForeignItemFn> = self.is_rust_display.then(|| {
            syn::parse_quote! {
                fn cxx_display #generics (This: & #ident #generics) -> String;
            }
        });
        syn::parse_quote! {
            #[cxx::bridge]
            pub(crate) mod ffi {
                #![allow(clippy::needless_lifetimes)]
                #[namespace = #cxx_namespace]
                unsafe extern "C++" {
                    include!(#cxx_include);

                    #[cxx_name = #cxx_name]
                    #[allow(unused)]
                    type #ident #generics = super :: #ident #generics;
                    #cxx_copy_new
                    #cxx_move_new
                    #cxx_default_new
                    #cxx_destruct
                    #cxx_operator_equal
                    #cxx_operator_not_equal
                    #cxx_operator_less_than
                    #cxx_operator_less_than_or_equal
                    #cxx_operator_greater_than
                    #cxx_operator_greater_than_or_equal
                    #cxx_operator_three_way_comparison
                    #cxx_hash
                    #cxx_debug
                    #cxx_display
                }
            }
        }
    }

    fn codegen_item_mod_auto_type_test(
        &self,
        ident: &syn::Ident,
        align: &proc_macro2::Literal,
        size: &proc_macro2::Literal,
    ) -> syn::ItemMod {
        let (_, ref generics) = {
            let all_static = true;
            self.codegen_generics(all_static)
        };
        let static_assert_is_copy: Option<syn::ItemMacro> = self.is_rust_copy.then(|| {
            syn::parse_quote!(
                ::cxx_auto::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Copy);
            )
        });
        let static_assert_is_unpin: Option<syn::ItemMacro> = self.is_rust_unpin.then(|| {
            syn::parse_quote!(
                ::cxx_auto::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Unpin);
            )
        });
        syn::parse_quote! {
            #[cfg(test)]
            mod info {
                use super::*;
                mod test {
                    use super::*;
                    #[test]
                    fn cxx_abi_align() {
                        ::core::assert_eq!(::core::mem::align_of::<#ident #generics>(), #align)
                    }
                    #[test]
                    fn cxx_abi_size() {
                        ::core::assert_eq!(::core::mem::size_of::<#ident #generics>(), #size)
                    }
                    #static_assert_is_copy
                    #static_assert_is_unpin
                }
            }
        }
    }

    #[must_use]
    pub fn codegen_file_mod_auto_type(&self, auto_out_dir: &Path) -> syn::File {
        let span = proc_macro2::Span::call_site();
        let rust_type_name: &syn::Ident = &syn::Ident::new(self.rust_name, proc_macro2::Span::call_site());
        let cxx_abi_align = &proc_macro2::Literal::usize_unsuffixed(self.cxx_abi_align);
        let cxx_abi_size = &proc_macro2::Literal::usize_unsuffixed(self.cxx_abi_size);
        let (ref generics_binder, ref generics) = {
            let all_static = false;
            self.codegen_generics(all_static)
        };
        let items_path_descendants = self
            .path_descendants
            .iter()
            .map(|descendant| {
                let path = auto_out_dir.join(descendant).with_extension("rs");
                let path_str = path.to_string_lossy();
                let rust_mod_name = syn::Ident::new(descendant, span);
                syn::parse_quote! {
                    #[path = #path_str]
                    pub mod #rust_mod_name;
                }
            })
            .collect::<Vec<syn::Item>>();
        let item_struct =
            self.codegen_item_struct(cxx_abi_align, cxx_abi_size, rust_type_name, generics_binder, generics);
        let item_impl_cxx_extern_type =
            self.codegen_item_impl_cxx_extern_type(rust_type_name, generics_binder, generics);
        let item_impl_drop = self.codegen_item_impl_drop(rust_type_name, generics_binder, generics);
        let item_impl_debug = self.codegen_item_impl_debug(rust_type_name, generics_binder, generics);
        let item_impl_default = self.codegen_item_impl_default(rust_type_name, generics_binder, generics);
        let item_impl_display = self.codegen_item_impl_display(rust_type_name, generics_binder, generics);
        let item_impl_moveit_copy_new =
            self.codegen_item_impl_moveit_copy_new(rust_type_name, generics_binder, generics);
        let item_impl_moveit_move_new =
            self.codegen_item_impl_moveit_move_new(rust_type_name, generics_binder, generics);
        let item_impl_partial_eq = self.codegen_item_impl_partial_eq(rust_type_name, generics_binder, generics);
        let item_impl_eq = self.codegen_item_impl_eq(rust_type_name, generics_binder, generics);
        let item_impl_partial_ord = self.codegen_item_impl_partial_ord(rust_type_name, generics_binder, generics);
        let item_impl_ord = self.codegen_item_impl_ord(rust_type_name, generics_binder, generics);
        let item_impl_hash = self.codegen_item_impl_hash(rust_type_name, generics_binder, generics);
        let item_mod_auto_type_ffi = self.codegen_item_mod_auto_type_ffi(rust_type_name, generics);
        let item_mod_auto_type_test = self.codegen_item_mod_auto_type_test(rust_type_name, cxx_abi_align, cxx_abi_size);
        syn::parse_quote! {
            #(#items_path_descendants)*
            #item_struct
            #item_impl_cxx_extern_type
            #item_impl_drop
            #item_impl_default
            #item_impl_moveit_copy_new
            #item_impl_moveit_move_new
            #item_impl_partial_eq
            #item_impl_eq
            #item_impl_partial_ord
            #item_impl_ord
            #item_impl_hash
            #item_impl_debug
            #item_impl_display
            #item_mod_auto_type_ffi
            #item_mod_auto_type_test
        }
    }
}
