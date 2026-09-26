use proc_macro2::Span;
use syn::punctuated::Punctuated;

/// Resolved C++ layout and capabilities used to emit one Rust binding.
///
/// # Specification
/// - requires: the C++ probe results describe one stable type and its true
///   layout.
/// - ensures: each capability is retained independently for the generated type.
/// - panics: none.
#[expect(
    clippy::struct_excessive_bools,
    reason = "Each capability corresponds to a distinct C++ probe and Rust trait"
)]
#[cfg(feature = "alloc")]
pub struct CxxAutoArtifactInfo
{
    /// Path segments used to place the generated Rust type.
    pub path_components: ::alloc::vec::Vec<&'static str>,
    /// Immediate child module names below this type.
    pub path_descendants: ::alloc::vec::Vec<&'static str>,
    /// Header declaring the bridged C++ type.
    pub cxx_include: &'static str,
    /// Header declaring the proxy functions called by the generated binding.
    pub cxx_proxy_include: &'static str,
    /// C++ namespace containing the bridged type.
    pub cxx_namespace: &'static str,
    /// C++ namespace containing the proxy functions.
    pub cxx_proxy_namespace: &'static str,
    /// C++ type name used by the bridge.
    pub cxx_name: &'static str,
    /// Rust type name written in the generated module.
    pub rust_name: &'static str,
    /// Rust lifetime names and their outlives bounds.
    pub lifetimes: ::indexmap::IndexMap<&'static str, ::alloc::vec::Vec<&'static str>>,
    /// C++ type alignment in bytes.
    pub align: usize,
    /// C++ type size in bytes.
    pub size: usize,
    /// Whether const C++ equality returns bool.
    pub cxx_has_operator_equal: bool,
    /// Whether const C++ inequality returns bool.
    pub cxx_has_operator_not_equal: bool,
    /// Whether const C++ less-than returns bool.
    pub cxx_has_operator_less_than: bool,
    /// Whether const C++ less-than-or-equal returns bool.
    pub cxx_has_operator_less_than_or_equal: bool,
    /// Whether const C++ greater-than returns bool.
    pub cxx_has_operator_greater_than: bool,
    /// Whether const C++ greater-than-or-equal returns bool.
    pub cxx_has_operator_greater_than_or_equal: bool,
    /// Whether CXX may pass this type by value.
    pub is_rust_cxx_extern_type_trivial: bool,
    /// Whether Rust may move this type after pinning.
    pub is_rust_unpin: bool,
    /// Whether Rust may transfer this type between threads.
    pub is_rust_send: bool,
    /// Whether Rust may share references between threads.
    pub is_rust_sync: bool,
    /// Whether Rust may duplicate this type by bit copy.
    pub is_rust_copy: bool,
    /// Whether Rust delegates debug formatting to C++.
    pub is_rust_debug: bool,
    /// Whether Rust may default-construct this type.
    pub is_rust_default: bool,
    /// Whether Rust delegates display formatting to C++.
    pub is_rust_display: bool,
    /// Whether Rust must call its C++ destructor.
    pub is_rust_drop: bool,
    /// Whether Rust may invoke a C++ copy constructor.
    pub is_rust_copy_new: bool,
    /// Whether Rust may invoke a C++ move constructor.
    pub is_rust_move_new: bool,
    /// Whether Rust exposes total equality.
    pub is_rust_eq: bool,
    /// Whether Rust exposes partial equality.
    pub is_rust_partial_eq: bool,
    /// Whether Rust exposes partial ordering.
    pub is_rust_partial_ord: bool,
    /// Whether Rust exposes total ordering.
    pub is_rust_ord: bool,
    /// Whether Rust delegates hashing to C++.
    pub is_rust_hash: bool,
}

