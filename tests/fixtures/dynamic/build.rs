use std::path::PathBuf;

/// Generate the fixture modules and compile their C++26 CXX bridges.
///
/// # Specification
/// - provides: a native archive implementing the value constructor, capability
///   probes, and generated comparison methods.
/// - fails: reports generation, compilation, or environment errors.
/// - panics: none.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let project =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").ok_or("missing manifest directory")?);
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").ok_or("missing output directory")?);
    cxx_auto::process_artifacts(&out_dir, &project.join("cfg/auto"))?;

    let probe = out_dir.join("src/auto/number.rs");
    let compiler = std::env::var_os("CXX").unwrap_or_else(|| "clang++".into());
    cxx_build::bridge(probe)
        .include(project.join("include"))
        .compiler(&compiler)
        .flag("-std=c++2c")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Werror")
        .try_compile("dynamic_binding_fixture")?;
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-changed=cfg/auto/number.json");
    println!("cargo:rerun-if-changed=include/number.hxx");
    println!("cargo:rerun-if-changed=include/probe.hxx");
    Ok(())
}
