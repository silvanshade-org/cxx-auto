use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write as _;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use crate::BoxResult;

/// C++ units compiled for one build: the cxx-auto module, the caller's
/// modules, and the caller's sources that import them.
///
/// # Specification
/// - provides: the importer flags and the objects to link.
/// - panics: none.
#[derive(Clone, Debug)]
pub struct Modules
{
    /// Flags that make every compiled module importable.
    flags: Vec<OsString>,
    /// Object files of every compiled unit.
    objects: Vec<PathBuf>,
}

impl Modules
{
    /// Let `build` import every compiled module.
    ///
    /// # Specification
    /// - requires: `build` uses the compiler and language flags the modules
    ///   were compiled with, since a compiled module interface is valid only
    ///   for matching flags.
    /// - ensures: translation units compiled by `build`, such as generated
    ///   bridges, can `import` each module, including through `cxx-auto.hxx`.
    /// - panics: none.
    #[inline]
    pub fn configure<'build>(
        &self,
        build: &'build mut cc::Build,
    ) -> &'build mut cc::Build
    {
        for flag in &self.flags {
            build.flag(flag);
        }
        build
    }

    /// The object files of every compiled unit.
    ///
    /// # Specification
    /// - provides: one object per unit, which `crate::generate` reads for type
    ///   records and the final library links, for example with
    ///   `cc::Build::objects`.
    /// - panics: none.
    #[inline]
    #[must_use]
    pub fn objects(&self) -> &[PathBuf]
    {
        &self.objects
    }
}

/// Compile the cxx-auto module together with the caller's C++ sources.
///
/// `cpp-deps` scans every unit for the modules it provides and imports, then
/// compiles them in dependency order, so `sources` may list module interfaces,
/// partitions, and importers such as `CXX_AUTO_EXPORT` sources in any order.
///
/// # Specification
/// - requires: `build` carries the compiler, language standard, include paths,
///   and flags that every unit and every later importer share; the compiler is
///   Clang or GCC.
/// - ensures: every unit compiles after the modules it imports; compiled
///   interfaces, objects, and dependency files lie under `out_dir`.
/// - provides: the importer flags and objects for `Modules::configure` and
///   `Modules::objects`.
/// - fails: the compiler is neither Clang nor GCC, a source is missing, the
///   scan or a compile fails, or the imports are missing, duplicated, or
///   cyclic; the error names the unit or module.
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: a crate whose record source imports cxx-auto through
///   `cxx-auto.hxx`, and whose generated bridges import it too, must build and
///   run under Clang and GCC without listing an order.
/// - witness: `tests/dynamic_binding.rs`
///   (`loads_generated_comparison_binding`), run with each compiler.
///
/// # Errors
///
/// Returns an error naming the unsupported compiler or the `cpp-deps` failure.
#[inline]
pub fn compile_modules<I, P>(
    build: &cc::Build,
    sources: I,
    out_dir: &Path,
) -> BoxResult<Modules>
where
    I: IntoIterator<Item = P>,
    P: Into<PathBuf>,
{
    let compiler = build.try_get_compiler()?;
    let is_clang = compiler.is_like_clang();
    if !is_clang && !compiler.is_like_gnu() {
        return Err(format!(
            "`{}` is neither Clang nor GCC; C++ modules are unsupported with it",
            compiler.path().display()
        )
        .into());
    }

    let module_dir = out_dir.join("cxx-auto-modules");
    let mut modules = cpp_deps::ModuleBuild::new(build.clone());
    modules.out_dir(&module_dir).source(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/cxx/module/cxx_auto.cppm"
    ));
    for source in sources {
        modules.source(source);
    }
    let output = modules.compile()?;

    let flags = if is_clang {
        output
            .interfaces
            .iter()
            .map(|(name, path)| {
                let mut flag = OsString::from(format!("-fmodule-file={}=", name.as_ref()));
                flag.push(path);
                flag
            })
            .collect()
    }
    else {
        let mapper = module_dir.join("cxx-auto-importers.map");
        let mut entries = String::new();
        for (name, path) in &output.interfaces {
            writeln!(entries, "{} {}", name.as_ref(), path.display())?;
        }
        std::fs::write(&mapper, entries)
            .map_err(|error| format!("{}: {error}", mapper.display()))?;
        let mut mapper_flag = OsString::from("-fmodule-mapper=");
        mapper_flag.push(mapper);
        alloc::vec![OsString::from("-fmodules"), mapper_flag]
    };
    Ok(Modules {
        flags,
        objects: output.objects,
    })
}
