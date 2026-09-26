/// Compile the final generated CXX bridge and the explicit C++ value
/// constructor.
///
/// # Specification
/// - provides: a C++26 archive with the generated comparison methods and
///   fixture constructor.
/// - fails: reports a C++ compiler or environment error.
/// - panics: none.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let compiler = std::env::var_os("CXX").unwrap_or_else(|| "clang++".into());
    cxx_build::bridges(["src/lib.rs", "src/auto/number.rs"])
        .include("include")
        .compiler(&compiler)
        .flag("-std=c++2c")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Werror")
        .try_compile("dynamic_binding_fixture")?;
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-changed=src/auto");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/number.hxx");
    Ok(())
}
