use std::path::Path;
use std::path::PathBuf;

/// Compile the cxx-auto module and the fixture's type record, generate its
/// binding, and compile the generated CXX bridge, all in one build script.
///
/// # Specification
/// - provides: a C++26 archive with the cxx-auto module, the generated
///   comparison methods and in-place construction shims, and the fixture
///   helpers, and the generated modules under `OUT_DIR`. Exceptions stay
///   enabled so the generated catching shims can report them.
/// - fails: reports a C++ compiler, generation, or environment error.
/// - panics: none.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").ok_or("missing output directory")?);
    // cc reads CXX itself, including a leading wrapper such as ccache; Clang is
    // only the fallback when CXX is unset.
    let fallback = std::env::var_os("CXX").is_none().then_some("clang++");
    let flags = ["-std=c++2c", "-fno-rtti", "-Wall", "-Wextra", "-Werror"];

    // An empty bridge set configures the include paths of cxx and cxx-auto.
    let mut base = cxx_build::bridges(Vec::<PathBuf>::new());
    base.include("include");
    if let Some(compiler) = fallback {
        base.compiler(compiler);
    }
    for flag in flags {
        base.flag(flag);
    }
    // cpp-deps orders the cxx_auto module before the record source that
    // imports it through `cxx-auto.hxx`.
    let modules = cxx_auto::compile_modules(&base, ["src/export.cxx"], &out_dir)?;
    let generated = cxx_auto::generate(modules.objects(), &out_dir)?;

    let bridges = generated
        .iter()
        .map(PathBuf::as_path)
        .chain([Path::new("src/lib.rs")]);
    let mut build = cxx_build::bridges(bridges);
    build.include("include");
    if let Some(compiler) = fallback {
        build.compiler(compiler);
    }
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
