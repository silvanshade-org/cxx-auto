use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write as _;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use crate::BoxResult;

/// One C++ named-module interface unit: the module's name and its source.
///
/// # Specification
/// - provides: an owned name and source path for `compile_modules`.
/// - panics: none.
#[derive(Clone, Debug)]
pub struct ModuleUnit
{
    /// The name after `export module` in the source.
    name: String,
    /// Path of the interface unit's source file.
    source: PathBuf,
}

impl ModuleUnit
{
    /// Describe a module interface unit.
    ///
    /// # Specification
    /// - requires: `name` is the name the source declares with `export module`.
    /// - provides: the unit, unchecked until `compile_modules` compiles it.
    /// - panics: none.
    #[inline]
    #[must_use]
    pub fn new<N, S>(
        name: N,
        source: S,
    ) -> Self
    where
        N: Into<String>,
        S: Into<PathBuf>,
    {
        Self {
            name: name.into(),
            source: source.into(),
        }
    }

    /// The `cxx_auto` module that `cxx-auto.hxx` imports.
    ///
    /// # Specification
    /// - provides: the unit for `cxx/module/cxx_auto.cppm` in this crate's
    ///   source, which every build using `cxx-auto.hxx` compiles first.
    /// - panics: none.
    #[inline]
    #[must_use]
    pub fn cxx_auto() -> Self
    {
        Self::new(
            "cxx_auto",
            concat!(env!("CARGO_MANIFEST_DIR"), "/cxx/module/cxx_auto.cppm"),
        )
    }
}

/// C++ named modules compiled for one build, ready to be imported.
///
/// # Specification
/// - provides: the importer flags and the module objects to link.
/// - panics: none.
#[derive(Clone, Debug)]
pub struct Modules
{
    /// Flags that make every compiled module importable.
    flags: Vec<OsString>,
    /// Object files holding the modules' definitions.
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
    /// - ensures: translation units compiled by `build` can `import` each
    ///   module, including through `cxx-auto.hxx`.
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

    /// The object files holding the modules' definitions.
    ///
    /// # Specification
    /// - provides: one object per unit, in compile order, which the final
    ///   library links, for example with `cc::Build::objects`.
    /// - panics: none.
    #[inline]
    #[must_use]
    pub fn objects(&self) -> &[PathBuf]
    {
        &self.objects
    }
}

/// How a compiler family consumes and produces compiled module interfaces.
enum Scheme
{
    /// Clang: one `.pcm` per module, named to each importer.
    Clang,
    /// GCC: one module mapper file naming every module's `.gcm`.
    Gnu
    {
        /// Path of the mapper file.
        mapper: PathBuf,
    },
}

/// Compile C++ module interface units in the given order.
///
/// Until cxx-auto discovers module dependencies itself, the caller lists the
/// units so that each comes after every module it imports; `cxx_auto` itself,
/// `ModuleUnit::cxx_auto`, therefore comes first.
///
/// # Specification
/// - requires: `build` carries the compiler, language standard, include paths,
///   and flags of the translation units that will import the modules; `units`
///   are in dependency order with distinct names; `out_dir` exists.
/// - ensures: each unit is compiled once, in order, with every earlier module
///   importable, and its compiled interface lies under `out_dir`.
/// - provides: the importer flags and module objects for `Modules::configure`
///   and `Modules::objects`.
/// - fails: the compiler is neither Clang nor GCC, a name is empty or repeats,
///   the GCC mapper cannot be written, or a unit fails to compile, for example
///   because it imports a module listed after it.
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: a crate that compiles the cxx-auto module first must be able
///   to import it through `cxx-auto.hxx` in both its record source and its
///   generated bridges, under Clang and GCC.
/// - witness: `tests/dynamic_binding.rs`
///   (`loads_generated_comparison_binding`), run with each compiler.
///
/// # Errors
///
/// Returns an error naming the offending unit or the compiler failure.
#[inline]
pub fn compile_modules<I>(
    build: &cc::Build,
    units: I,
    out_dir: &Path,
) -> BoxResult<Modules>
where
    I: IntoIterator<Item = ModuleUnit>,
{
    let units = units.into_iter().collect::<Vec<ModuleUnit>>();
    let mut names = BTreeSet::new();
    for unit in &units {
        if unit.name.is_empty() {
            return Err(format!(
                "module source `{}` has an empty name",
                unit.source.display()
            )
            .into());
        }
        if !names.insert(unit.name.as_str()) {
            return Err(format!("module `{}` is listed twice", unit.name).into());
        }
    }

    let compiler = build.try_get_compiler()?;
    let scheme = if compiler.is_like_clang() {
        Scheme::Clang
    }
    else if compiler.is_like_gnu() {
        let mapper = out_dir.join("cxx-auto-modules.map");
        let mut entries = String::new();
        for unit in &units {
            let gcm = out_dir.join(format!("{}.gcm", unit.name));
            writeln!(entries, "{} {}", unit.name, gcm.display())?;
        }
        std::fs::write(&mapper, entries)
            .map_err(|error| format!("{}: {error}", mapper.display()))?;
        Scheme::Gnu { mapper }
    }
    else {
        return Err(format!(
            "`{}` is neither Clang nor GCC; C++ modules are unsupported with it",
            compiler.path().display()
        )
        .into());
    };

    let mut importer_flags = match scheme {
        | Scheme::Clang => Vec::new(),
        | Scheme::Gnu { ref mapper } => {
            let mut mapper_flag = OsString::from("-fmodule-mapper=");
            mapper_flag.push(mapper);
            alloc::vec![OsString::from("-fmodules"), mapper_flag]
        },
    };
    let mut objects = Vec::with_capacity(units.len());
    for unit in &units {
        let mut unit_build = build.clone();
        for flag in &importer_flags {
            unit_build.flag(flag);
        }
        match scheme {
            | Scheme::Clang => {
                let mut output = OsString::from("-fmodule-output=");
                output.push(out_dir.join(format!("{}.pcm", unit.name)));
                unit_build.flag(output);
            },
            | Scheme::Gnu { .. } => {
                // GCC does not recognise `.cppm`; name the language explicitly.
                unit_build.flag("-x").flag("c++");
            },
        }
        unit_build.file(&unit.source);
        let unit_objects = unit_build
            .try_compile_intermediates()
            .map_err(|error| format!("module `{}`: {error}", unit.name))?;
        objects.extend(unit_objects);
        if matches!(scheme, Scheme::Clang) {
            let mut import = OsString::from(format!("-fmodule-file={}=", unit.name));
            import.push(out_dir.join(format!("{}.pcm", unit.name)));
            importer_flags.push(import);
        }
    }
    Ok(Modules {
        flags: importer_flags,
        objects,
    })
}
