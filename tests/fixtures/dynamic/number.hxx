#pragma once

#include <compare>

namespace demo {
/// A trivially movable C++ value with a total order.
///
/// # Specification
/// - provides: signed integer equality and ordering with no throwing operations.
/// - panics: none.
struct Number
{
  int value = 0;

  constexpr Number() noexcept = default;
  constexpr explicit Number(int number) noexcept
    : value(number)
  {
  }

  friend constexpr auto operator==(Number const&, Number const&) noexcept -> bool = default;
  friend constexpr auto operator<=>(Number const&, Number const&) noexcept = default;
};

/// Construct a number with the requested value.
///
/// # Specification
/// - provides: a Number whose value equals the argument.
/// - panics: none.
[[nodiscard]]
inline auto
make_number(int value) noexcept -> Number
{
  return Number{ value };
}
} // namespace demo
