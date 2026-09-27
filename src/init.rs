//! Build C++ objects directly in the Rust place that owns them.
//!
//! A C++ object that is not trivially relocatable must never be moved
//! bytewise, so it is constructed where it will live: in a `Pin<Box<T>>`, a
//! `Pin<Rc<T>>`, a `Pin<Arc<T>>`, or a pinned stack slot. Generated bindings
//! return constructors as initializers ([`PinInit`](crate::init::PinInit)), and
//! the owner runs them against its own storage. The owner destroys the object
//! when it ends, as for any Rust value, so there is no separate drop tracking.
//!
//! A C++ move is a move constructor: an initializer that builds a new object
//! from a pinned source and leaves the source, moved-from, with its own owner.
//!
//! Placement:
//!
//! ```ignore
//! let boxed = Box::pin_init(Widget::default_new());
//! cxx_auto::stack_pin_init!(let mut local = Widget::move_from(boxed.as_mut()));
//! ```

use core::convert::Infallible;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::pin::Pin;

/// An initializer that writes a `T` into storage it is given and may fail with
/// `E`, leaving the result pinned.
///
/// # Safety
///
/// An implementation of [`PinInit::pinned_init`] must, when it returns
/// `Ok(())`, have fully initialized `*slot`, and when it returns `Err`, have
/// left nothing in `*slot` that needs dropping: anything it built before
/// failing is already destroyed.
pub unsafe trait PinInit<T: ?Sized, E = Infallible>: Sized
{
    /// Initialize `*slot`.
    ///
    /// # Safety
    ///
    /// `slot` is valid for writes of a `T` and properly aligned. On `Ok`, the
    /// caller owns an initialized `T` at `slot` and treats it as pinned: it is
    /// dropped in place and never moved unless `T: Unpin`. On `Err`, the caller
    /// may reuse or free `slot` without dropping anything.
    ///
    /// # Errors
    ///
    /// Returns the initializer's error, with nothing left in `*slot`.
    unsafe fn pinned_init(
        self,
        slot: *mut T,
    ) -> Result<(), E>;
}

/// An initializer whose result may be moved afterwards.
///
/// # Safety
///
/// As for [`PinInit`], and [`Init::init`] must leave the same result as
/// [`PinInit::pinned_init`] without relying on the result being pinned.
pub unsafe trait Init<T: ?Sized, E = Infallible>: PinInit<T, E>
{
    /// Initialize `*slot`.
    ///
    /// # Safety
    ///
    /// As for [`PinInit::pinned_init`], except that the caller may move the
    /// result.
    ///
    /// # Errors
    ///
    /// Returns the initializer's error, with nothing left in `*slot`.
    unsafe fn init(
        self,
        slot: *mut T,
    ) -> Result<(), E>;
}

// SAFETY: writing a value fully initializes the slot and cannot fail.
unsafe impl<T> PinInit<T> for T
{
    #[inline]
    unsafe fn pinned_init(
        self,
        slot: *mut T,
    ) -> Result<(), Infallible>
    {
        // SAFETY: the caller guarantees `slot` is valid for writes of a `T`.
        unsafe { slot.write(self) };
        Ok(())
    }
}

// SAFETY: as for `PinInit`; a plain value never depends on its address.
unsafe impl<T> Init<T> for T
{
    #[inline]
    unsafe fn init(
        self,
        slot: *mut T,
    ) -> Result<(), Infallible>
    {
        // SAFETY: the caller guarantees `slot` is valid for writes of a `T`.
        unsafe { slot.write(self) };
        Ok(())
    }
}

// SAFETY: `Ok` writes the value; `Err` writes nothing.
unsafe impl<T, E> PinInit<T, E> for Result<T, E>
{
    #[inline]
    unsafe fn pinned_init(
        self,
        slot: *mut T,
    ) -> Result<(), E>
    {
        // SAFETY: the caller guarantees `slot` is valid for writes of a `T`.
        unsafe { slot.write(self?) };
        Ok(())
    }
}

// SAFETY: as for `PinInit`.
unsafe impl<T, E> Init<T, E> for Result<T, E>
{
    #[inline]
    unsafe fn init(
        self,
        slot: *mut T,
    ) -> Result<(), E>
    {
        // SAFETY: the caller guarantees `slot` is valid for writes of a `T`.
        unsafe { slot.write(self?) };
        Ok(())
    }
}

