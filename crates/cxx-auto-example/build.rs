use camino::Utf8Path;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

fn main() -> BoxResult<()> {
    let mut build = cxx_build::bridges(["src/gen/foo.rs"]);

    build.files(["cxx/lib/foo.cc"]).includes(["cxx/include"]);

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

    let objects = build.try_compile_intermediates()?;

    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = Utf8Path::new(&out_dir);

    cxx_auto_build::generate(&build, out_dir, objects)?;

    println!("cargo::rerun-if-changed=cxx");

    Ok(())
}
