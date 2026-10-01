#ifndef CXX_AUTO_RESULTS_HXX
#define CXX_AUTO_RESULTS_HXX

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace cxx_auto {
/// Whether a catching C++ operation completed or threw.
/// # Specification
/// - provides: Completed = 0 and Threw = 1 with the Rust u8 representation.
/// - panics: none.
enum class Completion : std::uint8_t
{
  Completed = 0,
  Threw = 1,
};

/// The equality or inequality classification sent across CXX.
/// # Specification
/// - provides: Equal = 0 and NotEqual = 1 with the Rust u8 representation.
/// - panics: none.
enum class Equality : std::uint8_t
{
  Equal = 0,
  NotEqual = 1,
};

/// The complete three-way comparison classification sent across CXX.
/// # Specification
/// - provides: Less, Equivalent, Greater, Unordered with Rust discriminants 0..3.
/// - panics: none.
enum class Comparison : std::uint8_t
{
  Less = 0,
  Equivalent = 1,
  Greater = 2,
  Unordered = 3,
};

/// The hash word sent across CXX.
/// # Specification
/// - provides: a single std::size_t word matching Rust's transparent usize wrapper.
/// - panics: none.
struct HashValue
{
  std::size_t value;
};

static_assert(sizeof(Completion) == sizeof(std::uint8_t));
static_assert(alignof(Completion) == alignof(std::uint8_t));
static_assert(sizeof(Equality) == sizeof(std::uint8_t));
static_assert(alignof(Equality) == alignof(std::uint8_t));
static_assert(sizeof(Comparison) == sizeof(std::uint8_t));
static_assert(alignof(Comparison) == alignof(std::uint8_t));
static_assert(sizeof(HashValue) == sizeof(std::size_t));
static_assert(alignof(HashValue) == alignof(std::size_t));
static_assert(std::is_trivially_copyable_v<Completion>);
static_assert(std::is_trivially_copyable_v<Equality>);
static_assert(std::is_trivially_copyable_v<Comparison>);
static_assert(std::is_trivially_copyable_v<HashValue>);
} // namespace cxx_auto

#endif // CXX_AUTO_RESULTS_HXX
