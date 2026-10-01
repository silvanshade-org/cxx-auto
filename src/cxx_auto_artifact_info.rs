use proc_macro2::Span;
use syn::punctuated::Punctuated;

/// Resolved C++ layout and capabilities used to emit one Rust binding.
///
/// # Specification
/// - requires: the fields come from one decoded type record (`crate::record`),
///   so names are valid identifiers and the layout is the C++ type's own.
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
    pub path_components: ::alloc::vec::Vec<::alloc::string::String>,
    /// Immediate child module names below this type.
    pub path_descendants: ::alloc::vec::Vec<::alloc::string::String>,
    /// Header declaring the proxy functions called by the generated binding.
    pub cxx_proxy_include: ::alloc::string::String,
    /// C++ namespace containing the bridged type.
    pub cxx_namespace: ::alloc::string::String,
    /// C++ namespace containing the proxy functions.
    pub cxx_proxy_namespace: ::alloc::string::String,
    /// C++ type name used by the bridge.
    pub cxx_name: ::alloc::string::String,
    /// Rust type name written in the generated module.
    pub rust_name: ::alloc::string::String,
    /// Rust lifetime names, without the leading `'`, each with the names of
    /// the lifetimes it outlives, in declaration order.
    pub lifetimes: ::alloc::vec::Vec<(
        ::alloc::string::String,
        ::alloc::vec::Vec<::alloc::string::String>,
    )>,
    /// C++ type alignment in bytes.
    pub align: usize,
    /// C++ type size in bytes.
    pub size: usize,
    /// Whether C++ declares its own `operator!=`, which Rust's `ne` then calls.
    pub cxx_has_operator_not_equal: bool,
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
    /// Whether Rust may invoke the C++ copy assignment operator.
    pub is_rust_copy_assign: bool,
    /// Whether Rust may invoke the C++ move assignment operator.
    pub is_rust_move_assign: bool,
    /// Which special member functions C++ declares unable to throw.
    pub cxx_nothrow: CxxNothrow,
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

