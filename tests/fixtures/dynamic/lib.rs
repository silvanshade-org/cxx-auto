/// Generated Rust representation of the C++ Number and its trait
/// implementations.
pub mod auto
{
    include!(concat!(env!("OUT_DIR"), "/auto.rs"));
}

#[cxx::bridge(namespace = "demo")]
mod ffi
{
    unsafe extern "C++" {
        include!("number.hxx");
        type Number = crate::auto::number::Number;
        fn make_number(value: i32) -> Number;
    }
}

/// Compare two C++ values through the generated Rust PartialEq and Ord
/// implementations.
///
/// # Specification
/// - provides: -1, 0, or 1 for less, equal, or greater; 2 exposes any
///   disagreement between the generated traits.
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: unequal and equal C++ values must retain both equality and
///   ordering semantics across CXX and the shared-library boundary.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
// FFI escape hatch: the C ABI takes and returns i32 so a dynamic loader can call this symbol
// without Rust-specific layout.
#[unsafe(no_mangle)]
pub extern "C" fn compare_number(
    lhs: i32,
    rhs: i32,
) -> i32
{
    let lhs = ffi::make_number(lhs);
    let rhs = ffi::make_number(rhs);
    match (lhs == rhs, lhs.cmp(&rhs)) {
        | (true, ::core::cmp::Ordering::Equal) => 0,
        | (false, ::core::cmp::Ordering::Less) => -1,
        | (false, ::core::cmp::Ordering::Greater) => 1,
        | _ => 2,
    }
}
