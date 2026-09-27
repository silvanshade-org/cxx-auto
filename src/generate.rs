use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use std::path::Path;
use std::path::PathBuf;

use crate::BoxResult;
use crate::cxx_auto_artifact_info::CxxAutoArtifactInfo;

/// Generate the Rust bindings for every type record in `objects`.
///
/// Call this from a build script after compiling the translation units that
/// invoke `CXX_AUTO_EXPORT`, for example with
/// `cc::Build::compile_intermediates`. Nothing in the objects is linked or
/// executed: each record is read as data.
///
/// # Specification
/// - requires: each path names a compiled object for a little-endian target;
///   together they define at least one record, and no two records share a
///   `rust_path`.
/// - ensures: `out_dir/auto.rs` declares the generated module tree, one file
///   per type lies under `out_dir/auto`, and any earlier contents of
///   `out_dir/auto` are replaced.
/// - provides: the generated type files, in `rust_path` order, which the caller
///   passes to `cxx_build::bridges`.
/// - fails: an object cannot be read, holds a malformed record, or duplicates a
///   `rust_path`; no records are found; or formatting or writing fails.
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: a crate that compiles its records and generates its bindings
///   in one build script must compare C++ values through the generated traits.
/// - witness: `tests/dynamic_binding.rs`
///   (`loads_generated_comparison_binding`).
///
/// # Errors
///
/// Returns an error naming the object and record, or the output path, that
/// failed.
#[inline]
pub fn generate<I, P>(
    objects: I,
    out_dir: &Path,
) -> BoxResult<Vec<PathBuf>>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut infos = Vec::new();
    for object in objects {
        infos.extend(crate::record::read_object(object.as_ref())?);
    }
    if infos.is_empty() {
        return Err("no cxx-auto type records found; is `CXX_AUTO_EXPORT` compiled in?".into());
    }
    infos.sort_by(|lhs, rhs| lhs.path_components.cmp(&rhs.path_components));
    if let Some(pair) = infos.windows(2).find(|pair| {
        pair.first().map(|info| &info.path_components)
            == pair.get(1).map(|info| &info.path_components)
    }) {
        let path = pair
            .first()
            .map(|info| info.path_components.join("::"))
            .unwrap_or_default();
        return Err(format!("two type records share the Rust path `{path}`").into());
    }

    let children = module_children(&infos);
    let auto_root = out_dir.join("auto");
    if auto_root.exists() {
        std::fs::remove_dir_all(&auto_root)?;
    }
    let type_paths = infos
        .iter()
        .map(|info| info.path_components.clone())
        .collect::<BTreeSet<Vec<String>>>();
    for (module, descendants) in &children {
        if !type_paths.contains(module) {
            let descendants = descendants.iter().cloned().collect::<Vec<String>>();
            CxxAutoArtifactInfo::write_module_for_dir(&auto_root, module, &descendants)?;
        }
    }
    let mut files = Vec::with_capacity(infos.len());
    for mut info in infos {
        info.path_descendants = children
            .get(&info.path_components)
            .map(|descendants| descendants.iter().cloned().collect())
            .unwrap_or_default();
        info.write_module_for_file(&auto_root)?;
        files.push(
            auto_root
                .join(PathBuf::from_iter(&info.path_components))
                .with_extension("rs"),
        );
    }
    Ok(files)
}

/// Map every module in the generated tree to its immediate children.
///
/// # Specification
/// - provides: an entry for the root (the empty path) and for every proper
///   prefix of a type's path, each with the next path component of every type
///   below it; a type with nothing below it has no entry.
/// - panics: none.
fn module_children(infos: &[CxxAutoArtifactInfo]) -> BTreeMap<Vec<String>, BTreeSet<String>>
{
    let mut children = BTreeMap::<Vec<String>, BTreeSet<String>>::new();
    for info in infos {
        let mut parent = Vec::new();
        for component in &info.path_components {
            children
                .entry(parent.clone())
                .or_default()
                .insert(component.clone());
            parent.push(component.clone());
        }
    }
    children
}
