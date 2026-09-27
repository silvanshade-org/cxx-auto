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
    : self(this)
    , value(that.value)
  {
    that.check();
    ++live;
  }

  Tracked(Tracked&& that) noexcept
    : self(this)
    , value(that.value)
  {
    that.check();
    that.value = -1;
    ++live;
  }

  auto operator=(Tracked const& that) -> Tracked&
  {
    check();
    that.check();
    if (that.value < 0) {
      throw std::invalid_argument("copy from a moved-from object");
    }
    value = that.value;
    return *this;
  }

  auto operator=(Tracked&& that) noexcept -> Tracked&
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

  void check() const noexcept
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
  auto operator=(Handle const&) -> Handle& = delete;

  Handle(Handle&& that) noexcept
    : owned(that.owned)
  {
    that.owned = nullptr;
  }

  auto operator=(Handle&&) -> Handle& = delete;

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
} // namespace demo
