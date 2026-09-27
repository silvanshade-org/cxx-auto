use std::path::Path;
use std::path::PathBuf;

/// Compile the cxx-auto module and the fixture's type record, generate its
/// binding, and compile the generated CXX bridge, all in one build script.
///
/// # Specification
/// - provides: a C++26 archive with the cxx-auto module, the generated
///   comparison methods, and the fixture constructor, and the generated modules
///   under `OUT_DIR`.
/// - fails: reports a C++ compiler, generation, or environment error.
/// - panics: none.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").ok_or("missing output directory")?);
    let compiler = std::env::var_os("CXX").unwrap_or_else(|| "clang++".into());
    let flags = [
        "-std=c++2c",
        "-fno-exceptions",
        "-fno-rtti",
        "-Wall",
        "-Wextra",
        "-Werror",
    ];

    // An empty bridge set configures the include paths of cxx and cxx-auto.
    let mut base = cxx_build::bridges(Vec::<PathBuf>::new());
    base.include("include").compiler(&compiler);
    for flag in flags {
        base.flag(flag);
    }
    // The cxx_auto module comes first: every later unit imports it through
    // `cxx-auto.hxx`.
    let modules = cxx_auto::compile_modules(&base, [cxx_auto::ModuleUnit::cxx_auto()], &out_dir)?;

    let mut records = base.clone();
    modules.configure(&mut records).file("src/export.cxx");
    let objects = records.try_compile_intermediates()?;
    let generated = cxx_auto::generate(&objects, &out_dir)?;

    let bridges = generated
        .iter()
        .map(PathBuf::as_path)
        .chain([Path::new("src/lib.rs")]);
    let mut build = cxx_build::bridges(bridges);
    build.include("include").compiler(&compiler);
    for flag in flags {
        build.flag(flag);
    }
    modules.configure(&mut build).objects(modules.objects());
    build.try_compile("dynamic_binding_fixture")?;

    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=include");
    Ok(())
}
