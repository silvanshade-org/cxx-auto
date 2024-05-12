use alloc::collections::BTreeSet;
use proc_macro2::Span;
use std::path::{Path, PathBuf};
use syn::punctuated::Punctuated;

mod type_elab;
mod type_spec;

pub(crate) fn item_fn_emit_mod_file_for_type() -> syn::ItemFn {
    syn::parse_quote! {
        #[allow(clippy::missing_errors_doc)]
        pub(crate) fn emit_mod_file(auto_out_dir_root: &::std::path::Path) -> ::cxx_auto::BoxResult<()> {
            ::cxx_auto::emit::mod_file_for_type(&self::elab_type_from_spec(), auto_out_dir_root)
        }
    }
}

pub(crate) fn item_mods_for_path_descendants(
    project_dir: &Path,
    out_dir: &Path,
    path: &Path,
    skip_paths: &mut BTreeSet<PathBuf>,
    path_descendants: &mut BTreeSet<String>,
    items: &mut Vec<syn::ItemMod>,
) -> crate::BoxResult<()> {
    skip_paths.insert(path.to_path_buf());
    find_immediate_path_descendants(path, path_descendants)?;
    let span = Span::call_site();
    for descendant in &*path_descendants {
        let mod_name = syn::Ident::new(descendant, span);
        let descendant_path = path.join(descendant).with_extension("rs");
        let mod_suffix = descendant_path.strip_prefix(project_dir)?;
        let mod_path = out_dir.join(mod_suffix);
        let mod_path_str = mod_path.to_string_lossy();
        items.push(syn::parse_quote! {
            #[path = #mod_path_str]
            pub mod #mod_name;
        });
    }
    Ok(())
}

pub(crate) fn item_fn_emit_mod_file_for_dir(
    path_components: &Vec<String>,
    path_descendants: &BTreeSet<String>,
) -> syn::ItemFn {
    syn::parse_quote! {
        #[allow(clippy::missing_errors_doc)]
        pub(crate) fn emit_mod_file(out_dir: &::std::path::Path) -> ::cxx_auto::BoxResult<()> {
            let path_components = &[#(#path_components),*];
            let path_descendants = &[#(#path_descendants),*];
            ::cxx_auto::emit::mod_file_for_dir(out_dir, path_components, path_descendants)
        }
    }
}

pub(crate) fn item_fn_generate<'a>(walked_path_components: impl Iterator<Item = &'a Vec<String>>) -> syn::ItemFn {
    let span = Span::call_site();
    let items_stmt_expr_call_emit_mod_file = walked_path_components.map(|path_components| -> syn::Stmt {
        let path = syn::Path {
            leading_colon: None,
            segments: core::iter::once(&String::from("self"))
                .chain(path_components.iter())
                .map(|component| syn::PathSegment::from(syn::Ident::new(component, span)))
                .collect(),
        };
        syn::parse_quote!(#path::emit_mod_file(auto_out_dir_root)?;)
    });
    syn::parse_quote! {
        #[allow(clippy::missing_errors_doc)]
        pub fn generate(out_dir: &::std::path::Path) -> ::cxx_auto::BoxResult<()> {
            let auto_out_dir_root = &out_dir.join("src/auto");
            #(#items_stmt_expr_call_emit_mod_file)*
            Ok(())
        }
    }
}

fn find_immediate_path_descendants(path: &Path, path_descendants: &mut BTreeSet<String>) -> crate::BoxResult<()> {
    for result in walkdir::WalkDir::new(path).min_depth(1).max_depth(1) {
        let entry = result?;
        if let Some(file_stem) = entry
            .path()
            .file_stem()
            .map(|s| {
                s.to_os_string()
                    .into_string()
                    .map_err(|err| format!("Failed to convert to String: {err:?}"))
            })
            .transpose()?
        {
            path_descendants.insert(file_stem);
        }
    }
    Ok(())
}

fn field(name: &str, ty: syn::Type) -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new(name, Span::call_site())),
        colon_token: Some(syn::Token![:](Span::call_site())),
        ty,
    }
}

fn extract_ref_types_from_generics(generics: &syn::Generics) -> Punctuated<syn::Type, syn::Token![,]> {
    generics
        .params
        .iter()
        .filter_map(extract_ref_type_from_generic)
        .collect::<Punctuated<syn::Type, syn::Token![,]>>()
}

fn extract_ref_type_from_generic(generic_param: &syn::GenericParam) -> Option<syn::Type> {
    let syn::GenericParam::Lifetime(ref lifetime_param) = *generic_param else {
        return None;
    };
    let lifetime = &lifetime_param.lifetime;
    Some(syn::parse_quote!(&#lifetime ()))
}

fn field_memory_layout(size: &proc_macro2::Literal) -> syn::Field {
    let name = "_layout";
    let ty = syn::parse_quote!([u8; #size]);
    field(name, ty)
}

fn field_neither_send_nor_sync(info: &crate::TypeElab) -> Option<syn::Field> {
    (!info.is_rust_send && !info.is_rust_sync).then(|| {
        let name = "_neither_send_nor_sync";
        let ty = syn::parse_quote!(::core::marker::PhantomData<[*const u8; 0]>);
        field(name, ty)
    })
}

fn field_phantom_pinned(info: &crate::TypeElab) -> Option<syn::Field> {
    (!info.is_rust_unpin).then(|| {
        let name = "_pinned";
        let ty = syn::parse_quote!(::core::marker::PhantomPinned);
        field(name, ty)
    })
}

fn field_phantom_data_lifetimes(generics: &syn::Generics) -> Option<syn::Field> {
    let ref_types = extract_ref_types_from_generics(generics);
    (!ref_types.is_empty()).then(|| {
        let name = "_lifetimes";
        let ty = syn::parse_quote!(::core::marker::PhantomData<(#ref_types,)>);
        field(name, ty)
    })
}