/// Records an initializer's target and error types without owning either.
type InitMarker<T, E> = PhantomData<fn(*mut T) -> Result<(), E>>;

/// A pinned initializer backed by a closure; see [`pin_init_from_closure`].
pub struct PinInitClosure<F, T: ?Sized, E>
{
    /// The closure that initializes the slot.
    body: F,
    /// The initialized type and error, without owning either.
    marker: InitMarker<T, E>,
}

// SAFETY: `pin_init_from_closure`'s caller vouched for the closure.
unsafe impl<F, T: ?Sized, E> PinInit<T, E> for PinInitClosure<F, T, E>
where
    F: FnOnce(*mut T) -> Result<(), E>,
{
    #[inline]
    unsafe fn pinned_init(
        self,
        slot: *mut T,
    ) -> Result<(), E>
    {
        (self.body)(slot)
    }
}

/// An unpinned initializer backed by a closure; see [`init_from_closure`].
pub struct InitClosure<F, T: ?Sized, E>
{
    /// The closure that initializes the slot.
    body: F,
    /// The initialized type and error, without owning either.
    marker: InitMarker<T, E>,
}

// SAFETY: `init_from_closure`'s caller vouched for the closure.
unsafe impl<F, T: ?Sized, E> PinInit<T, E> for InitClosure<F, T, E>
where
    F: FnOnce(*mut T) -> Result<(), E>,
{
    #[inline]
    unsafe fn pinned_init(
        self,
        slot: *mut T,
    ) -> Result<(), E>
    {
        (self.body)(slot)
    }
}

// SAFETY: `init_from_closure`'s caller vouched that the result may move.
unsafe impl<F, T: ?Sized, E> Init<T, E> for InitClosure<F, T, E>
where
    F: FnOnce(*mut T) -> Result<(), E>,
{
    #[inline]
    unsafe fn init(
        self,
        slot: *mut T,
    ) -> Result<(), E>
    {
        (self.body)(slot)
    }
}

/// Make a pinned initializer from a closure.
///
/// # Safety
///
/// Called with a `slot` valid for writes of a `T`, `body` must either fully
/// initialize `*slot` and return `Ok(())`, or leave nothing in it that needs
/// dropping and return `Err`.
#[inline]
pub const unsafe fn pin_init_from_closure<T, E, F>(body: F) -> PinInitClosure<F, T, E>
where
    T: ?Sized,
    F: FnOnce(*mut T) -> Result<(), E>,
{
    PinInitClosure {
        body,
        marker: PhantomData,
    }
}

/// Make an unpinned initializer from a closure.
///
/// # Safety
///
/// As for [`pin_init_from_closure`], and the value `body` builds must remain
/// valid when moved bytewise.
#[inline]
pub const unsafe fn init_from_closure<T, E, F>(body: F) -> InitClosure<F, T, E>
where
    T: ?Sized,
    F: FnOnce(*mut T) -> Result<(), E>,
{
    InitClosure {
        body,
        marker: PhantomData,
    }
}

/// Owners that can allocate their storage and run an initializer in it.
#[cfg(feature = "alloc")]
pub trait InPlaceInit<T>: Sized
{
    /// Allocate, run `init` in the new storage, and pin the result.
    ///
    /// # Errors
    ///
    /// Returns the initializer's error; the storage is then freed and nothing
    /// is dropped.
    fn try_pin_init<E>(init: impl PinInit<T, E>) -> Result<Pin<Self>, E>;

    /// Allocate, run an infallible `init`, and pin the result.
    #[inline]
    fn pin_init(init: impl PinInit<T>) -> Pin<Self>
    {
        match Self::try_pin_init(init) {
            | Ok(pinned) => pinned,
            | Err(never) => match never {},
        }
    }

    /// Allocate and run `init`, whose result may move.
    ///
    /// # Errors
    ///
    /// Returns the initializer's error; the storage is then freed and nothing
    /// is dropped.
    fn try_init<E>(init: impl Init<T, E>) -> Result<Self, E>;

