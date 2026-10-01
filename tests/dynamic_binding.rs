/// End-to-end shared-library exercise for the generated CXX binding.
#[cfg(test)]
mod tests
{
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use std::time::SystemTime;

    #[test]
    fn loads_generated_comparison_binding() -> Result<(), Box<dyn core::error::Error>>
    {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("cxx-auto-binding-{}-{nonce}", std::process::id()));
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dynamic");
        let binding = root.join("binding");
        let target_dir = root.join("target");
        for directory in [binding.join("include"), binding.join("src")] {
            fs::create_dir_all(directory)?;
        }
        for (source, destination) in [
            ("number.hxx", binding.join("include/number.hxx")),
            ("objects.hxx", binding.join("include/objects.hxx")),
            ("proxy.hxx", binding.join("include/proxy.hxx")),
            ("export.cxx", binding.join("src/export.cxx")),
            ("lib.rs", binding.join("src/lib.rs")),
            ("build.rs", binding.join("build.rs")),
        ] {
            fs::copy(fixture.join(source), destination)?;
        }

        let project = env!("CARGO_MANIFEST_DIR");
        let binding_manifest = format!(
            r#"[package]
name = "dynamic-binding-fixture"
version = "0.0.0"
edition = "2024"
build = "build.rs"

[lib]
crate-type = ["cdylib"]

[dependencies]
cxx = {{ version = "1.0", features = ["c++20"] }}
cxx-auto = {{ path = "{project}" }}

[build-dependencies]
cxx-auto = {{ path = "{project}" }}
cxx-build = "1.0"
"#
        );
        fs::write(binding.join("Cargo.toml"), binding_manifest)?;
        let output = Command::new("cargo")
            .args(["build", "--quiet", "--manifest-path"])
            .arg(binding.join("Cargo.toml"))
            .env("CARGO_TARGET_DIR", &target_dir)
            .output()?;
        assert!(
            output.status.success(),
            "generated binding build failed in {}:\n{}\n{}",
            binding.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        let library_name = format!(
            "{}dynamic_binding_fixture.{}",
            std::env::consts::DLL_PREFIX,
            std::env::consts::DLL_EXTENSION
        );
        let library_path = root.join("target/debug").join(library_name);
        // SAFETY: the library is built from the checked-in fixture sources above.
        let library = unsafe { libloading::Library::new(library_path)? };
        // SAFETY: the fixture exports compare_number with exactly this C ABI signature.
        let compare: libloading::Symbol<'_, unsafe extern "C" fn(i32, i32) -> i32> =
            unsafe { library.get(b"compare_number")? };
        let comparisons: [(i32, i32, i32); 5] = [
            (4, 7, -1),
            (7, 4, 1),
            (7, 7, 0),
            (i32::MIN, i32::MAX, -1),
            (i32::MAX, i32::MIN, 1),
        ];
        for (lhs, rhs, expected) in comparisons {
            // SAFETY: the loaded symbol has the fixture's C ABI and accepts every i32 pair.
            let actual = unsafe { compare(lhs, rhs) };
            assert_eq!(actual, expected, "comparing {lhs} and {rhs}");
        }
        // SAFETY: the fixture exports exercise_objects with exactly this C ABI
        // signature.
        let exercise: libloading::Symbol<'_, unsafe extern "C" fn() -> i32> =
            unsafe { library.get(b"exercise_objects")? };
        // SAFETY: the loaded symbol has the fixture's C ABI and takes no arguments.
        let failed_step = unsafe { exercise() };
        assert_eq!(
            failed_step, 0_i32,
            "in-place construction step {failed_step} failed"
        );
        // SAFETY: these fixture exports have the stated C ABI and no arguments.
        let traits: libloading::Symbol<'_, unsafe extern "C" fn() -> i32> =
            unsafe { library.get(b"exercise_traits")? };
        // SAFETY: exercise_throwing has the stated C ABI and no arguments.
        let throwing: libloading::Symbol<'_, unsafe extern "C" fn() -> i32> =
            unsafe { library.get(b"exercise_throwing")? };
        // SAFETY: both symbols were loaded with their exact fixture signatures.
        let trait_step = unsafe { traits() };
        assert_eq!(trait_step, 0_i32, "generated trait semantics");
        // SAFETY: this export takes no arguments and has its exact loaded signature.
        let throwing_step = unsafe { throwing() };
        assert_eq!(throwing_step, 0_i32, "catching special-member semantics");
        drop(library);
        fs::remove_dir_all(root)?;
        Ok(())
    }
}
