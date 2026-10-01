/// Generated Rust representations of the fixture's C++ types and their trait
/// implementations.
pub mod auto
{
    include!(concat!(env!("OUT_DIR"), "/auto.rs"));
}

use core::pin::Pin;

use auto::handle::Handle;
use auto::throwing::Throwing;
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
        type PartialNumber = crate::auto::partial_number::PartialNumber;
        type WeakNumber = crate::auto::weak_number::WeakNumber;
        type LegacyNumber = crate::auto::legacy_number::LegacyNumber;
        type Throwing = crate::auto::throwing::Throwing;
        fn make_partial(value: f64) -> PartialNumber;
        fn make_weak(value: i32) -> WeakNumber;
        fn make_legacy(value: f64) -> LegacyNumber;
        fn throwing_live() -> i32;
        fn throwing_fail_next(failure: i32);
        fn throwing_value(object: &Throwing) -> i32;
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

/// Exercise partial, weak and legacy ordering, hashing, and C++ formatting.
///
/// # Specification
/// - provides: 0 when all results agree with the C++ relation and projection,
///   otherwise the failing step's number.
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: equivalent, reversed and unordered classifications must
///   survive the typed bridge, as must the hash word and distinct formatting
///   paths.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
// FFI escape hatch: the C ABI returns i32 for the dynamic loader.
#[unsafe(no_mangle)]
pub extern "C" fn exercise_traits() -> i32
{
    use core::cmp::Ordering::Equal;
    use core::cmp::Ordering::Greater;
    use core::cmp::Ordering::Less;
    use core::hash::Hash as _;
    use core::hash::Hasher as _;
    for (left, right, expected) in [
        (1.0, 2.0, Some(Less)),
        (2.0, 1.0, Some(Greater)),
        (-0.0, 0.0, Some(Equal)),
        (f64::NAN, 1.0, None),
        (1.0, f64::NAN, None),
        (f64::NAN, f64::NAN, None),
    ] {
        let lhs = ffi::make_partial(left);
        let rhs = ffi::make_partial(right);
        if lhs.partial_cmp(&rhs) != expected
            || (lhs == rhs) != (left == right)
            || (lhs != rhs) != (left != right)
        {
            return 1;
        }
        let lhs = ffi::make_legacy(left);
        let rhs = ffi::make_legacy(right);
        if lhs.partial_cmp(&rhs) != expected
            || (lhs == rhs) != (left == right)
            || (lhs != rhs) != (left != right)
        {
            return 2;
        }
    }
    for (left, right, expected) in [(11, 19, Equal), (11, 20, Less), (20, 11, Greater)] {
        let lhs = ffi::make_weak(left);
        let rhs = ffi::make_weak(right);
        if lhs.partial_cmp(&rhs) != Some(expected)
            || (lhs == rhs) != (expected == Equal)
            || (lhs != rhs) != (expected != Equal)
        {
            return 3;
        }
    }
    for value in [0, 7, -1, i32::MIN, i32::MAX] {
        let number = ffi::make_number(value);
        let mut actual = std::collections::hash_map::DefaultHasher::new();
        number.hash(&mut actual);
        let mut expected = std::collections::hash_map::DefaultHasher::new();
        let Ok(hash) = usize::try_from(u32::from_ne_bytes(value.to_ne_bytes()))
        else {
            return 4;
        };
        let Some(hash) = hash.checked_add(0x51)
        else {
            return 4;
        };
        hash.hash(&mut expected);
        if actual.finish() != expected.finish() {
            return 4;
        }
        if format!("{number:?}") != format!("Number({value})") {
            return 5;
        }
        if format!("{number}") != value.to_string() {
            return 6;
        }
    }
    0
}

/// Exercise all catching special members, including an unknown C++ exception.
///
/// # Specification
/// - ensures: failed construction creates no live object and success destroys
///   each object once; failed assignments preserve this fixture's source and
///   target.
/// - provides: 0 on agreement with C++, otherwise the failing step's number.
/// - panics: none.
///
/// # Adequacy
/// - hypothesis: typed completion must preserve error messages, construction
///   rollback, moved-from ownership and catching-assignment semantics.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
// FFI escape hatch: the C ABI returns i32 for the dynamic loader.
#[unsafe(no_mangle)]
pub extern "C" fn exercise_throwing() -> i32
{
    let before = ffi::throwing_live();
    {
        let Ok(mut first) = Box::try_pin_init(Throwing::default_new())
        else {
            return 1;
        };
        let Ok(mut copied) = Box::try_pin_init(Throwing::copy_from(&first))
        else {
            return 2;
        };
        if ffi::throwing_value(&copied) != 7 {
            return 3;
        }
        let live = ffi::throwing_live();
        ffi::throwing_fail_next(1);
        let failed = Box::try_pin_init(Throwing::copy_from(&first));
        if failed
            .as_ref()
            .err()
            .map(cxx_auto::init::CxxException::what)
            != Some("special member refused")
            || ffi::throwing_live() != live
        {
            return 4;
        }
        ffi::throwing_fail_next(1);
        let failed = Box::try_pin_init(Throwing::move_from(first.as_mut()));
        if failed
            .as_ref()
            .err()
            .map(cxx_auto::init::CxxException::what)
            != Some("special member refused")
            || ffi::throwing_live() != live
            || ffi::throwing_value(&first) != 7
        {
            return 5;
        }
        let Ok(mut moved) = Box::try_pin_init(Throwing::move_from(first.as_mut()))
        else {
            return 6;
        };
        if ffi::throwing_value(&moved) != 7 || ffi::throwing_value(&first) != -1 {
            return 7;
        }
        ffi::throwing_fail_next(1);
        let failed = first.as_mut().move_assign(moved.as_mut());
        if failed
            .as_ref()
            .err()
            .map(cxx_auto::init::CxxException::what)
            != Some("special member refused")
            || ffi::throwing_value(&first) != -1
            || ffi::throwing_value(&moved) != 7
        {
            return 8;
        }
        if first.as_mut().move_assign(moved.as_mut()).is_err()
            || ffi::throwing_value(&first) != 7
            || ffi::throwing_value(&moved) != -1
        {
            return 9;
        }
        ffi::throwing_fail_next(1);
        let failed = copied.as_mut().copy_assign(&moved);
        if failed
            .as_ref()
            .err()
            .map(cxx_auto::init::CxxException::what)
            != Some("special member refused")
            || ffi::throwing_value(&copied) != 7
        {
            return 10;
        }
        if copied.as_mut().copy_assign(&moved).is_err() || ffi::throwing_value(&copied) != -1 {
            return 11;
        }
        ffi::throwing_fail_next(2);
        let failed = Box::try_pin_init(Throwing::default_new());
        if failed
            .as_ref()
            .err()
            .map(cxx_auto::init::CxxException::what)
            != Some("unknown C++ exception")
            || Some(ffi::throwing_live()) != live.checked_add(1)
        {
            return 12;
        }
    }
    if ffi::throwing_live() != before {
        return 13;
    }
    0
}
