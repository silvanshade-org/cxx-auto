use alloc::collections::BTreeSet;
use proc_macro2::Span;
use quote::ToTokens;
use rust_format::Formatter;
use std::path::{Path, PathBuf};

pub(crate) fn auto_module(project_dir: &Path, out_dir: &Path, cfg_dir: &Path) -> crate::BoxResult<()> {
    let cfg_dir_walker = walkdir::WalkDir::new(cfg_dir).min_depth(1);
    let self_named_mod_file_skip_paths = BTreeSet::new();
    let mut walked_path_components = Vec::new();

    auto_sub_module_loop(
        project_dir,
        out_dir,
        cfg_dir_walker.into_iter(),
        self_named_mod_file_skip_paths,
        &mut walked_path_components,
    )?;

    walked_path_components.sort();

    let pretty = {
        let mut path_descendants = BTreeSet::new();
        let mut item_mods = vec![];
        crate::codegen::item_mods_for_path_descendants(
            project_dir,
            out_dir,
            cfg_dir,
            &mut BTreeSet::new(),
            &mut path_descendants,
            &mut item_mods,
        )?;
        let item_fn_emit_mod_file = crate::codegen::item_fn_emit_mod_file_for_dir(&vec![], &path_descendants);
        let item_fn_generate = {
            let root = vec![];
            let path_components_from_root = core::iter::once(&root).chain(walked_path_components.iter());
            crate::codegen::item_fn_generate(path_components_from_root)
        };
        let syntax: syn::File = syn::parse_quote! {
            #(#item_mods)*
            #item_fn_emit_mod_file
            #item_fn_generate
        };
        let tokens = syntax.to_token_stream();
        rust_format::RustFmt::default().format_tokens(tokens)?
    };
    std::fs::write(out_dir.join("auto.rs"), pretty)?;

    Ok(())
}

