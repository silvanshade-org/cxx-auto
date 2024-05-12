use std::path::PathBuf;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type BoxResult<T> = Result<T, BoxError>;

fn main() -> BoxResult<()> {
    // Compile an empty bridge to ensure `cxx-auto` lib is always produced.
    #[allow(unused_mut)]
    let mut bridges = vec!["src/gen/empty.rs"];

    #[cfg(feature = "ctypes")]
    // Compile bindings for extra ctypes if feature is enabled
    bridges.push("src/gen/ctypes.rs");

    cxx_build::bridges(bridges)
        .flag_if_supported("-fno-rtti")
        .flag_if_supported("-std=gnu++2b")
        .flag_if_supported("-Werror")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-pedantic")
        .flag_if_supported("-Wno-ambiguous-reversed-operator")
        .flag_if_supported("-Wno-deprecated-anon-enum-enum-conversion")
        .flag_if_supported("-Wno-deprecated-builtins")
        .flag_if_supported("-Wno-dollar-in-identifier-extension")
        .flag_if_supported("-Wno-nested-anon-types")
        .flag_if_supported("-Wno-unused-parameter")
        .try_compile("cxx-auto")?;

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let cxxbridge = out_dir.join("cxxbridge");

    println!("cargo::metadata=cxxbridge={}", cxxbridge.display());
    println!("cargo::rerun-if-changed=cxx");
    println!("cargo::rerun-if-changed=src/gen");

    Ok(())
}