/// Which C++ special member functions cannot throw. A generated initializer or
/// assignment is infallible when its operation cannot throw, and otherwise
/// returns the caught exception as [`CxxException`](crate::init::CxxException).
#[expect(
    clippy::struct_excessive_bools,
    reason = "Each flag is a distinct C++ `is_nothrow_*` probe for one operation"
)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CxxNothrow
{
    /// Default construction.
    pub default_new: bool,
    /// Copy construction.
    pub copy_new: bool,
    /// Move construction.
    pub move_new: bool,
    /// Copy assignment.
    pub copy_assign: bool,
    /// Move assignment.
    pub move_assign: bool,
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
        let ident: &syn::Ident = &syn::Ident::new(&self.rust_name, Span::call_site());
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
                    /// A descendant of this generated binding module.
                    ///
                    /// # Specification
                    /// trivial.
                    #[path = #path]
                    pub(crate) mod #ident;
                }
            })
            .collect::<alloc::vec::Vec<syn::Item>>();
        let item_struct = emit_struct(self, align, size, ident, generics_binder, generics);
        let item_impl_cxx_extern_type =
            emit_impl_cxx_extern_type(self, ident, generics_binder, generics);
        let items_impl_send_sync = emit_impls_send_sync(self, ident, generics_binder, generics);
        let item_impl_drop = emit_impl_drop(self, ident, generics_binder, generics);
        let item_impl_debug = emit_impl_debug(self, ident, generics_binder, generics);
        let item_impl_construction = emit_impl_construction(self, ident, generics_binder, generics);
        let item_impl_display = emit_impl_display(self, ident, generics_binder, generics);
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
            #(#items_impl_send_sync)*
            #item_impl_drop
            #item_impl_construction
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
        path_components: &[::alloc::string::String],
        path_descendants: &[::alloc::string::String],
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
                        /// A descendant of this generated binding module.
                        ///
                        /// # Specification
                        /// trivial.
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
        let contents = rust_format::RustFmt::from_config(
            rust_format::Config::new_str().option("wrap_comments", "false"),
        )
        .format_tokens(tokens)?;
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
        // Preserve specification-list indentation instead of reflowing its clauses.
        let contents = rust_format::RustFmt::from_config(
            rust_format::Config::new_str().option("wrap_comments", "false"),
        )
        .format_tokens(tokens)?;
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
        /// Rust storage for the recorded C++ type.
        ///
        /// # Specification
        /// - ensures: size and alignment match the C++ type record.
        /// - provides: owned storage with the recorded lifetime and capability boundaries.
        /// - panics: none.
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
        attrs: ::alloc::vec![
            syn::parse_quote!(#[doc = r#"Private C++ representation or capability marker.

# Specification
trivial."#])
        ],
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
pub fn emit_generics(
    info: &CxxAutoArtifactInfo,
    all_static: bool,
) -> (syn::Generics, syn::Generics)
{
    let span = Span::call_site();
    let mut binder_params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    let mut params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    for lifetime_and_bounds in &info.lifetimes {
        let bounds = &lifetime_and_bounds.1;
        let name = if all_static {
            "static"
        }
        else {
            lifetime_and_bounds.0.as_str()
        };

        let lifetime = syn::Lifetime::new(&::alloc::format!("'{name}"), span);
        let lifetime_param = syn::LifetimeParam::new(lifetime);

        let mut lifetime_param_binder = lifetime_param.clone();
        for bound in bounds {
            let lifetime = syn::Lifetime::new(&::alloc::format!("'{bound}"), span);
            lifetime_param_binder.bounds.push(lifetime);
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
        /// Associate storage with its recorded C++ identity.
        ///
        /// # Safety
        /// - unsafe invariants: the record and bridge name the same C++ type;
        ///   storage matches its size and alignment. Trivial kind requires
        ///   C++ relocation and destruction to permit value passing.
        // SAFETY: the compiled type record supplies this identity and layout.
        unsafe impl #generics_binder ::cxx::ExternType for #ident #generics {
            /// The fully qualified C++ type identity.
            type Id = ::cxx::type_id!(#type_id);
            /// The recorded CXX value-passing capability.
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
                /// Destroy the C++ object in its owner's storage.
                ///
                /// # Specification
                /// - ensures: the live object's C++ destructor runs exactly once in place.
                /// - provides: release of resources owned by the C++ object.
                /// - panics: a throwing C++ destructor terminates at the native noexcept boundary.
                #[cfg_attr(feature = "tracing", tracing::instrument)]
                #[inline]
                fn drop(&mut self) {
                    // SAFETY: self is a live, exclusively borrowed C++ object;
                    // its noexcept destructor runs in place.
                    unsafe {
                        self::ffi::cxx_destruct(::core::ptr::from_mut(self));
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
                /// Format the C++ debug representation.
                ///
                /// # Specification
                /// - ensures: writes the C++ debug string to the formatter.
                /// - provides: the formatter's completion result.
                /// - fails: returns `fmt::Error` when the formatter rejects output.
                /// - panics: the formatting sink may panic; a C++ rendering or string-allocation
                ///   exception terminates at the native noexcept boundary.
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let string = self::ffi::cxx_debug(self);
                    write!(f, "{string}")
                }
            }
        }
    }
    else {
        let name = info.rust_name.as_str();
        syn::parse_quote! {
            impl #generics_binder ::core::fmt::Debug for #ident #generics {
                /// Format the type name when C++ supplies no debug representation.
                ///
                /// # Specification
                /// - ensures: writes a debug struct containing the type name and no fields.
                /// - provides: the formatter's completion result.
                /// - fails: returns `fmt::Error` when the formatter rejects output.
                /// - panics: the formatting sink may panic.
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name).finish()
                }
            }
        }
    }
}

