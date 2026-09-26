/// First-stage modules call C++ capability probes and write the final binding
/// source.
mod auto
{
    include!(concat!(env!("OUT_DIR"), "/src/auto.rs"));
}

/// Run the generated C++ probes and emit final modules into the requested
/// library directory.
///
/// # Specification
/// - provides: final Rust bindings generated from C++ layout and trait
///   capability results.
/// - fails: a missing output directory argument, probe failure, or output write
///   error.
/// - panics: none.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let destination = std::path::PathBuf::from(
        std::env::args_os()
            .nth(1)
            .ok_or("missing binding output directory")?,
    );
    auto::process_artifacts(&destination)?;
    Ok(())
}
