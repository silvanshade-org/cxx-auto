#pragma once

#include <cstdint>
#include <cstdlib>
#include <stdexcept>

namespace demo {
/// A C++ object that must never move bytewise: it stores its own address and
/// aborts the process if it ever finds itself elsewhere.
///
/// # Specification
/// - provides: a default constructor that throws when `fail_next` is set, a
///   copy assignment that throws when copying from a moved-from object, and
///   `noexcept` copy and move construction, move assignment, and destruction.
/// - ensures: `live` counts objects constructed and not yet destroyed.
/// - panics: aborts when any operation observes an object away from the
///   address it was constructed at.
struct Tracked final
{
  Tracked* self;
  std::int32_t value = 0;

  static inline std::int32_t live = 0;
  static inline bool fail_next = false;

  Tracked()
    : self(this)
  {
    if (fail_next) {
      fail_next = false;
      throw std::runtime_error("refused");
    }
    ++live;
  }

  Tracked(Tracked const& that) noexcept
    : self(this), value(that.value)
  {
    that.check();
    ++live;
  }

  Tracked(Tracked&& that) noexcept
    : self(this), value(that.value)
  {
    that.check();
    that.value = -1;
    ++live;
  }

  auto
  operator=(Tracked const& that) -> Tracked&
  {
    check();
    that.check();
    if (that.value < 0) {
      throw std::invalid_argument("copy from a moved-from object");
    }
    value = that.value;
    return *this;
  }

  auto
  operator=(Tracked&& that) noexcept -> Tracked&
  {
    check();
    that.check();
    value = that.value;
    that.value = -1;
    return *this;
  }

  ~Tracked()
  {
    check();
    --live;
  }

  void
  check() const noexcept
  {
    if (self != this) {
      std::abort();
    }
  }
};

/// An owning handle whose author declares it relocatable: it holds no pointer
/// into itself, so moving its bytes and abandoning the old storage is sound.
///
/// # Specification
/// - provides: `noexcept` default and move construction and destruction.
/// - ensures: `live` counts handles that own their allocation.
/// - panics: none.
struct Handle
{
  std::int32_t* owned = nullptr;

  static inline std::int32_t live = 0;

  Handle() noexcept
    : owned(new std::int32_t(0))
  {
    ++live;
  }

  Handle(Handle const&) = delete;
  auto
  operator=(Handle const&) -> Handle& = delete;

  Handle(Handle&& that) noexcept
    : owned(that.owned)
  {
    that.owned = nullptr;
  }

  auto
  operator=(Handle&&) -> Handle& = delete;

  ~Handle()
  {
    if (owned != nullptr) {
      delete owned;
      --live;
    }
  }
};

[[nodiscard]]
inline auto
tracked_live() noexcept -> std::int32_t
{
  return Tracked::live;
}

inline auto
tracked_fail_next() noexcept -> void
{
  Tracked::fail_next = true;
}

[[nodiscard]]
inline auto
tracked_value(Tracked const& tracked) noexcept -> std::int32_t
{
  tracked.check();
  return tracked.value;
}

inline auto
tracked_set(Tracked& tracked, std::int32_t value) noexcept -> void
{
  tracked.check();
  tracked.value = value;
}

[[nodiscard]]
inline auto
handle_live() noexcept -> std::int32_t
{
  return Handle::live;
}

/// Every special member can throw; failed construction must not create a live object.
/// # Specification
/// - ensures: live counts constructed objects not yet destroyed.
/// - provides: throwing constructors and assignments with a strong fixture guarantee.
/// - fails: consumes an injected standard or unknown exception before changing objects.
/// - panics: none.
/// # Adequacy
/// - hypothesis: failed construction must neither register nor destroy an object;
///   failed assignment must preserve the source and destination.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
struct Throwing final
{
  std::int32_t value = 7;
  static inline std::int32_t live = 0;
  static inline std::int32_t failure = 0;

  /// Consume the next injected special-member failure.
  ///
  /// # Specification
  /// - ensures: clears failure.
  /// - fails: throws runtime_error for 1, an integer for 2, otherwise completes.
  /// - panics: none.
  static auto
  check_failure() -> void
  {
    auto next = failure;
    failure = 0;
    if (next == 1) {
      throw std::runtime_error("special member refused");
    }
    if (next == 2) {
      throw 42;
    }
  }

  /// Construct a live default payload or throw before registration.
  ///
  /// # Specification
  /// - ensures: success sets value to 7 and increments live once.
  /// - fails: injected failure throws without incrementing live.
  /// - panics: none.
  Throwing()
  {
    check_failure();
    ++live;
  }

  /// Copy-construct a live payload.
  ///
  /// # Specification
  /// - ensures: success copies value and increments live once; the source is unchanged.
  /// - fails: injected failure throws without incrementing live.
  /// - panics: none.
  Throwing(Throwing const& that)
    : value(that.value)
  {
    check_failure();
    ++live;
  }

  /// Move-construct a live payload.
  ///
  /// # Specification
  /// - ensures: success copies value, marks the source -1, and increments live once.
  /// - fails: injected failure throws without changing the source or live.
  /// - panics: none.
  Throwing(Throwing&& that)
    : value(that.value)
  {
    check_failure();
    that.value = -1;
    ++live;
  }

  /// Copy-assign the payload.
  ///
  /// # Specification
  /// - ensures: success copies value without changing the source or live.
  /// - provides: this object.
  /// - fails: injected failure throws without changing either object.
  /// - panics: none.
  auto
  operator=(Throwing const& that) -> Throwing&
  {
    check_failure();
    value = that.value;
    return *this;
  }

  /// Move-assign the payload.
  ///
  /// # Specification
  /// - ensures: success copies value and marks the source -1, without changing live.
  /// - provides: this object.
  /// - fails: injected failure throws without changing either object.
  /// - panics: none.
  auto
  operator=(Throwing&& that) -> Throwing&
  {
    check_failure();
    value = that.value;
    that.value = -1;
    return *this;
  }

  /// Unregister one live object.
  ///
  /// # Specification
  /// - ensures: decrements live once.
  /// - panics: none.
  ~Throwing() { --live; }
};

/// Read the constructed-object census.
///
/// # Specification
/// - provides: the current Throwing live count.
/// - panics: none.
inline auto
throwing_live() noexcept -> std::int32_t
{
  return Throwing::live;
}

/// Select the next special-member failure.
///
/// # Specification
/// - ensures: the next special member consumes the requested failure code.
/// - panics: none.
inline auto
throwing_fail_next(std::int32_t failure) noexcept -> void
{
  Throwing::failure = failure;
}

/// Read a Throwing object's payload.
///
/// # Specification
/// - provides: the current value.
/// - panics: none.
inline auto
throwing_value(Throwing const& object) noexcept -> std::int32_t
{
  return object.value;
}
} // namespace demo