/// Build a generated initializer or assignment body around one C++ call.
///
/// # Specification
/// - provides: when `nothrow`, a call to the plain shim `plain` with `args`,
///   evaluating to `Ok(())`; otherwise a call to the catching shim `catching`
///   through `cxx_auto::init::cxx_try`, which supplies its `what` argument.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_cxx_call(
    nothrow: bool,
    plain: &str,
    catching: &str,
    args: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream
{
    let span = proc_macro2::Span::call_site();
    if nothrow {
        let plain = syn::Ident::new(plain, span);
        quote::quote! {
            // SAFETY: the operation cannot throw, and the arguments satisfy the
            // shim's contract.
            unsafe { self::ffi::#plain(#args) };
            ::core::result::Result::Ok(())
        }
    }
    else {
        let catching = syn::Ident::new(catching, span);
        quote::quote! {
            ::cxx_auto::init::cxx_try(|what| {
                // SAFETY: the shim catches every exception and reports it
                // through `what`; the arguments satisfy its contract.
                unsafe { self::ffi::#catching(#args, what) }
            })
        }
    }
}

/// The error type of a generated operation.
#[cfg(feature = "alloc")]
fn emit_error_type(nothrow: bool) -> syn::Type
{
    if nothrow {
        syn::parse_quote!(::core::convert::Infallible)
    }
    else {
        syn::parse_quote!(::cxx_auto::init::CxxException)
    }
}

/// Emit C++ construction and assignment as initializers and methods.
///
/// # Specification
/// - provides: `default_new`, `copy_from`, and `move_from` initializers (`impl
///   PinInit<Self, E>`) and `copy_assign` / `move_assign` methods on `Pin<&mut
///   Self>`, each only when C++ supports the operation. An operation C++
///   declares `noexcept` has error type `Infallible` (assignments return `()`);
///   any other returns the caught exception as `CxxException`.
/// - ensures: a moved-from source stays in its own place, owned and later
///   destroyed by its owner; nothing is relocated bytewise.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impl_construction(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl>
{
    let mut methods = emit_initializer_methods(info);
    methods.extend(emit_assignment_methods(info));
    (!methods.is_empty()).then(|| {
        syn::parse_quote! {
            impl #generics_binder #ident #generics {
                #(#methods)*
            }
        }
    })
}

/// Emit the generated initializers: `default_new`, `copy_from`, `move_from`.
///
/// # Specification
/// - provides: one method per construction C++ supports; see
///   [`emit_impl_construction`].
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_initializer_methods(info: &CxxAutoArtifactInfo) -> alloc::vec::Vec<syn::ImplItemFn>
{
    let nothrow = info.cxx_nothrow;
    let mut methods: alloc::vec::Vec<syn::ImplItemFn> = alloc::vec::Vec::new();
    if info.is_rust_default {
        let error = emit_error_type(nothrow.default_new);
        let call = emit_cxx_call(
            nothrow.default_new,
            "cxx_default_new",
            "cxx_try_default_new",
            &quote::quote!(this),
        );
        methods.push(syn::parse_quote! {
            /// The C++ default constructor, run in the owner's storage.
            ///
            /// # Specification
            /// - ensures: running the initializer constructs Self in the owner's storage;
            ///   success leaves a live pinned object, failure leaves nothing to destroy.
            /// - provides: an initializer with the recorded C++ exception capability.
            /// - fails: a throwing constructor returns `CxxException` with the caught message;
            ///   a noexcept constructor has error type Infallible.
            /// - panics: running a throwing C++ noexcept operation, or a throwing
            ///   operation without exception support, terminates.
            #[inline]
            pub(crate) fn default_new() -> impl ::cxx_auto::init::PinInit<Self, #error> {
                let body = move |this: *mut Self| -> ::core::result::Result<(), #error> { #call };
                // SAFETY: the constructor either fully initializes `this` or
                // throws, in which case C++ has destroyed anything it built.
                unsafe { ::cxx_auto::init::pin_init_from_closure(body) }
            }
        });
    }
    if info.is_rust_copy_new {
        let error = emit_error_type(nothrow.copy_new);
        let call = emit_cxx_call(
            nothrow.copy_new,
            "cxx_copy_new",
            "cxx_try_copy_new",
            &quote::quote!(this, that),
        );
        methods.push(syn::parse_quote! {
            /// The C++ copy constructor, run in the owner's storage.
            ///
            /// # Specification
            /// - ensures: running the initializer copy-constructs Self in the owner's
            ///   storage without modifying that; failure leaves nothing to destroy.
            /// - provides: an initializer borrowing that until construction finishes.
            /// - fails: a throwing constructor returns `CxxException` with the caught message;
            ///   a noexcept constructor has error type Infallible.
            /// - panics: running a throwing C++ noexcept operation, or a throwing
            ///   operation without exception support, terminates.
            #[inline]
            pub(crate) fn copy_from(that: &Self) -> impl ::cxx_auto::init::PinInit<Self, #error> + '_ {
                let body = move |this: *mut Self| -> ::core::result::Result<(), #error> { #call };
                // SAFETY: as for `default_new`; `that` stays borrowed until
                // the initializer runs.
                unsafe { ::cxx_auto::init::pin_init_from_closure(body) }
            }
        });
    }
    if info.is_rust_move_new {
        let error = emit_error_type(nothrow.move_new);
        let call = emit_cxx_call(
            nothrow.move_new,
            "cxx_move_new",
            "cxx_try_move_new",
            &quote::quote!(this, that),
        );
        methods.push(syn::parse_quote! {
            /// The C++ move constructor, run in the owner's storage. The
            /// source is left moved-from in its own place, and its owner
            /// still destroys it.
            ///
            /// # Specification
            /// - ensures: running the initializer move-constructs Self in the owner's
            ///   storage; that stays pinned, moved-from, and owned by its original owner.
            /// - provides: an initializer exclusively borrowing that until construction finishes.
            /// - fails: a throwing constructor returns `CxxException` with the caught message
            ///   and leaves no destination to destroy; a noexcept constructor has error type
            ///   Infallible. Source state on failure follows its C++ constructor.
            /// - panics: running a throwing C++ noexcept operation, or a throwing
            ///   operation without exception support, terminates.
            #[inline]
            pub(crate) fn move_from(
                that: ::core::pin::Pin<&mut Self>,
            ) -> impl ::cxx_auto::init::PinInit<Self, #error> + '_ {
                // SAFETY: C++ moves from `that` in place and never relocates it.
                let that: *mut Self = ::core::ptr::from_mut(unsafe { ::core::pin::Pin::into_inner_unchecked(that) });
                let body = move |this: *mut Self| -> ::core::result::Result<(), #error> { #call };
                // SAFETY: as for `default_new`; `that` stays exclusively
                // borrowed until the initializer runs.
                unsafe { ::cxx_auto::init::pin_init_from_closure(body) }
            }
        });
    }
    methods
}

