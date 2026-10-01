#pragma once

#include <compare>
#include <cstdint>
#include <functional>
#include <ostream>
#include <string>

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

  friend constexpr auto
  operator==(Number const&, Number const&) noexcept -> bool = default;
  friend constexpr auto
  operator<=>(Number const&, Number const&) noexcept = default;

  /// Render the signed decimal payload.
  ///
  /// # Specification
  /// - provides: std::to_string(value).
  /// - fails: string allocation may throw.
  /// - panics: none.
  explicit
  operator std::string() const
  {
    return std::to_string(value);
  }

  /// Write the C++ debug representation.
  ///
  /// # Specification
  /// - ensures: writes Number(value) to the stream.
  /// - provides: the same stream.
  /// - fails: propagates stream failures.
  /// - panics: none.
  friend auto
  operator<<(std::ostream& out, Number const& number) -> std::ostream&
  {
    return out << "Number(" << number.value << ')';
  }
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

/// Floating-point spaceship comparisons include an unordered NaN result.
/// # Specification
/// - provides: floating-point equality and partial ordering.
/// - panics: none.
/// # Adequacy
/// - hypothesis: NaN must remain unordered rather than greater.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
struct PartialNumber
{
  double value;
  /// Compare floating-point equality.
  ///
  /// # Specification
  /// - provides: payload equality; NaN is unequal to every value.
  /// - panics: none.
  friend constexpr auto
  operator==(PartialNumber const&, PartialNumber const&) noexcept -> bool = default;
  /// Compare floating-point order.
  ///
  /// # Specification
  /// - provides: partial ordering with NaN unordered.
  /// - panics: none.
  friend constexpr auto
  operator<=>(PartialNumber const&, PartialNumber const&) noexcept = default;
};

/// Equivalent values share a decimal bucket, even when their payloads differ.
/// # Specification
/// - provides: equality and weak ordering by the integer quotient value / 10.
/// - panics: none.
/// # Adequacy
/// - hypothesis: different payloads within one bucket must remain equivalent.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
struct WeakNumber
{
  int value;

  /// Compare decimal-bucket equivalence.
  ///
  /// # Specification
  /// - provides: equality exactly when integer quotient buckets agree.
  /// - panics: none.
  friend constexpr auto
  operator==(WeakNumber lhs, WeakNumber rhs) noexcept -> bool
  {
    return lhs.value / 10 == rhs.value / 10;
  }

  /// Order decimal buckets.
  ///
  /// # Specification
  /// - provides: weak equivalence within a bucket and order between buckets.
  /// - panics: none.
  friend constexpr auto
  operator<=>(WeakNumber lhs, WeakNumber rhs) noexcept -> std::weak_ordering
  {
    return (lhs.value / 10) <=> (rhs.value / 10);
  }
};

/// Legacy equality and less-than must still distinguish unordered NaN.
/// # Specification
/// - provides: floating-point equality and less-than without a spaceship operator.
/// - panics: none.
/// # Adequacy
/// - hypothesis: legacy ordering must preserve unordered and reversed comparisons.
/// - witness: tests/dynamic_binding.rs (loads_generated_comparison_binding).
struct LegacyNumber
{
  double value;

  /// Compare legacy floating-point equality.
  ///
  /// # Specification
  /// - provides: payload equality; NaN is unequal to every value.
  /// - panics: none.
  friend constexpr auto
  operator==(LegacyNumber lhs, LegacyNumber rhs) noexcept -> bool
  {
    return lhs.value == rhs.value;
  }

  /// Compare legacy floating-point less-than.
  ///
  /// # Specification
  /// - provides: strict payload order; false if either payload is NaN.
  /// - panics: none.
  friend constexpr auto
  operator<(LegacyNumber lhs, LegacyNumber rhs) noexcept -> bool
  {
    return lhs.value < rhs.value;
  }
};

/// Wrap a floating-point payload for spaceship ordering.
///
/// # Specification
/// - provides: a PartialNumber with the requested payload.
/// - panics: none.
inline auto
make_partial(double value) noexcept -> PartialNumber
{
  return { value };
}

/// Wrap an integer payload for bucket ordering.
///
/// # Specification
/// - provides: a WeakNumber with the requested payload.
/// - panics: none.
inline auto
make_weak(int value) noexcept -> WeakNumber
{
  return { value };
}

/// Wrap a floating-point payload for legacy ordering.
///
/// # Specification
/// - provides: a LegacyNumber with the requested payload.
/// - panics: none.
inline auto
make_legacy(double value) noexcept -> LegacyNumber
{
  return { value };
}
} // namespace demo

/// Deterministic hash projection for the generated Hash witness.
///
/// # Specification
/// - provides: the unsigned 32-bit payload plus 0x51 as a size_t word.
/// - panics: none.
template<>
struct std::hash<demo::Number>
{
  /// Project the payload to a native hash word.
  ///
  /// # Specification
  /// - provides: the unsigned 32-bit payload plus 0x51.
  /// - panics: none.
  auto
  operator()(demo::Number const& number) const noexcept -> std::size_t
  {
    return static_cast<std::uint32_t>(number.value) + std::size_t{ 0x51 };
  }
};