#[cfg(feature = "alloc")]
impl CxxAutoArtifactInfo
{
    /// Emit a Rust type whose layout and trait implementations follow these C++
    /// capabilities.
    ///
    /// # Specification
    /// - requires: Rust names and lifetime bounds are valid identifiers and
    ///   capability flags are coherent.
    /// - ensures: emits the declared aligned layout, bridge methods, and
    ///   selected Rust traits.
    /// - panics: syn rejects invalid generated identifiers.
    #[inline]
    #[must_use]
    pub fn emit_file(
        &self,
        auto_out_dir: &::std::path::Path,
    ) -> syn::File
    {
        let span = Span::call_site();
        let ident: &syn::Ident = &syn::Ident::new(self.rust_name, Span::call_site());
        let align = &proc_macro2::Literal::usize_unsuffixed(self.align);
        let size = &proc_macro2::Literal::usize_unsuffixed(self.size);
        let generics_pair = emit_generics(self, false);
        let (generics_binder, generics) = (&generics_pair.0, &generics_pair.1);
        let items_path_descendants = self
            .path_descendants
            .iter()
            .map(|descendant| {
                let path = auto_out_dir.join(descendant).with_extension("rs");
                let path = path.to_string_lossy();
                let ident = syn::Ident::new(descendant, span);
                syn::parse_quote! {
                    #[path = #path]
                    pub(crate) mod #ident;
                }
            })
            .collect::<alloc::vec::Vec<syn::Item>>();
        let item_struct = emit_struct(self, align, size, ident, generics_binder, generics);
        let item_impl_cxx_extern_type =
            emit_impl_cxx_extern_type(self, ident, generics_binder, generics);
        let item_impl_drop = emit_impl_drop(self, ident, generics_binder, generics);
        let item_impl_debug = emit_impl_debug(self, ident, generics_binder, generics);
        let item_impl_default = emit_impl_default(self, ident, generics_binder, generics);
        let item_impl_display = emit_impl_display(self, ident, generics_binder, generics);
        let item_impl_moveit_copy_new =
            emit_impl_moveit_copy_new(self, ident, generics_binder, generics);
        let item_impl_moveit_move_new =
            emit_impl_moveit_move_new(self, ident, generics_binder, generics);
        let item_impl_partial_eq = emit_impl_partial_eq(self, ident, generics_binder, generics);
        let item_impl_eq = emit_impl_eq(self, ident, generics_binder, generics);
        let item_impl_partial_ord = emit_impl_partial_ord(self, ident, generics_binder, generics);
        let item_impl_ord = emit_impl_ord(self, ident, generics_binder, generics);
        let item_impl_hash = emit_impl_hash(self, ident, generics_binder, generics);
        let item_mod_cxx_bridge = emit_item_mod_cxx_bridge(self, ident, generics);
        let item_info_test_module = emit_info_test_module(self, ident, align, size);
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
            #item_mod_cxx_bridge
            #item_info_test_module
        }
    }

    /// Write an intermediate module referencing its immediate descendants.
    ///
    /// # Specification
    /// - ensures: the module at `path_components` references exactly
    ///   `path_descendants`.
    /// - fails: I/O and formatter failures propagate with their causes.
    /// - panics: syn rejects invalid module identifiers.
    /// # Errors
    ///
    /// Will return `Err` under the following circumstances:
    /// - failure to create the output parent directory for the generated module
    /// - failure to run `rustfmt` on the generated module
    /// - failure to write the generated module to disk
    #[cfg(feature = "std")]
    #[inline]
    pub fn write_module_for_dir(
        auto_out_dir_root: &std::path::Path,
        path_components: &[&str],
        path_descendants: &[&str],
    ) -> crate::BoxResult<()>
    {
        use quote::ToTokens as _;
        use rust_format::Formatter as _;
        let auto_out_dir = auto_out_dir_root.join(std::path::PathBuf::from_iter(path_components));
        if let Some(parent) = auto_out_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let path = auto_out_dir.with_extension("rs");
        let file: syn::File = {
            let span = Span::call_site();
            let items = path_descendants
                .iter()
                .map(|descendant| {
                    let path = auto_out_dir.join(descendant).with_extension("rs");
                    let path = path.to_string_lossy();
                    let ident = syn::Ident::new(descendant, span);
                    syn::parse_quote! {
                        #[path = #path]
                        pub(crate) mod #ident;
                    }
                })
                .collect::<alloc::vec::Vec<syn::Item>>();
            syn::File {
                shebang: None,
                frontmatter: None,
                attrs: alloc::vec![],
                items,
            }
        };
        let tokens = file.to_token_stream();
        let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
    /// Write this type and its bridge as one generated Rust module.
    ///
    /// # Specification
    /// - ensures: the module at `path_components` contains this type and its
    ///   selected capabilities.
    /// - fails: I/O and formatter failures propagate with their causes.
    /// - panics: syn rejects invalid Rust names.
    ///
    /// # Errors
    ///
    /// Will return `Err` under the following circumstances:
    /// - failure to create the output parent directory for the generated module
    /// - failure to run `rustfmt` on the generated module
    /// - failure to write the generated module to disk
    #[cfg(feature = "std")]
    #[inline]
    pub fn write_module_for_file(
        &self,
        auto_out_dir_root: &::std::path::Path,
    ) -> crate::BoxResult<()>
    {
        use quote::ToTokens as _;
        use rust_format::Formatter as _;
        let auto_out_dir =
            auto_out_dir_root.join(std::path::PathBuf::from_iter(&self.path_components));
        if let Some(parent) = auto_out_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let path = auto_out_dir.with_extension("rs");
        let file = self.emit_file(&auto_out_dir);
        let tokens = file.to_token_stream();
        let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

/// Build the aligned Rust struct and its capability marker fields.
///
/// # Specification
/// - provides: an aligned C-layout struct with byte storage and only the
///   markers demanded by the capability flags.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_struct(
    info: &CxxAutoArtifactInfo,
    align: &proc_macro2::Literal,
    size: &proc_macro2::Literal,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemStruct
{
    let attribute = emit_derive_attribute(info);
    let field_layout = field_layout(size);
    let field_neither_send_nor_sync = field_neither_send_nor_sync(info);
    let field_pinned = field_pinned(info);
    let field_lifetimes = field_lifetimes(generics);
    let fields = syn::FieldsNamed {
        brace_token: syn::token::Brace::default(),
        named: ::alloc::vec![
            Some(field_layout),
            field_neither_send_nor_sync,
            field_pinned,
            field_lifetimes,
        ]
        .into_iter()
        .flatten()
        .collect::<Punctuated<syn::Field, syn::Token![,]>>(),
    };
    syn::parse_quote! {
        #attribute
        #[repr(C, align(#align))]
        pub struct #ident #generics_binder #fields
    }
}

/// Select the Copy derive when C++ permits bitwise duplication.
///
/// # Specification
/// - provides: a `Copy` and `Clone` derive only when `is_rust_copy` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_derive_attribute(info: &CxxAutoArtifactInfo) -> Option<syn::Attribute>
{
    info.is_rust_copy
        .then(|| syn::parse_quote!(#[derive(Clone, Copy)]))
}

/// Construct a private named field for a generated struct.
///
/// # Specification
/// - provides: a syn field with the requested identifier and type.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_field(
    name: &str,
    ty: syn::Type,
) -> syn::Field
{
    syn::Field {
        attrs: ::alloc::vec![],
        vis: syn::Visibility::Inherited,
        modifiers: syn::FieldModifiers::default(),
        ident: Some(syn::Ident::new(name, Span::call_site())),
        colon_token: Some(syn::Token![:](Span::call_site())),
        ty,
        default: None,
    }
}

/// Generate the lifetime binder and the type-use parameters.
///
/// # Specification
/// - provides: declared outlives bounds on the binder and matching lifetime
///   arguments on the type.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_generics(
    info: &CxxAutoArtifactInfo,
    all_static: bool,
) -> (syn::Generics, syn::Generics)
{
    let span = Span::call_site();
    let mut binder_params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    let mut params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    for (name, bounds) in &info.lifetimes {
        let name = if all_static { "static" } else { name };

        let lifetime = syn::Lifetime::new(&::alloc::format!("'{name}"), span);
        let lifetime_param = syn::LifetimeParam::new(lifetime);

        let mut lifetime_param_binder = lifetime_param.clone();
        for bound in bounds {
            let lifetime = syn::Lifetime::new(&::alloc::format!("'{bound}"), span);
            lifetime_param_binder.bounds.push_value(lifetime);
        }

        binder_params.push(syn::GenericParam::Lifetime(lifetime_param_binder));
        params.push(syn::GenericParam::Lifetime(lifetime_param));
    }
    let lt_token = if params.is_empty() {
        None
    }
    else {
        Some(syn::Token![<](span))
    };
    let gt_token = if params.is_empty() {
        None
    }
    else {
        Some(syn::Token![>](span))
    };
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

/// Associate a generated Rust type with its CXX external type identity.
///
/// # Specification
/// - provides: an unsafe `ExternType` impl with the C++ type id and selected
///   representation kind.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_cxx_extern_type(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemImpl
{
    let type_id = ::alloc::format!("{}::{}", info.cxx_namespace, info.cxx_name);
    let kind: syn::Type = if info.is_rust_cxx_extern_type_trivial {
        syn::parse_quote!(::cxx::kind::Trivial)
    }
    else {
        syn::parse_quote!(::cxx::kind::Opaque)
    };
    syn::parse_quote! {
        unsafe impl #generics_binder ::cxx::ExternType for #ident #generics {
            type Id = ::cxx::type_id!(#type_id);
            type Kind = #kind;
        }
    }
}

/// Select C++ destruction for types requiring nontrivial drop.
///
/// # Specification
/// - provides: a `Drop` impl only when `is_rust_drop` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_drop(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_drop.then(|| {
        syn::parse_quote! {
            impl #generics_binder ::core::ops::Drop for #ident #generics {
                #[cfg_attr(feature = "tracing", tracing::instrument)]
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

/// Select C++-backed Debug formatting.
///
/// # Specification
/// - provides: a `Debug` impl only when `is_rust_debug` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_debug(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemImpl
{
    if info.is_rust_debug {
        syn::parse_quote! {
            impl #generics_binder ::core::fmt::Debug for #ident #generics {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let string = self::ffi::cxx_debug(self);
                    write!(f, "{string}")
                }
            }
        }
    }
    else {
        let name = info.rust_name;
        syn::parse_quote! {
            impl #generics_binder ::core::fmt::Debug for #ident #generics {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name).finish()
                }
            }
        }
    }
}

