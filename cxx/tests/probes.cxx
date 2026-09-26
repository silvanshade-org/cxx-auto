#include "cxx-auto.hxx"

#include <compare>
#include <cstdint>
#include <cstdlib>
#include <iterator>
#include <limits>
#include <utility>
#include <vector>

namespace {
struct ConstEqual {
  int value;
  [[maybe_unused]] friend constexpr auto operator==(ConstEqual const& lhs, ConstEqual const& rhs) noexcept -> bool
  {
    return lhs.value == rhs.value;
  }
};

struct MutableOnly {
  auto operator==(MutableOnly&) noexcept -> bool { return true; }
};

struct BoolProxy {
  struct Result {
    explicit operator bool() const noexcept { return true; }
  };
  [[maybe_unused]] friend auto operator==(BoolProxy const&, BoolProxy const&) noexcept -> Result { return {}; }
};

struct WeaklyOrdered {
  int value;
  friend constexpr auto operator<=>(WeaklyOrdered const& lhs, WeaklyOrdered const& rhs) noexcept -> std::weak_ordering
  {
    if (lhs.value < rhs.value) {
      return std::weak_ordering::less;
    }
    if (rhs.value < lhs.value) {
      return std::weak_ordering::greater;
    }
    return std::weak_ordering::equivalent;
  }
};

struct NonOrdering {
  [[maybe_unused]] friend constexpr auto operator<=>(NonOrdering const&, NonOrdering const&) noexcept -> int { return 0; }
};

struct LegacyPartial {
  double value;
  friend constexpr auto operator<(LegacyPartial const& lhs, LegacyPartial const& rhs) noexcept -> bool
  {
    return lhs.value < rhs.value;
  }
  friend constexpr auto operator==(LegacyPartial const& lhs, LegacyPartial const& rhs) noexcept -> bool
  {
    return lhs.value == rhs.value;
  }
};

static_assert(cxx_auto::detection::has_operator_equal<ConstEqual>);
static_assert(!cxx_auto::detection::has_operator_equal<MutableOnly>);
static_assert(!cxx_auto::detection::has_operator_equal<BoolProxy>);
static_assert(cxx_auto::detection::has_operator_three_way_comparison<int>);
static_assert(cxx_auto::detection::has_operator_three_way_comparison<WeaklyOrdered>);
static_assert(cxx_auto::detection::has_operator_three_way_comparison<double>);
static_assert(!cxx_auto::detection::has_operator_three_way_comparison<NonOrdering>);
static_assert(cxx_auto::detection::is_input_copy_iterator<std::vector<int>::iterator>);
static_assert(!cxx_auto::detection::is_input_copy_iterator<std::vector<int>::const_iterator>);
static_assert(cxx_auto::detection::is_input_move_iterator<std::move_iterator<std::vector<int>::iterator>>);
static_assert(!cxx_auto::detection::is_input_move_iterator<std::vector<int>::iterator>);
} // namespace

namespace fixture {
struct Value {
  int value = 0;
  friend constexpr auto operator==(Value const& lhs, Value const& rhs) noexcept -> bool
  {
    return lhs.value == rhs.value;
  }
};
} // namespace fixture

namespace fixture::proxy {
CXX_AUTO_PRELUDE(Value, ::fixture::Value)
} // namespace fixture::proxy

int main()
{
  constexpr WeaklyOrdered less{ 1 };
  constexpr WeaklyOrdered greater{ 2 };
  if (cxx_auto::cxx_operator_three_way_comparison(less, greater) != -1 ||
      cxx_auto::cxx_operator_three_way_comparison(greater, less) != 1 ||
      cxx_auto::cxx_operator_three_way_comparison(less, less) != 0) {
    return EXIT_FAILURE;
  }

  const double nan = std::numeric_limits<double>::quiet_NaN();
  if (cxx_auto::cxx_operator_three_way_comparison(nan, 1.0) != std::numeric_limits<std::int8_t>::max()) {
    return EXIT_FAILURE;
  }

  const LegacyPartial unordered{ nan };
  const LegacyPartial one{ 1.0 };
  const LegacyPartial two{ 2.0 };
  if (cxx_auto::cxx_operator_three_way_comparison(unordered, one) != std::numeric_limits<std::int8_t>::max() ||
      cxx_auto::cxx_operator_three_way_comparison(one, two) != -1 ||
      cxx_auto::cxx_operator_three_way_comparison(two, one) != 1 ||
      cxx_auto::cxx_operator_three_way_comparison(one, one) != 0) {
    return EXIT_FAILURE;
  }

  const fixture::Value value{ 3 };
  const fixture::Value other{ 4 };
  if (fixture::proxy::cxx_abi_size() != sizeof(fixture::Value) ||
      !fixture::proxy::cxx_operator_equal(value, value) || fixture::proxy::cxx_operator_equal(value, other)) {
    return EXIT_FAILURE;
  }
  return EXIT_SUCCESS;
}
