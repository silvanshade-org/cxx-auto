#include "cxx-auto.hxx"

#include <compare>
#include <cstdint>
#include <cstdlib>
#include <iterator>
#include <limits>
#include <utility>
#include <vector>

namespace {
struct ConstEqual
{
  int value;

  [[maybe_unused]]
  friend constexpr auto
  operator==(ConstEqual const& lhs, ConstEqual const& rhs) noexcept -> bool
  {
    return lhs.value == rhs.value;
  }
};

struct MutableOnly
{
  auto
  operator==(MutableOnly& /*unused*/) noexcept -> bool
  {
    return true;
  }
};

struct BoolProxy
{
  struct Result
  {
    explicit
    operator bool() const noexcept
    {
      return true;
    }
  };

  [[maybe_unused]]
  friend auto
  operator==(BoolProxy const& /*unused*/, BoolProxy const& /*unused*/) noexcept -> Result
  {
    return {};
  }
};

struct WeaklyOrdered
{
  int value;

  friend constexpr auto
  operator<=>(WeaklyOrdered const& lhs, WeaklyOrdered const& rhs) noexcept -> std::weak_ordering
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

struct NonOrdering
{
  [[maybe_unused]]
  friend constexpr auto
  operator<=>(NonOrdering const& /*unused*/, NonOrdering const& /*unused*/) noexcept -> int
  {
    return 0;
  }
};

struct LegacyPartial
{
  double value;

  friend constexpr auto
  operator<(LegacyPartial const& lhs, LegacyPartial const& rhs) noexcept -> bool
  {
    return lhs.value < rhs.value;
  }

  friend constexpr auto
  operator==(LegacyPartial const& lhs, LegacyPartial const& rhs) noexcept -> bool
  {
    return lhs.value == rhs.value;
  }
};

struct Polymorphic
{
  virtual ~Polymorphic() = default;
  Polymorphic() = default;
  Polymorphic(Polymorphic const&) = default;
  Polymorphic(Polymorphic&&) = default;
  auto
  operator=(Polymorphic const&) -> Polymorphic& = default;
  auto
  operator=(Polymorphic&&) -> Polymorphic& = default;
};

struct FinalValue final
{
  int value = 0;
};

struct AssignableBase
{
  int value = 0;
};

class OwningPointer
{
public:
  OwningPointer() = default;
  OwningPointer(OwningPointer const&) = delete;
  auto
  operator=(OwningPointer const&) -> OwningPointer& = delete;

  OwningPointer(OwningPointer&& that) noexcept
    : owned(std::exchange(that.owned, nullptr))
  {
  }

  auto
  operator=(OwningPointer&&) -> OwningPointer& = delete;

  ~OwningPointer() { delete owned; }

private:
  int* owned = nullptr;
};

class RelocatablePointer
{
public:
  RelocatablePointer() = default;
  RelocatablePointer(RelocatablePointer const&) = delete;
  auto
  operator=(RelocatablePointer const&) -> RelocatablePointer& = delete;

  RelocatablePointer(RelocatablePointer&& that) noexcept
    : owned(std::exchange(that.owned, nullptr))
  {
  }

  auto
  operator=(RelocatablePointer&&) -> RelocatablePointer& = delete;

  ~RelocatablePointer() { delete owned; }

private:
  int* owned = nullptr;
};
} // namespace

template<>
inline constexpr bool cxx_auto::rust_relocatable<RelocatablePointer> = true;
template<>
inline constexpr bool cxx_auto::rust_assignable<AssignableBase> = true;

namespace {

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
static_assert(cxx_auto::detection::has_strong_ordering<int>);
static_assert(!cxx_auto::detection::has_strong_ordering<WeaklyOrdered>);
static_assert(!cxx_auto::detection::has_strong_ordering<double>);
static_assert(!cxx_auto::rust_should_impl_eq<double>(), "a partial order does not claim Eq");
static_assert(!cxx_auto::rust_should_impl_ord<WeaklyOrdered>(), "a weak order does not claim Ord");
static_assert(cxx_auto::rust_should_impl_partial_ord<WeaklyOrdered>(), "any <=> yields PartialOrd");
static_assert(cxx_auto::rust_should_impl_partial_eq<LegacyPartial>());
static_assert(!cxx_auto::rust_should_impl_eq<LegacyPartial>(), "== alone does not claim Eq");
static_assert(!cxx_auto::rust_should_impl_send<ConstEqual>() && !cxx_auto::rust_should_impl_sync<ConstEqual>());
static_assert(
  std::is_copy_assignable_v<Polymorphic> && !cxx_auto::rust_should_impl_copy_assign<Polymorphic>(),
  "a polymorphic type's assignment would slice a more-derived object"
);
static_assert(!cxx_auto::rust_should_impl_move_assign<Polymorphic>());
static_assert(
  std::is_copy_assignable_v<ConstEqual> && !cxx_auto::rust_should_impl_copy_assign<ConstEqual>(),
  "a non-final type may be the base of a class with invariants over its state"
);
static_assert(cxx_auto::rust_should_impl_copy_assign<FinalValue>(), "nothing derives from a final class");
static_assert(cxx_auto::rust_should_impl_move_assign<FinalValue>());
static_assert(cxx_auto::rust_should_impl_copy_assign<AssignableBase>(), "the author's assignment opt-in claims it");
static_assert(!cxx_auto::rust_should_impl_unpin<OwningPointer>(), "a user-provided move is not relocation");
static_assert(cxx_auto::rust_should_impl_unpin<RelocatablePointer>(), "the author's relocation opt-in claims Unpin");
} // namespace

namespace fixture {
struct Value // NOLINT(misc-use-internal-linkage): the exported FFI proxy names fixture::Value.
{
  int value = 0;

