type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

fn main() -> BoxResult<()> {
    let mut build = cxx_build::bridges(&[] as &[&str]);

    let compiler = build.try_get_compiler()?;

    if compiler.is_like_clang() || compiler.is_like_gnu() {
        build.std("gnu++23");
    } else if compiler.is_like_msvc() {
        build.std("c++latest");
    } else {
        return Err(BoxError::from("Unrecognized C++ compiler"));
    }

    build
        .warnings_into_errors(true)
        .warnings(true)
        .extra_warnings(true)
        .flag_if_supported("-Wpedantic")
        .flag_if_supported("-Wno-dollar-in-identifier-extension");

    build.files(["cxx/lib/cxx-auto.cc"]);

    build.try_compile("cxx-auto")?;

    println!("cargo::rerun-if-changed=cxx");

    Ok(())
}