/// Select the C++ default constructor wrapper.
///
/// # Specification
/// - provides: a `default_new` inherent method only when `is_rust_default` is
///   true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_default(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_default.then(|| {
        syn::parse_quote! {
            impl #generics_binder #ident #generics {
                #[inline]
                pub(crate) fn default_new() -> impl ::moveref::New<Output = #ident #generics> {
                    unsafe {
                        ::moveref::new::by_raw(move |this| {
                            let this = this.get_unchecked_mut().as_mut_ptr();
                            self::ffi::cxx_default_new(this);
                        })
                    }
                }
            }
        }
    })
}

/// Select C++-backed Display formatting.
///
/// # Specification
/// - provides: a `Display` impl only when `is_rust_display` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_display(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_display.then(|| {
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

/// Select a C++ copy constructor implementation.
///
/// # Specification
/// - provides: a `CopyNew` impl only when `is_rust_copy_new` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_moveit_copy_new(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_copy_new.then(|| syn::parse_quote! {
            impl #generics_binder ::moveref::CopyNew for #ident #generics {
                #[inline]
                unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
                    let this = this.get_unchecked_mut().as_mut_ptr();
                    self::ffi::cxx_copy_new(this, that);
                }
            }
        })
}

/// Select a C++ move constructor implementation.
///
/// # Specification
/// - provides: a `MoveNew` impl only when `is_rust_move_new` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_moveit_move_new(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_move_new.then(|| {
        syn::parse_quote! {
            impl #generics_binder ::moveref::MoveNew for #ident #generics {
                #[inline]
                unsafe fn move_new(
                    that: ::core::pin::Pin<::moveref::MoveRef<'_, Self>>,
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

/// Select a Rust partial-equality implementation.
///
/// # Specification
/// - provides: a `PartialEq` impl when `is_rust_partial_eq` is true, with `ne`
///   only if C++ supplies it.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_partial_eq(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_partial_eq.then(|| {
        let ne: Option<syn::ImplItemFn> = info.cxx_has_operator_not_equal.then(|| {
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

/// Select a Rust total-equality marker.
///
/// # Specification
/// - provides: an `Eq` impl only when `is_rust_eq` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_eq(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_eq.then(|| {
        syn::parse_quote! {
            impl #generics_binder ::core::cmp::Eq for #ident #generics {}
        }
    })
}

/// Select a Rust partial-ordering implementation.
///
/// # Specification
/// - provides: a `PartialOrd` impl and only the comparison overrides supported
///   by C++.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_partial_ord(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_partial_ord.then(|| {
        let lt: Option<syn::ImplItemFn> = info.cxx_has_operator_less_than.then(|| {
            syn::parse_quote! {
                #[inline]
                fn lt(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_less_than(self, other)
                }
            }
        });
        let le: Option<syn::ImplItemFn> = info.cxx_has_operator_less_than_or_equal.then(|| {
            syn::parse_quote! {
                #[inline]
                fn le(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_less_than_or_equal(self, other)
                }
            }
        });
        let gt: Option<syn::ImplItemFn> = info.cxx_has_operator_greater_than.then(|| {
            syn::parse_quote! {
                #[inline]
                fn gt(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_greater_than(self, other)
                }
            }
        });
        let ge: Option<syn::ImplItemFn> = info.cxx_has_operator_greater_than_or_equal.then(|| {
            syn::parse_quote! {
                #[inline]
                fn ge(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_greater_than_or_equal(self, other)
                }
            }
        });
        let partial_cmp: syn::ImplItemFn = if info.is_rust_ord {
            syn::parse_quote! {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
        }
        else {
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

/// Select a Rust total-ordering implementation.
///
/// # Specification
/// - provides: an `Ord` impl only when `is_rust_ord` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_ord(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_ord.then(|| {
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

/// Select a C++-backed hash implementation.
///
/// # Specification
/// - provides: a `Hash` impl only when `is_rust_hash` is true.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_hash(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    info.is_rust_hash.then(|| {
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

/// Emit runtime layout checks for the generated type.
///
/// # Specification
/// - provides: size and alignment tests, with positive Copy and Unpin
///   assertions when declared.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_info_test_module(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    align: &proc_macro2::Literal,
    size: &proc_macro2::Literal,
) -> syn::ItemMod
{
    let generics_pair = emit_generics(info, true);
    let generics = &generics_pair.1;
    let static_assert_is_copy: Option<syn::ItemMacro> = info.is_rust_copy.then(|| {
        syn::parse_quote!(
            ::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Copy);
        )
    });
    let static_assert_is_unpin: Option<syn::ItemMacro> = info.is_rust_unpin.then(|| {
        syn::parse_quote!(
            ::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Unpin);
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

/// Generate CXX declarations only for the constructors, comparisons, and traits
/// permitted by metadata.
///
/// # Specification
/// - requires: the proxy header declares each selected helper in
///   `cxx_proxy_namespace`, and the C++ type is declared in `cxx_namespace`.
/// - provides: a CXX bridge that uses the type namespace for its type alias and
///   the proxy namespace for selected constructors, comparisons, and traits.
/// - panics: syn rejects malformed Rust identifiers.
///
/// # Adequacy
/// - hypothesis: a type and its proxy in distinct namespaces must compile and
///   return the C++ comparison ordering, including equality and reversed
///   operands.
/// - witness: `tests/dynamic_binding.rs`
///   (`loads_generated_comparison_binding`).
#[cfg(feature = "alloc")]
fn emit_item_mod_cxx_bridge(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics: &syn::Generics,
) -> syn::ItemMod
{
    let cxx_proxy_include = &info.cxx_proxy_include;
    let cxx_namespace = &info.cxx_namespace;
    let cxx_proxy_namespace = &info.cxx_proxy_namespace;
    let cxx_name = &info.cxx_name;
    let cxx_copy_new: Option<syn::ForeignItemFn> = info.is_rust_copy_new.then(|| {
        syn::parse_quote! {
            unsafe fn cxx_copy_new #generics (This: *mut #ident #generics, that: &#ident #generics);
        }
    });
    let cxx_move_new: Option<syn::ForeignItemFn> = info.is_rust_move_new.then(|| syn::parse_quote! {
            unsafe fn cxx_move_new #generics (This: *mut #ident #generics, that: *mut #ident #generics);
        });
    let cxx_default_new: Option<syn::ForeignItemFn> = info.is_rust_default.then(|| {
        syn::parse_quote! {
            unsafe fn cxx_default_new #generics (This: *mut #ident #generics);
        }
    });
    let cxx_destruct: Option<syn::ForeignItemFn> = info.is_rust_drop.then(|| {
        syn::parse_quote! {
            unsafe fn cxx_destruct #generics (This: *mut #ident #generics);
        }
    });
    let cxx_operator_equal: Option<syn::ForeignItemFn> = info.is_rust_eq.then(|| syn::parse_quote! {
            fn cxx_operator_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        });
    let cxx_operator_not_equal: Option<syn::ForeignItemFn> = info.is_rust_eq.then(|| syn::parse_quote! {
            fn cxx_operator_not_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        });
    let cxx_operator_less_than: Option<syn::ForeignItemFn> = info.cxx_has_operator_less_than.then(|| syn::parse_quote! {
            fn cxx_operator_less_than #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        });
    let cxx_operator_less_than_or_equal: Option<syn::ForeignItemFn> = info.cxx_has_operator_less_than_or_equal.then(|| syn::parse_quote! {
            fn cxx_operator_less_than_or_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        });
    let cxx_operator_greater_than: Option<syn::ForeignItemFn> = info.cxx_has_operator_greater_than.then(|| syn::parse_quote! {
            fn cxx_operator_greater_than #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        });
    let cxx_operator_greater_than_or_equal: Option<syn::ForeignItemFn> = info.cxx_has_operator_greater_than_or_equal.then(|| syn::parse_quote! {
            fn cxx_operator_greater_than_or_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        });
    let cxx_operator_three_way_comparison: Option<syn::ForeignItemFn> = info.is_rust_partial_ord.then(|| syn::parse_quote! {
            fn cxx_operator_three_way_comparison #generics (This: & #ident #generics, That: & #ident #generics) -> i8;
        });
    let cxx_hash: Option<syn::ForeignItemFn> = info.is_rust_hash.then(|| {
        syn::parse_quote! {
            fn cxx_hash #generics (This: & #ident #generics) -> usize;
        }
    });
    let cxx_debug: Option<syn::ForeignItemFn> = info.is_rust_debug.then(|| {
        syn::parse_quote! {
            fn cxx_debug #generics (This: & #ident #generics) -> String;
        }
    });
    let cxx_display: Option<syn::ForeignItemFn> = info.is_rust_display.then(|| {
        syn::parse_quote! {
            fn cxx_display #generics (This: & #ident #generics) -> String;
        }
    });
    syn::parse_quote! {
        #[cxx::bridge]
        pub(crate) mod ffi {
            #![allow(clippy::needless_lifetimes)]
            #[namespace = #cxx_proxy_namespace]
            unsafe extern "C++" {
                include!(#cxx_proxy_include);

                #[namespace = #cxx_namespace]
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
/// Collect lifetime references for a generated phantom field.
///
/// # Specification
/// - provides: one reference type per declared lifetime parameter.
/// - panics: malformed generated identifiers may be rejected by syn.
fn emit_refs_from_lifetimes(generics: &syn::Generics) -> Punctuated<syn::Type, syn::Token![,]>
{
    generics
        .params
        .iter()
        .filter_map(emit_ref_type_from_lifetime)
        .collect::<Punctuated<syn::Type, syn::Token![,]>>()
}
/// Project a lifetime parameter into a reference type.
///
/// # Specification
/// - provides: a unit reference for lifetime parameters and no type for other
///   parameters.
/// - panics: malformed generated identifiers may be rejected by syn.
fn emit_ref_type_from_lifetime(generic_param: &syn::GenericParam) -> Option<syn::Type>
{
    if let syn::GenericParam::Lifetime(ref lifetime_param) = *generic_param {
        let lifetime = &lifetime_param.lifetime;
        Some(syn::parse_quote!(&#lifetime ()))
    }
    else {
        None
    }
}
/// Allocate byte storage matching the C++ size.
///
/// # Specification
/// - provides: an uninitialized byte-array field sized to the C++ type.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn field_layout(size: &proc_macro2::Literal) -> syn::Field
{
    let name = "_layout";
    let ty = syn::parse_quote!([u8; #size]);
    emit_field(name, ty)
}
/// Prevent unintended Send and Sync auto-traits.
///
/// # Specification
/// - provides: a marker field when both send and sync are unavailable.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn field_neither_send_nor_sync(info: &CxxAutoArtifactInfo) -> Option<syn::Field>
{
    let is_neither_send_nor_sync = !info.is_rust_send && !info.is_rust_sync;
    is_neither_send_nor_sync.then(|| {
        let name = "_neither_send_nor_sync";
        let ty = syn::parse_quote!(::core::marker::PhantomData<[*const u8; 0]>);
        emit_field(name, ty)
    })
}
/// Pin types whose C++ move semantics forbid Rust Unpin.
///
/// # Specification
/// - provides: a pin marker only when `is_rust_unpin` is false.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn field_pinned(info: &CxxAutoArtifactInfo) -> Option<syn::Field>
{
    if info.is_rust_unpin {
        None
    }
    else {
        let name = "_pinned";
        let ty = syn::parse_quote!(::core::marker::PhantomPinned);
        Some(emit_field(name, ty))
    }
}
/// Carry declared Rust lifetimes into the struct representation.
///
/// # Specification
/// - provides: a phantom field for all lifetime reference types when any are
///   declared.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn field_lifetimes(generics: &syn::Generics) -> Option<syn::Field>
{
    let ref_types = emit_refs_from_lifetimes(generics);
    if ref_types.is_empty() {
        None
    }
    else {
        let name = "_lifetimes";
        let ty = syn::parse_quote!(::core::marker::PhantomData<(#ref_types,)>);
        Some(emit_field(name, ty))
    }
}