/// Emit the generated assignments: `copy_assign`, `move_assign`.
///
/// # Specification
/// - provides: one method per assignment C++ supports; see
///   [`emit_impl_construction`].
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_assignment_methods(info: &CxxAutoArtifactInfo) -> alloc::vec::Vec<syn::ImplItemFn>
{
    let nothrow = info.cxx_nothrow;
    let mut methods: alloc::vec::Vec<syn::ImplItemFn> = alloc::vec::Vec::new();
    if info.is_rust_copy_assign {
        let error = emit_error_type(nothrow.copy_assign);
        let this = quote::quote!(::core::ptr::from_mut(unsafe {
            ::core::pin::Pin::into_inner_unchecked(self)
        }));
        let call = emit_cxx_call(
            nothrow.copy_assign,
            "cxx_copy_assign",
            "cxx_try_copy_assign",
            &quote::quote!(this, that),
        );
        let (output, finish) = assignment_result(nothrow.copy_assign, &error);
        methods.push(syn::parse_quote! {
            /// The C++ copy assignment operator.
            ///
            /// # Specification
            /// - ensures: success copy-assigns that into self without relocating either object.
            /// - provides: in-place assignment with the recorded C++ exception capability.
            /// - fails: a throwing assignment returns `CxxException` with the caught message;
            ///   destination state on failure follows its C++ assignment operator.
            /// - panics: a throwing C++ noexcept operation, or a throwing
            ///   operation without exception support, terminates.
            #[inline]
            pub(crate) fn copy_assign(self: ::core::pin::Pin<&mut Self>, that: &Self) #output {
                // SAFETY: C++ assigns in place and never relocates `self`.
                let this: *mut Self = #this;
                let result: ::core::result::Result<(), #error> = { #call };
                #finish
            }
        });
    }
    if info.is_rust_move_assign {
        let error = emit_error_type(nothrow.move_assign);
        let this = quote::quote!(::core::ptr::from_mut(unsafe {
            ::core::pin::Pin::into_inner_unchecked(self)
        }));
        let call = emit_cxx_call(
            nothrow.move_assign,
            "cxx_move_assign",
            "cxx_try_move_assign",
            &quote::quote!(this, that),
        );
        let (output, finish) = assignment_result(nothrow.move_assign, &error);
        methods.push(syn::parse_quote! {
            /// The C++ move assignment operator. The source is left
            /// moved-from in its own place, and its owner still destroys it.
            ///
            /// # Specification
            /// - ensures: success move-assigns that into self without relocating either object;
            ///   that stays pinned, moved-from, and owned by its original owner.
            /// - provides: in-place assignment with the recorded C++ exception capability.
            /// - fails: a throwing assignment returns `CxxException` with the caught message;
            ///   both objects' states on failure follow their C++ assignment operator.
            /// - panics: a throwing C++ noexcept operation, or a throwing
            ///   operation without exception support, terminates.
            #[inline]
            pub(crate) fn move_assign(
                self: ::core::pin::Pin<&mut Self>,
                that: ::core::pin::Pin<&mut Self>,
            ) #output {
                // SAFETY: C++ assigns in place and relocates neither object.
                let this: *mut Self = #this;
                // SAFETY: as above.
                let that: *mut Self = ::core::ptr::from_mut(unsafe { ::core::pin::Pin::into_inner_unchecked(that) });
                let result: ::core::result::Result<(), #error> = { #call };
                #finish
            }
        });
    }
    methods
}