  friend constexpr auto
  operator==(Value const& lhs, Value const& rhs) noexcept -> bool
  {
    return lhs.value == rhs.value;
  }
};
} // namespace fixture

// Opt-ins declared beside the type, before the export reads them.
template<>
inline constexpr bool cxx_auto::rust_send<fixture::Value> = true;
template<>
inline constexpr bool cxx_auto::rust_eq<fixture::Value> = true;

namespace fixture::proxy {
// NOLINTNEXTLINE(misc-use-anonymous-namespace): macro generates named FFI proxy helpers.
CXX_AUTO_PRELUDE(Value, ::fixture::Value)
} // namespace fixture::proxy

CXX_AUTO_EXPORT(
  value,
  fixture::proxy,
  .rust_path = "fixture::value",
  .rust_name = "Value",
  .rust_lifetimes = "'a",
  .cxx_name = "Value",
  .cxx_namespace = "fixture",
  .cxx_proxy_include = "fixture.hxx"
)

namespace {
constexpr auto
has_bit(std::uint64_t flags, unsigned bit) -> bool
{
  return ((flags >> bit) & 1U) != 0;
}

constexpr std::string_view value_spec{ "fixture::value\0Value\0'a\0Value\0fixture\0fixture::proxy\0fixture.hxx\0", 65 };
static_assert(cxx_auto_type_value.magic == cxx_auto::record_magic);
static_assert(cxx_auto_type_value.version == cxx_auto::record_version);
static_assert(cxx_auto_type_value.abi_size == sizeof(fixture::Value));
static_assert(cxx_auto_type_value.abi_align == alignof(fixture::Value));
static_assert(cxx_auto_type_value.spec_len == value_spec.size());
static_assert(std::string_view{ cxx_auto_type_value.spec.data(), cxx_auto_type_value.spec.size() } == value_spec);
static_assert(has_bit(cxx_auto_type_value.flags, cxx_auto::record_bit::send), "the rust_send specialization is read");
static_assert(!has_bit(cxx_auto_type_value.flags, cxx_auto::record_bit::sync));
static_assert(has_bit(cxx_auto_type_value.flags, cxx_auto::record_bit::eq), "the rust_eq specialization is read");
static_assert(has_bit(cxx_auto_type_value.flags, cxx_auto::record_bit::partial_eq));
static_assert(!has_bit(cxx_auto_type_value.flags, cxx_auto::record_bit::ord));
} // namespace

auto
main() -> int
{
  constexpr WeaklyOrdered less{ 1 };
  constexpr WeaklyOrdered greater{ 2 };
  if (
    cxx_auto::cxx_operator_three_way_comparison(less, greater) != cxx_auto::Comparison::Less
    || cxx_auto::cxx_operator_three_way_comparison(greater, less) != cxx_auto::Comparison::Greater
    || cxx_auto::cxx_operator_three_way_comparison(less, less) != cxx_auto::Comparison::Equivalent) {
    return EXIT_FAILURE;
  }

  double const nan = std::numeric_limits<double>::quiet_NaN();
  if (cxx_auto::cxx_operator_three_way_comparison(nan, 1.0) != cxx_auto::Comparison::Unordered) {
    return EXIT_FAILURE;
  }

  LegacyPartial const unordered{ nan };
  LegacyPartial const one{ 1.0 };
  LegacyPartial const two{ 2.0 };
  if (
    cxx_auto::cxx_operator_three_way_comparison(unordered, one) != cxx_auto::Comparison::Unordered
    || cxx_auto::cxx_operator_three_way_comparison(one, two) != cxx_auto::Comparison::Less
    || cxx_auto::cxx_operator_three_way_comparison(two, one) != cxx_auto::Comparison::Greater
    || cxx_auto::cxx_operator_three_way_comparison(one, one) != cxx_auto::Comparison::Equivalent) {
    return EXIT_FAILURE;
  }

  fixture::Value const value{ 3 };
  fixture::Value const other{ 4 };
  if (
    fixture::proxy::cxx_operator_equal(value, value) != cxx_auto::Equality::Equal
    || fixture::proxy::cxx_operator_equal(value, other) != cxx_auto::Equality::NotEqual) {
    return EXIT_FAILURE;
  }
  return EXIT_SUCCESS;
}
