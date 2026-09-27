/// Compile the bridge and export its C++ header to downstream builds.
///
/// # Specification
/// - requires: CXX, when set, names a compiler accepting C++26 without
///   exceptions, optionally behind a wrapper such as `ccache clang++`; cc
///   parses it. Unset, the compiler is `clang++`.
/// - ensures: the bridge is compiled with C++26 and exceptions disabled.
/// - fails: returns the C++ compiler's error on a failed build.
/// - panics: bridge generation may reject malformed Rust input.
fn main() -> Result<(), Box<dyn core::error::Error>>
{
    // An empty bridge exports cxx/include/**/*.hxx to dependencies.
    let mut build = cxx_build::bridge("src/gen/ctypes.rs");
    // cc reads CXX itself, including a wrapper prefix; only the fallback is
    // chosen here.
    if std::env::var_os("CXX").is_none() {
        build.compiler("clang++");
    }
    build
        .flag("-std=c++2c")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-Werror")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-pedantic")
        .try_compile("cxx-auto")?;

    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-changed=cxx");
    println!("cargo:rerun-if-changed=src/gen/ctypes.rs");
    Ok(())
}