/// The return type and final expression of a generated assignment.
///
/// # Specification
/// - provides: nothing and a discarded `Infallible` result when `nothrow`,
///   otherwise `-> Result<(), error>` returning the result.
#[cfg(feature = "alloc")]
fn assignment_result(
    nothrow: bool,
    error: &syn::Type,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
{
    if nothrow {
        (proc_macro2::TokenStream::new(), quote::quote! {
            match result {
                ::core::result::Result::Ok(()) => {}
                ::core::result::Result::Err(never) => match never {},
            }
        })
    }
    else {
        (
            quote::quote!(-> ::core::result::Result<(), #error>),
            quote::quote!(result),
        )
    }
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
                /// Format the C++ display representation.
                ///
                /// # Specification
                /// - ensures: writes the C++ display string to the formatter.
                /// - provides: the formatter's completion result.
                /// - fails: returns `fmt::Error` when the formatter rejects output.
                /// - panics: the formatting sink may panic; a C++ rendering or string-allocation
                ///   exception terminates at the native noexcept boundary.
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let string = self::ffi::cxx_display(self);
                    write!(f, "{string}")
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
                /// Test the C++ inequality relation.
                ///
                /// # Specification
                /// - provides: the recorded C++ inequality result for self and other.
                /// - panics: a throwing C++ inequality operation terminates at the native noexcept boundary.
                #[allow(clippy::partialeq_ne_impl)]
                #[inline]
                fn ne(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_not_equal(self, other) == ::cxx_auto::bridge::Equality::NotEqual
                }
            }
        });
        syn::parse_quote! {
            impl #generics_binder ::core::cmp::PartialEq for #ident #generics {
                /// Test the C++ equality relation.
                ///
                /// # Specification
                /// - provides: the recorded C++ equality result for self and other.
                /// - panics: a throwing C++ equality operation terminates at the native noexcept boundary.
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_equal(self, other) == ::cxx_auto::bridge::Equality::Equal
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
/// - provides: a `PartialOrd` impl whose `partial_cmp` makes one C++ three-way
///   comparison, so `lt`, `le`, `gt`, and `ge` keep their default definitions
///   and cannot disagree with it.
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
        let partial_cmp: syn::ImplItemFn = if info.is_rust_ord {
            syn::parse_quote! {
                /// Compare values using the recorded C++ total order.
                ///
                /// # Specification
                /// - provides: Some(Less), Some(Equal), or Some(Greater) according to C++.
                /// - panics: a throwing C++ comparison or equality operation terminates
                ///   at the native noexcept boundary.
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
        }
        else {
            syn::parse_quote! {
                /// Compare values using the recorded C++ partial order.
                ///
                /// # Specification
                /// - provides: Some(Less), Some(Equal), or Some(Greater) for ordered operands;
                ///   None for an unordered C++ comparison.
                /// - panics: a throwing C++ comparison or equality operation terminates
                ///   at the native noexcept boundary.
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                    let res = self::ffi::cxx_operator_three_way_comparison(self, other);
                    match res {
                        ::cxx_auto::bridge::Comparison::Less => Some(::core::cmp::Ordering::Less),
                        ::cxx_auto::bridge::Comparison::Equivalent => Some(::core::cmp::Ordering::Equal),
                        ::cxx_auto::bridge::Comparison::Greater => Some(::core::cmp::Ordering::Greater),
                        ::cxx_auto::bridge::Comparison::Unordered => None,
                    }
                }
            }
        };
        syn::parse_quote! {
            impl #generics_binder ::core::cmp::PartialOrd for #ident #generics {
                #partial_cmp
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
                /// Compare values using the recorded C++ total order.
                ///
                /// # Specification
                /// - provides: Less, Equal, or Greater according to the C++ comparison.
                /// - panics: a throwing C++ comparison or equality operation terminates
                ///   at the native noexcept boundary.
                #[inline]
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                    let res = self::ffi::cxx_operator_three_way_comparison(self, other);
                    // The recorded rust_ord invariant excludes Unordered.
                    res.cmp(&::cxx_auto::bridge::Comparison::Equivalent)
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
                /// Feed the recorded C++ hash into the Rust hasher.
                ///
                /// # Specification
                /// - ensures: writes the C++ hash as one pointer-sized word to state.
                /// - provides: the hash projection selected by the C++ type author.
                /// - panics: the caller-provided `Hasher` may panic; a throwing C++ hash
                ///   operation terminates at the native noexcept boundary.
                #[inline]
                fn hash<H>(&self, state: &mut H)
                where
                    H: ::core::hash::Hasher,
                {
                    let hash = self::ffi::cxx_hash(self);
                    state.write_usize(hash.value);
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
        /// Layout and capability witnesses for the recorded C++ type.
        #[cfg(test)]
        mod info {
            use super::*;
            /// Runtime layout witnesses.
            mod test {
                use super::*;
                /// Check alignment against the compiled C++ type record.
                ///
                /// # Specification
                /// - ensures: succeeds only when Rust and C++ alignment agree.
                /// - panics: Rust alignment differs from the recorded C++ alignment.
                #[test]
                fn cxx_abi_align() {
                    ::core::assert_eq!(::core::mem::align_of::<#ident #generics>(), #align)
                }
                /// Check size against the compiled C++ type record.
                ///
                /// # Specification
                /// - ensures: succeeds only when Rust and C++ size agree.
                /// - panics: Rust size differs from the recorded C++ size.
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
    let construction = emit_construction_bridge_fns(info, ident, generics);
    let cxx_destruct: Option<syn::ForeignItemFn> = info.is_rust_drop.then(|| {
        syn::parse_quote! {
            /// Destroy one live C++ object in place.
            ///
            /// # Specification
            /// - requires: This points to a live, uniquely owned object of the recorded type.
            /// - ensures: the noexcept C++ destructor runs once; This no longer contains a live object.
            /// - provides: release of the object's resources.
            /// - panics: a throwing C++ destructor terminates at the native noexcept boundary.
            ///
            /// # Safety
            /// - unsafe invariants: This is non-null, aligned, valid for the object, and
            ///   exclusively accessible; it must not be used as a live object after this call.
            unsafe fn cxx_destruct #generics (This: *mut #ident #generics);
        }
    });
    let cxx_operator_equal: Option<syn::ForeignItemFn> = info.is_rust_partial_eq.then(|| syn::parse_quote! {
            /// Evaluate C++ equality for two live objects.
            ///
            /// # Specification
            /// - provides: the recorded C++ equality relation for This and That.
            /// - panics: a throwing C++ equality operation terminates at the native noexcept boundary.
            fn cxx_operator_equal #generics (This: & #ident #generics, That: & #ident #generics) -> Equality;
        });
    let cxx_operator_not_equal: Option<syn::ForeignItemFn> = (info.is_rust_partial_eq && info.cxx_has_operator_not_equal).then(|| syn::parse_quote! {
            /// Evaluate C++ inequality for two live objects.
            ///
            /// # Specification
            /// - provides: the recorded C++ inequality relation for This and That.
            /// - panics: a throwing C++ inequality operation terminates at the native noexcept boundary.
            fn cxx_operator_not_equal #generics (This: & #ident #generics, That: & #ident #generics) -> Equality;
        });
    let cxx_operator_three_way_comparison: Option<syn::ForeignItemFn> = info.is_rust_partial_ord.then(|| syn::parse_quote! {
            /// Evaluate the C++ three-way comparison.
            ///
            /// # Specification
            /// - provides: less, equivalent, greater, or unordered according to C++.
            /// - panics: a throwing C++ comparison or equality operation terminates
            ///   at the native noexcept boundary.
            fn cxx_operator_three_way_comparison #generics (This: & #ident #generics, That: & #ident #generics) -> Comparison;
        });
    let cxx_hash: Option<syn::ForeignItemFn> = info.is_rust_hash.then(|| {
        syn::parse_quote! {
            /// Compute the C++ hash projection.
            ///
            /// # Specification
            /// - provides: the pointer-sized hash selected by the C++ type author.
            /// - panics: a throwing C++ hash operation terminates at the native noexcept boundary.
            fn cxx_hash #generics (This: & #ident #generics) -> HashValue;
        }
    });
    let cxx_debug: Option<syn::ForeignItemFn> = info.is_rust_debug.then(|| {
        syn::parse_quote! {
            /// Obtain the C++ debug representation.
            ///
            /// # Specification
            /// - provides: the C++ debug string converted lossily to UTF-8.
            /// - panics: a C++ rendering or string-allocation exception terminates
            ///   at the native noexcept boundary.
            fn cxx_debug #generics (This: & #ident #generics) -> String;
        }
    });
    let cxx_display: Option<syn::ForeignItemFn> = info.is_rust_display.then(|| {
        syn::parse_quote! {
            /// Obtain the C++ display representation.
            ///
            /// # Specification
            /// - provides: the C++ display string converted lossily to UTF-8.
            /// - panics: a C++ rendering or string-allocation exception terminates
            ///   at the native noexcept boundary.
            fn cxx_display #generics (This: & #ident #generics) -> String;
        }
    });
    syn::parse_quote! {
        /// CXX declarations for the recorded C++ capabilities.
        ///
        /// # Safety
        /// - unsafe invariants: the included proxy implements the declared signatures
        ///   and recorded layout. Safe shims accept live borrowed objects, do not
        ///   relocate them, and cannot unwind through Rust; construction shims catch
        ///   exceptions or call operations recorded as noexcept.
        ///   Shared enum results have the co-versioned Rust/C++ discriminants and
        ///   contain only declared variants; CXX does not validate that agreement.
        #[cxx::bridge]
        pub(crate) mod ffi {
            #![allow(clippy::needless_lifetimes)]
            // SAFETY: the module's Safety section states the foreign block's
            // layout, lifetime, and unwinding invariants. CXX rejects doc
            // attributes on the foreign block itself.
            #[namespace = #cxx_proxy_namespace]
            unsafe extern "C++" {
                include!(#cxx_proxy_include);
                /// Shared Completion representation used by every generated bridge.
                #[namespace = "cxx_auto"]
                type Completion = ::cxx_auto::bridge::Completion;
                /// Shared Equality representation used by every generated bridge.
                #[namespace = "cxx_auto"]
                type Equality = ::cxx_auto::bridge::Equality;
                /// Shared Comparison representation used by every generated bridge.
                #[namespace = "cxx_auto"]
                type Comparison = ::cxx_auto::bridge::Comparison;
                /// Shared HashValue representation used by every generated bridge.
                #[namespace = "cxx_auto"]
                type HashValue = ::cxx_auto::bridge::HashValue;

                /// The C++ type represented by the enclosing Rust storage.
                #[namespace = #cxx_namespace]
                #[cxx_name = #cxx_name]
                #[allow(unused)]
                type #ident #generics = super :: #ident #generics;
                #(#construction)*
                #cxx_destruct
                #cxx_operator_equal
                #cxx_operator_not_equal
                #cxx_operator_three_way_comparison
                #cxx_hash
                #cxx_debug
                #cxx_display
            }
        }
    }
}

/// Declare the C++ shims behind the generated initializers and assignments.
///
/// # Specification
/// - provides: for each operation C++ supports, the plain `noexcept` shim when
///   the operation cannot throw, otherwise the catching `cxx_try_` shim that
///   reports the exception through a `String`.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_construction_bridge_fns(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics: &syn::Generics,
) -> alloc::vec::Vec<syn::ForeignItemFn>
{
    let nothrow = info.cxx_nothrow;
    let this: syn::FnArg = syn::parse_quote!(This: *mut #ident #generics);
    let copied: syn::FnArg = syn::parse_quote!(that: &#ident #generics);
    let moved: syn::FnArg = syn::parse_quote!(that: *mut #ident #generics);
    let operations = [
        (
            info.is_rust_default,
            nothrow.default_new,
            "default_new",
            None,
            "This is aligned, writable, uninitialized storage for one object.",
            "constructs a default object at This",
        ),
        (
            info.is_rust_copy_new,
            nothrow.copy_new,
            "copy_new",
            Some(&copied),
            "This is aligned, writable, uninitialized storage disjoint from the live borrowed that.",
            "copy-constructs an object at This without modifying that",
        ),
        (
            info.is_rust_move_new,
            nothrow.move_new,
            "move_new",
            Some(&moved),
            "This is aligned, writable, uninitialized storage; that is a disjoint live, exclusively accessible object.",
            "move-constructs an object at This; that stays live, moved-from, and owned in its original place",
        ),
        (
            info.is_rust_copy_assign,
            nothrow.copy_assign,
            "copy_assign",
            Some(&copied),
            "This points to a live exclusively accessible object; that is a disjoint live borrowed object.",
            "copy-assigns that into This without relocating either object",
        ),
        (
            info.is_rust_move_assign,
            nothrow.move_assign,
            "move_assign",
            Some(&moved),
            "This and that are disjoint live, exclusively accessible objects.",
            "move-assigns that into This; that stays live, moved-from, and owned in its original place",
        ),
    ];
    let span = proc_macro2::Span::call_site();
    operations
        .into_iter()
        .filter(|&(supported, ..)| supported)
        .map(|(_, cannot_throw, operation, that, requires, ensures)| {
            let that = that.into_iter();
            if cannot_throw {
                let documentation = alloc::format!(
                    r"Run the C++ {operation} operation in place.

# Specification
- requires: {requires}
- ensures: success {ensures}.
- provides: the recorded C++ operation without relocating either storage place.
- panics: a C++ operation that throws terminates at the native noexcept boundary.

# Safety
- unsafe invariants: {requires} Pointers retain the recorded type's layout and
  lifetime; storage remains pinned while a non-relocatable object lives there.
"
                );
                let name = syn::Ident::new(&alloc::format!("cxx_{operation}"), span);
                syn::parse_quote! {
                    #[doc = #documentation]
                    unsafe fn #name #generics (#this #(, #that)*);
                }
            }
            else {
                let name = syn::Ident::new(&alloc::format!("cxx_try_{operation}"), span);
                let documentation = alloc::format!(
                    r"Run the catching C++ {operation} operation in place.

# Specification
- requires: {requires}
- ensures: success {ensures}; failed construction leaves no destination object
  to destroy. State after a failed assignment or move follows the C++ operation.
- provides: completion status; what contains the caught message on failure.
- fails: with exception support, catches standard and unknown C++ exceptions
  before returning to Rust.
- panics: without exception support, a throwing C++ operation terminates.

# Safety
- unsafe invariants: {requires} Pointers retain the recorded type's layout and
  lifetime; storage remains pinned while a non-relocatable object lives there.
"
                );
                syn::parse_quote! {
                    #[doc = #documentation]
                    unsafe fn #name #generics (#this #(, #that)*, what: &mut String) -> Completion;
                }
            }
        })
        .collect()
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
/// Suppress the `Send` and `Sync` auto-traits unless C++ opted into both.
///
/// # Specification
/// - provides: a `!Send + !Sync` marker field unless both `is_rust_send` and
///   `is_rust_sync` hold; `emit_impls_send_sync` then restores whichever one
///   C++ opted into.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn field_neither_send_nor_sync(info: &CxxAutoArtifactInfo) -> Option<syn::Field>
{
    let is_both = info.is_rust_send && info.is_rust_sync;
    (!is_both).then(|| {
        let name = "_neither_send_nor_sync";
        let ty = syn::parse_quote!(::core::marker::PhantomData<[*const u8; 0]>);
        emit_field(name, ty)
    })
}
/// Restore the one thread-safety trait C++ opted into when not both.
///
/// # Specification
/// - provides: an `unsafe impl Send` or `unsafe impl Sync` for each opt-in when
///   exactly one holds; nothing when neither or both hold, since the marker
///   field is then either kept whole or absent.
/// - requires: the C++ author's `rust_send` / `rust_sync` specialization is the
///   soundness claim these impls rest on.
/// - panics: malformed generated identifiers may be rejected by syn.
#[cfg(feature = "alloc")]
fn emit_impls_send_sync(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> ::alloc::vec::Vec<syn::ItemImpl>
{
    if info.is_rust_send == info.is_rust_sync {
        return ::alloc::vec![];
    }
    let item: syn::ItemImpl = if info.is_rust_send {
        syn::parse_quote! {
            /// Permit ownership transfer according to the C++ author's opt-in.
            ///
            /// # Safety
            /// - unsafe invariants: rust_send promises the value and its resources
            ///   can be transferred between threads without violating their lifetimes.
            // SAFETY: the C++ type's author specialized `cxx_auto::rust_send`.
            unsafe impl #generics_binder ::core::marker::Send for #ident #generics {}
        }
    }
    else {
        syn::parse_quote! {
            /// Permit shared access according to the C++ author's opt-in.
            ///
            /// # Safety
            /// - unsafe invariants: rust_sync promises shared references can be used
            ///   concurrently across threads without data races or lifetime violations.
            // SAFETY: the C++ type's author specialized `cxx_auto::rust_sync`.
            unsafe impl #generics_binder ::core::marker::Sync for #ident #generics {}
        }
    };
    ::alloc::vec![item]
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