///
/// Arguments:
/// * `self_named_mod_file_skip_paths` - Paths of "self-named modules" with filenames `./foo.rs`
///   adjacent to a directory `./foo/.` of the same name. When processing the auto directory
///   hierarchy, the "self-name module" file entries must be processed in the same loop iteration as
///   their similarly named directories. Once processed, the file entries are added to the skip
///   paths so they are not (re-)processed again individually.
fn auto_sub_module_loop(
    project_dir: &Path,
    out_dir: &Path,
    mut cfg_dir_walker: impl Iterator<Item = walkdir::Result<walkdir::DirEntry>>,
    mut self_named_mod_file_skip_paths: BTreeSet<PathBuf>,
    walked_path_file_components: &mut Vec<Vec<String>>,
) -> crate::BoxResult<()> {
    while let Some(entry) = cfg_dir_walker.next().transpose()? {
        let path_entry = entry.path();

        if !self_named_mod_file_skip_paths.contains(path_entry) {
            let path_components = relativized_components_from_path(path_entry)?;
            let mut path_descendants = BTreeSet::new();
            let mut item_mods = vec![];

            // If the entry is a dir, gather its sub modules so we can include them in the module codegen.
            if path_entry.is_dir() {
                crate::codegen::item_mods_for_path_descendants(
                    project_dir,
                    out_dir,
                    path_entry,
                    &mut self_named_mod_file_skip_paths,
                    &mut path_descendants,
                    &mut item_mods,
                )?;
            }

            let item_mod_auto_type_abi;
            let item_fn_elab_type_from_spec;
            let item_fn_emit_mod;

            if let Some(ref file_path) = Some(path_entry.to_path_buf())
                .filter(|p| p.is_file())
                .or_else(|| Some(path_entry.with_extension("json")).filter(|p| p.is_file()))
            {
                // If the entry is a file, parse the type spec include the elaboration function in the module codegen.
                let text = std::fs::read_to_string(file_path)?;
                let spec = serde_json::from_str::<crate::TypeSpec>(&text)?;

                item_mod_auto_type_abi = Some(spec.codegen_item_mod_auto_type_abi());
                item_fn_elab_type_from_spec =
                    Some(spec.codegen_item_fn_elab_type_from_spec(path_components.iter(), path_descendants.iter()));
                item_fn_emit_mod = crate::codegen::item_fn_emit_mod_file_for_type();

                // Add the file path to the skip paths so we don't reprocess next iteration.
                self_named_mod_file_skip_paths.insert(file_path.clone());
            } else {
                // Otherwise, don't include type spec parsing or the elaboration function and only add sub module declarations.
                item_mod_auto_type_abi = None;
                item_fn_elab_type_from_spec = None;
                item_fn_emit_mod = crate::codegen::item_fn_emit_mod_file_for_dir(&path_components, &path_descendants);
            }

            let auto_sub_mod_dir_path = out_dir.join(
                core::iter::once(&String::from("auto"))
                    .chain(&path_components)
                    .collect::<PathBuf>(),
            );

            if let Some(parent) = auto_sub_mod_dir_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let auto_sub_mod_file_path = auto_sub_mod_dir_path.with_extension("rs");
            let syntax: syn::File = syn::parse_quote! {
                #(#item_mods)*
                #item_mod_auto_type_abi
                #item_fn_elab_type_from_spec
                #item_fn_emit_mod
            };
            let tokens = syntax.to_token_stream();
            let pretty = rust_format::RustFmt::default().format_tokens(tokens)?;
            std::fs::write(auto_sub_mod_file_path.with_extension("rs"), pretty)?;

            walked_path_file_components.push(path_components);
        }
    }
    Ok(())
}

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - failure to create the output parent directory for the generated module
/// - failure to run `rustfmt` on the generated module
/// - failure to write the generated module to disk
pub fn mod_file_for_dir(
    auto_out_dir_root: &Path,
    path_components: &[&str],
    path_descendants: &[&str],
) -> crate::BoxResult<()> {
    let auto_out_dir = auto_out_dir_root.join(PathBuf::from_iter(path_components));
    if let Some(parent) = auto_out_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file: syn::File = {
        let span = Span::call_site();
        let items = path_descendants
            .iter()
            .map(|descendant| {
                let sub_mod_file_path = auto_out_dir.join(descendant).with_extension("rs");
                let sub_mod_file_path_str = sub_mod_file_path.to_string_lossy();
                let sub_mod_name = syn::Ident::new(descendant, span);
                syn::parse_quote! {
                    #[path = #sub_mod_file_path_str]
                    pub mod #sub_mod_name;
                }
            })
            .collect::<Vec<syn::Item>>();
        syn::File {
            shebang: None,
            attrs: vec![],
            items,
        }
    };
    let mod_file_path = auto_out_dir.with_extension("rs");
    let tokens = file.to_token_stream();
    let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
    std::fs::write(mod_file_path, contents)?;
    Ok(())
}

/// # Errors
///
/// Will return `Err` under the following circumstances:
/// - failure to create the output parent directory for the generated module
/// - failure to run `rustfmt` on the generated module
/// - failure to write the generated module to disk
pub fn mod_file_for_type(elab: &crate::TypeElab, auto_out_dir_root: &Path) -> crate::BoxResult<()> {
    let auto_out_dir = auto_out_dir_root.join(PathBuf::from_iter(&elab.path_components));
    if let Some(parent) = auto_out_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let path = auto_out_dir.with_extension("rs");
    let file = elab.codegen_file_mod_auto_type(&auto_out_dir);
    let tokens = file.to_token_stream();
    let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
    std::fs::write(path, contents)?;
    Ok(())
}

fn relativized_components_from_path(path: &Path) -> crate::BoxResult<Vec<String>> {
    let path_components = path
        .with_extension("")
        .components()
        .skip_while(|component| component.as_os_str() != "auto")
        .skip(1)
        .map(|component| {
            component
                .as_os_str()
                .to_os_string()
                .into_string()
                .map_err(|err| format!("Failed to convert to String: {err:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(path_components)
}