    /// Allocate and run an infallible `init` whose result may move.
    #[inline]
    fn init(init: impl Init<T>) -> Self
    {
        match Self::try_init(init) {
            | Ok(owned) => owned,
            | Err(never) => match never {},
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> InPlaceInit<T> for alloc::boxed::Box<T>
{
    #[inline]
    fn try_pin_init<E>(init: impl PinInit<T, E>) -> Result<Pin<Self>, E>
    {
        let mut storage = Self::new_uninit();
        // SAFETY: fresh, aligned, uniquely owned storage for one `T`; on error
        // the uninitialized box is freed without dropping a `T`.
        unsafe { init.pinned_init(storage.as_mut_ptr()) }?;
        // SAFETY: the initializer returned `Ok`, so the value is initialized.
        Ok(Self::into_pin(unsafe { storage.assume_init() }))
    }

    #[inline]
    fn try_init<E>(init: impl Init<T, E>) -> Result<Self, E>
    {
        let mut storage = Self::new_uninit();
        // SAFETY: as above.
        unsafe { init.init(storage.as_mut_ptr()) }?;
        // SAFETY: the initializer returned `Ok`, so the value is initialized.
        Ok(unsafe { storage.assume_init() })
    }
}

#[cfg(feature = "alloc")]
impl<T> InPlaceInit<T> for alloc::rc::Rc<T>
{
    #[inline]
    fn try_pin_init<E>(init: impl PinInit<T, E>) -> Result<Pin<Self>, E>
    {
        let mut storage = Self::new_uninit();
        // SAFETY: `storage` was just allocated and is referenced nowhere else.
        let slot = unsafe { unique_slot(alloc::rc::Rc::get_mut(&mut storage)) };
        // SAFETY: fresh, aligned, uniquely owned storage for one `T`; on error
        // the uninitialized allocation is freed without dropping a `T`.
        unsafe { init.pinned_init(slot) }?;
        // SAFETY: the initializer returned `Ok`, so the value is initialized.
        let owned = unsafe { storage.assume_init() };
        // SAFETY: the value never moves out of its shared allocation.
        Ok(unsafe { Pin::new_unchecked(owned) })
    }

    #[inline]
    fn try_init<E>(init: impl Init<T, E>) -> Result<Self, E>
    {
        let mut storage = Self::new_uninit();
        // SAFETY: `storage` was just allocated and is referenced nowhere else.
        let slot = unsafe { unique_slot(alloc::rc::Rc::get_mut(&mut storage)) };
        // SAFETY: as above.
        unsafe { init.init(slot) }?;
        // SAFETY: the initializer returned `Ok`, so the value is initialized.
        Ok(unsafe { storage.assume_init() })
    }
}

#[cfg(feature = "alloc")]
impl<T> InPlaceInit<T> for alloc::sync::Arc<T>
{
    #[inline]
    fn try_pin_init<E>(init: impl PinInit<T, E>) -> Result<Pin<Self>, E>
    {
        let mut storage = Self::new_uninit();
        // SAFETY: `storage` was just allocated and is referenced nowhere else.
        let slot = unsafe { unique_slot(alloc::sync::Arc::get_mut(&mut storage)) };
        // SAFETY: fresh, aligned, uniquely owned storage for one `T`; on error
        // the uninitialized allocation is freed without dropping a `T`.
        unsafe { init.pinned_init(slot) }?;
        // SAFETY: the initializer returned `Ok`, so the value is initialized.
        let owned = unsafe { storage.assume_init() };
        // SAFETY: the value never moves out of its shared allocation.
        Ok(unsafe { Pin::new_unchecked(owned) })
    }

    #[inline]
    fn try_init<E>(init: impl Init<T, E>) -> Result<Self, E>
    {
        let mut storage = Self::new_uninit();
        // SAFETY: `storage` was just allocated and is referenced nowhere else.
        let slot = unsafe { unique_slot(alloc::sync::Arc::get_mut(&mut storage)) };
        // SAFETY: as above.
        unsafe { init.init(slot) }?;
        // SAFETY: the initializer returned `Ok`, so the value is initialized.
        Ok(unsafe { storage.assume_init() })
    }
}

/// The storage behind a freshly allocated reference-counted pointer.
///
/// # Safety
///
/// `unique` came from `get_mut` on an allocation nothing else references,
/// which is always `Some`.
#[cfg(feature = "alloc")]
#[inline]
unsafe fn unique_slot<T>(unique: Option<&mut MaybeUninit<T>>) -> *mut T
{
    match unique {
        | Some(storage) => storage.as_mut_ptr(),
        // SAFETY: the caller passes the result of `get_mut` on a new allocation.
        | None => unsafe { core::hint::unreachable_unchecked() },
    }
}

/// Stack storage for one pinned value; use it through [`stack_pin_init!`] and
/// [`stack_try_pin_init!`](crate::stack_try_pin_init), which keep it unnamed.
///
/// [`stack_pin_init!`]: crate::stack_pin_init
pub struct StackSlot<T>
{
    /// Storage for the value.
    value: MaybeUninit<T>,
    /// Whether `value` holds a live `T` that this slot must drop.
    initialized: bool,
}

impl<T> StackSlot<T>
{
    /// An empty slot.
    #[inline]
    #[must_use]
    pub const fn uninit() -> Self
    {
        Self {
            value: MaybeUninit::uninit(),
            initialized: false,
        }
    }

    /// Run `init` in this slot and pin the result.
    ///
    /// # Safety
    ///
    /// Once this returns `Ok`, the slot is neither moved nor forgotten until it
    /// is dropped, which drops the value in place. The macros guarantee this by
    /// keeping the slot in a variable nothing else can name.
    ///
    /// # Errors
    ///
    /// Returns the initializer's error; the slot then stays empty.
    #[inline]
    pub unsafe fn try_pin_init<E, I>(
        &mut self,
        init: I,
    ) -> Result<Pin<&mut T>, E>
    where
        I: PinInit<T, E>,
    {
        if self.initialized {
            self.initialized = false;
            // SAFETY: `initialized` recorded a live value.
            unsafe { self.value.assume_init_drop() };
        }
        // SAFETY: the slot's storage is valid and aligned for a `T`.
        unsafe { init.pinned_init(self.value.as_mut_ptr()) }?;
        self.initialized = true;
        // SAFETY: the initializer returned `Ok`, so the value is initialized.
        let value = unsafe { self.value.assume_init_mut() };
        // SAFETY: the caller keeps the slot in place while the value is live.
        Ok(unsafe { Pin::new_unchecked(value) })
    }

    /// Run an infallible `init` in this slot and pin the result.
    ///
    /// # Safety
    ///
    /// As for [`StackSlot::try_pin_init`].
    #[inline]
    pub unsafe fn pin_init<I>(
        &mut self,
        init: I,
    ) -> Pin<&mut T>
    where
        I: PinInit<T>,
    {
        // SAFETY: forwarded to the caller.
        match unsafe { self.try_pin_init(init) } {
            | Ok(pinned) => pinned,
            | Err(never) => match never {},
        }
    }
}

impl<T> Drop for StackSlot<T>
{
    #[inline]
    fn drop(&mut self)
    {
        if self.initialized {
            // SAFETY: `initialized` records a live value, dropped in place.
            unsafe { self.value.assume_init_drop() };
        }
    }
}

/// Construct a value in a pinned stack slot: `stack_pin_init!(let x =
/// init);` binds `x: Pin<&mut T>`. The initializer must be infallible.
#[macro_export]
macro_rules! stack_pin_init {
    (let $var:ident $(: $ty:ty)? = $init:expr $(;)?) => {
        let init = $init;
        let mut slot = $crate::init::StackSlot $(::<$ty>)? ::uninit();
        // SAFETY: `slot` is a hygienic local that no other code can name, so
        // it is neither moved nor forgotten before it drops.
        let $var = unsafe { slot.pin_init(init) };
    };
    (let mut $var:ident $(: $ty:ty)? = $init:expr $(;)?) => {
        let init = $init;
        let mut slot = $crate::init::StackSlot $(::<$ty>)? ::uninit();
        // SAFETY: as above.
        let mut $var = unsafe { slot.pin_init(init) };
    };
}

/// Construct a value in a pinned stack slot with a fallible initializer:
/// `stack_try_pin_init!(let x = init);` binds `x: Result<Pin<&mut T>, E>`.
#[macro_export]
macro_rules! stack_try_pin_init {
    (let $var:ident $(: $ty:ty)? = $init:expr $(;)?) => {
        let init = $init;
        let mut slot = $crate::init::StackSlot $(::<$ty>)? ::uninit();
        // SAFETY: `slot` is a hygienic local that no other code can name, so
        // it is neither moved nor forgotten before it drops.
        let $var = unsafe { slot.try_pin_init(init) };
    };
    (let mut $var:ident $(: $ty:ty)? = $init:expr $(;)?) => {
        let init = $init;
        let mut slot = $crate::init::StackSlot $(::<$ty>)? ::uninit();
        // SAFETY: as above.
        let mut $var = unsafe { slot.try_pin_init(init) };
    };
}

/// A C++ exception caught at the binding boundary.
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CxxException
{
    /// `what()` of a `std::exception`, or a fixed description otherwise.
    what: alloc::string::String,
}

#[cfg(feature = "alloc")]
impl CxxException
{
    /// Wrap the message a generated shim caught.
    #[inline]
    #[must_use]
    pub const fn new(what: alloc::string::String) -> Self
    {
        Self { what }
    }

    /// The exception's message.
    #[inline]
    #[must_use]
    pub fn what(&self) -> &str
    {
        &self.what
    }
}

/// Run a generated catching shim and turn its report into a `Result`.
///
/// The shim returns `true` when the C++ operation completed, and otherwise
/// `false` after writing the exception's message into the `String` it is
/// given. Generated bindings call this; it is not meant for direct use.
///
/// # Errors
///
/// Returns the caught exception when the shim reports one.
// The `bool` is the shim's C++ return type across the bridge, which has no
// richer shape to offer.
#[cfg(feature = "alloc")]
#[doc(hidden)]
#[inline]
pub fn cxx_try<F>(shim: F) -> Result<(), CxxException>
where
    F: FnOnce(&mut alloc::string::String) -> bool,
{
    let mut what = alloc::string::String::new();
    if shim(&mut what) {
        Ok(())
    }
    else {
        Err(CxxException::new(what))
    }
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for CxxException
{
    #[inline]
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    {
        write!(f, "C++ exception: {}", self.what)
    }
}

#[cfg(feature = "alloc")]
impl core::error::Error for CxxException
{
}

#[cfg(test)]
mod tests
{
    extern crate std;

    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::sync::Arc;
    use core::cell::Cell;
    use core::marker::PhantomPinned;
    use core::pin::Pin;

    use super::InPlaceInit as _;
    use super::PinInit;
    use super::pin_init_from_closure;

    std::thread_local! {
        static DROPS: Cell<usize> = const { Cell::new(0) };
    }

    /// A value that must not move after construction and counts its drops.
    struct Anchored
    {
        this: *const Self,
        _pin: PhantomPinned,
    }

    impl Anchored
    {
        /// Construct in place, recording the final address.
        fn new() -> impl PinInit<Self>
        {
            let body = |slot: *mut Self| {
                // SAFETY: the owner passes storage valid for writes of `Self`.
                unsafe {
                    slot.write(Self {
                        this: slot,
                        _pin: PhantomPinned,
                    });
                };
                Ok(())
            };
            // SAFETY: `body` fully writes the slot and never fails.
            unsafe { pin_init_from_closure(body) }
        }

        /// An initializer that fails before building anything.
        fn failing() -> impl PinInit<Self, &'static str>
        {
            // SAFETY: fails without writing the slot.
            unsafe { pin_init_from_closure(|_slot: *mut Self| Err("refused")) }
        }

        fn at_home(self: Pin<&Self>) -> bool
        {
            core::ptr::eq(self.this, &raw const *self)
        }
    }

    impl Drop for Anchored
    {
        fn drop(&mut self)
        {
            DROPS.with(|drops| drops.set(drops.get().saturating_add(1)));
        }
    }

    fn drops() -> usize
    {
        DROPS.with(Cell::get)
    }

    #[test]
    fn owners_construct_in_place_and_drop_once()
    {
        let before = drops();
        {
            let boxed = Box::pin_init(Anchored::new());
            let rc = Rc::pin_init(Anchored::new());
            let arc = Arc::pin_init(Anchored::new());
            crate::stack_pin_init!(let local = Anchored::new());
            assert!(boxed.as_ref().at_home());
            assert!(rc.as_ref().at_home());
            assert!(arc.as_ref().at_home());
            assert!(local.as_ref().at_home());
        };
        assert_eq!(drops().saturating_sub(before), 4);
    }

    #[test]
    fn failed_initializers_leave_nothing_to_drop()
    {
        let before = drops();
        assert_eq!(
            Box::try_pin_init(Anchored::failing()).err(),
            Some("refused")
        );
        assert_eq!(Rc::try_pin_init(Anchored::failing()).err(), Some("refused"));
        assert_eq!(
            Arc::try_pin_init(Anchored::failing()).err(),
            Some("refused")
        );
        {
            crate::stack_try_pin_init!(let local = Anchored::failing());
            assert_eq!(local.err(), Some("refused"));
        };
        assert_eq!(drops(), before);
    }
}
