/// Generated Rust representations of the fixture's C++ types and their trait
/// implementations.
pub mod auto
{
    include!(concat!(env!("OUT_DIR"), "/auto.rs"));
}

use core::pin::Pin;

use auto::handle::Handle;
use auto::tracked::Tracked;
use cxx_auto::init::InPlaceInit as _;

#[cxx::bridge(namespace = "demo")]
mod ffi
{
    unsafe extern "C++" {
        include!("number.hxx");
        include!("objects.hxx");
        type Number = crate::auto::number::Number;
        type Tracked = crate::auto::tracked::Tracked;
        type Handle = crate::auto::handle::Handle;
        fn make_number(value: i32) -> Number;
        fn tracked_live() -> i32;
        fn tracked_fail_next();
        fn tracked_value(tracked: &Tracked) -> i32;
        fn tracked_set(
            tracked: Pin<&mut Tracked>,
            value: i32,
        );
        fn handle_live() -> i32;
    }
}

/// Fails to compile if `T: Unpin`, through an ambiguous associated item.
trait AmbiguousIfUnpin<Marker>
{
    /// Named by the check below.
    fn check()
    {
    }
}
impl<T: ?Sized> AmbiguousIfUnpin<()> for T
{
}
/// The second impl's marker.
struct IsUnpin;
impl<T: ?Sized + Unpin> AmbiguousIfUnpin<IsUnpin> for T
{
}

// `Tracked` is not relocatable, so it must stay pinned.
const _: fn() = || <Tracked as AmbiguousIfUnpin<_>>::check();
// `Handle`'s author declared it relocatable, so it may move.
const _: fn() = || {
    fn unpin<T: Unpin>()
    {
    }
    unpin::<Handle>();
};

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

/// Construct, copy, move, assign, and destroy C++ objects in Rust-owned places
/// through the generated initializers and methods.
///
/// # Specification
/// - provides: 0 when every step behaves as C++ specifies, otherwise the number
///   of the first step that did not.
/// - ensures: every C++ object constructed here is destroyed exactly once, and
///   `Tracked` is never relocated (it aborts the process if it is).
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: generated construction must run C++ special members in place,
///   leave moved-from sources with their owners, report thrown exceptions as
///   errors without leaking or double-destroying, and let a relocatable type
///   move as a plain Rust value.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
// FFI escape hatch: the C ABI returns i32 so a dynamic loader can call this symbol without
// Rust-specific layout.
#[unsafe(no_mangle)]
pub extern "C" fn exercise_objects() -> i32
{
    let tracked_before = ffi::tracked_live();
    let handles_before = ffi::handle_live();
    let step = exercise_tracked().and_then(|()| exercise_handle());
    if let Err(step) = step {
        return step;
    }
    if ffi::tracked_live() != tracked_before {
        return 90;
    }
    if ffi::handle_live() != handles_before {
        return 91;
    }
    0
}

/// Fail with `step` unless `condition` holds.
fn expect(
    condition: bool,
    step: i32,
) -> Result<(), i32>
{
    if condition { Ok(()) } else { Err(step) }
}

/// Exercise the non-relocatable `Tracked`.
fn exercise_tracked() -> Result<(), i32>
{
    // Default construction may throw, so its initializer is fallible.
    cxx_auto::stack_try_pin_init!(let first = Tracked::default_new());
    let mut first = first.map_err(|_| 1)?;
    ffi::tracked_set(first.as_mut(), 7);

    // Copy construction into the heap.
    let mut copied: Pin<Box<Tracked>> = Box::pin_init(Tracked::copy_from(&first));
    expect(ffi::tracked_value(&copied) == 7, 2)?;

    // Move construction into a new stack place; the source stays, moved-from.
    cxx_auto::stack_pin_init!(let mut moved = Tracked::move_from(first.as_mut()));
    expect(ffi::tracked_value(&moved) == 7, 3)?;
    expect(ffi::tracked_value(&first) == -1, 4)?;

    // Move assignment cannot throw, so it returns nothing.
    ffi::tracked_set(moved.as_mut(), 8);
    copied.as_mut().move_assign(moved.as_mut());
    expect(ffi::tracked_value(&copied) == 8, 5)?;
    expect(ffi::tracked_value(&moved) == -1, 6)?;

    // Copy assignment may throw; copying from a moved-from object does.
    let refused = copied.as_mut().copy_assign(&moved);
    expect(
        refused
            .as_ref()
            .err()
            .map(cxx_auto::init::CxxException::what)
            == Some("copy from a moved-from object"),
        7,
    )?;
    expect(ffi::tracked_value(&copied) == 8, 8)?;
    expect(copied.as_mut().copy_assign(&first).is_err(), 9)?;

    // A throwing constructor leaves nothing constructed, on the heap or stack.
    let live = ffi::tracked_live();
    ffi::tracked_fail_next();
    let failed = Box::try_pin_init(Tracked::default_new());
    expect(
        failed
            .as_ref()
            .err()
            .map(cxx_auto::init::CxxException::what)
            == Some("refused"),
        10,
    )?;
    ffi::tracked_fail_next();
    cxx_auto::stack_try_pin_init!(let failed_local = Tracked::default_new());
    expect(failed_local.is_err(), 11)?;
    expect(ffi::tracked_live() == live, 12)?;
    Ok(())
}

/// Exercise the relocatable `Handle`, which Rust may move as a plain value.
fn exercise_handle() -> Result<(), i32>
{
    let pinned = Box::pin_init(Handle::default_new());
    expect(ffi::handle_live() >= 1, 20)?;
    // `Handle: Unpin`, so it leaves its box and moves bytewise onto the stack;
    // its destructor then runs once, at the new address.
    let boxed: Box<Handle> = Pin::into_inner(pinned);
    let local: Handle = *boxed;
    let moved = [local];
    drop(moved);
    Ok(())
}
