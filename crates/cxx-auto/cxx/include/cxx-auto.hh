#pragma once

#include "rust/cxx.h"
#include "sys/types.h"

#include <compare>
#include <concepts>
#include <initializer_list>
#include <iterator>
#include <limits>
#include <memory>
#include <ranges>
#include <sstream>
#include <type_traits>

#ifdef CXX_AUTO_CTYPES
// NOLINTBEGIN(google-runtime-int)
using c_char = char;
using c_int = int;
using c_long = long;
using c_longlong = long long;
using c_schar = signed char;
using c_short = short;
using c_uchar = unsigned char;
using c_uint = unsigned int;
using c_ulong = unsigned long;
using c_ulonglong = unsigned long long;
using c_ushort = unsigned short;
using c_void = void;
using c_off_t = off_t;
using c_time_t = time_t;
// NOLINTEND(google-runtime-int)
#endif

namespace cxx_auto::detection {
template<typename T, typename... U>
concept same_as_any_of = (::std::same_as<T, U> or ...);

template<typename T>
concept has_operator_equal = requires(T const& lhs, T const& rhs) {
  { lhs == rhs } -> ::std::same_as<bool>;
};

template<typename T>
concept has_operator_not_equal = requires(T const& lhs, T const& rhs) {
  { lhs != rhs } -> ::std::same_as<bool>;
};

template<typename T>
concept has_operator_less_than = requires(T const& lhs, T const& rhs) {
  { lhs < rhs } -> ::std::same_as<bool>;
};

template<typename T>
concept has_operator_less_than_or_equal = requires(T const& lhs, T const& rhs) {
  { lhs <= rhs } -> ::std::same_as<bool>;
};

template<typename T>
concept has_operator_greater_than = requires(T const& lhs, T const& rhs) {
  { lhs > rhs } -> ::std::same_as<bool>;
};

template<typename T>
concept has_operator_greater_than_or_equal = requires(T const& lhs, T const& rhs) {
  { lhs >= rhs } -> ::std::same_as<bool>;
};

template<typename T>
concept has_operator_three_way_comparison = requires(T const& lhs, T const& rhs) {
  requires same_as_any_of<decltype(lhs <=> rhs), ::std::partial_ordering, ::std::strong_ordering>;
};

template<typename T>
concept is_std_hashable = requires(T const& arg) {
  { ::std::hash<T> {}(arg) } -> ::std::same_as<::std::size_t>;
};

template<typename T>
concept has_operator_std_string = requires(T const& arg) {
  { arg.operator ::std::string() } -> ::std::same_as<::std::string>;
};

template<typename T>
concept has_operator_std_string_view = requires(T const& arg) {
  { arg.operator ::std::string_view() } -> ::std::same_as<::std::string_view>;
};

template<typename T>
concept has_to_string = requires(T const& arg) {
  { ::std::to_string(arg) } -> ::std::same_as<::std::string>;
};

template<typename T>
concept has_operator_ostream_left_shift = requires(T const& arg, ::std::ostream& os) {
  { os << arg } -> ::std::same_as<::std::ostream&>;
};

template<typename T, typename It>
concept is_constructible_from_iterator = requires(It first, It last) {
  requires ::std::input_iterator<It>;
  { T { first, last } } -> ::std::same_as<T>;
};

template<typename T>
concept is_iterable = ::std::ranges::range<T>;

template<typename T, typename V>
concept is_input_iterable = requires {
  requires ::std::ranges::input_range<T>;
  requires ::std::same_as<::std::iter_value_t<::std::ranges::iterator_t<T>>, ::std::remove_reference_t<V>>;
};

template<typename T>
concept is_input_copy_iterator = requires {
  requires ::std::input_iterator<T>;
  requires ::std::same_as<::std::iter_reference_t<T>, ::std::add_lvalue_reference_t<::std::iter_value_t<T>>>;
};

template<typename T>
concept is_input_move_iterator = requires {
  requires ::std::input_iterator<T>;
  requires ::std::same_as<::std::iter_reference_t<T>, ::std::add_rvalue_reference_t<::std::iter_value_t<T>>>;
};
} // namespace cxx_auto::detection

namespace cxx_auto::satisfy {
template<typename T, typename... Args>
concept cxx_is_constructible = ::std::is_constructible_v<T, Args...>;

template<typename T>
concept cxx_is_default_constructible = ::std::is_default_constructible_v<T>;

template<typename T>
concept cxx_is_copy_constructible = ::std::is_copy_constructible_v<T>;

template<typename T>
concept cxx_is_move_constructible = ::std::is_move_constructible_v<T>;

template<typename T>
concept cxx_is_destructible = ::std::is_destructible_v<T>;

template<typename T>
concept cxx_is_trivially_copyable = ::std::is_trivially_copyable_v<T>;

template<typename T>
concept cxx_is_trivially_movable = ::std::is_trivially_move_constructible_v<T>
                               and ::std::is_trivially_destructible_v<T>;

template<typename T>
concept cxx_is_trivially_destructible = ::std::is_trivially_destructible_v<T>;

template<typename T>
concept cxx_is_equality_comparable = ::std::equality_comparable<T>;

template<typename T>
concept cxx_has_operator_equal = detection::has_operator_equal<T>;

template<typename T>
concept cxx_has_operator_not_equal = detection::has_operator_not_equal<T>;

template<typename T>
concept cxx_has_operator_less_than = detection::has_operator_less_than<T>;

template<typename T>
concept cxx_has_operator_less_than_or_equal = detection::has_operator_less_than_or_equal<T>;

template<typename T>
concept cxx_has_operator_greater_than = detection::has_operator_greater_than<T>;

template<typename T>
concept cxx_has_operator_greater_than_or_equal = detection::has_operator_greater_than_or_equal<T>;

template<typename T>
concept cxx_has_operator_three_way_comparison = detection::has_operator_three_way_comparison<T>
                                             or (not detection::has_operator_three_way_comparison<T>
                                                 and detection::has_operator_less_than<T>
                                                 and detection::has_operator_equal<T>);

template<typename T>
concept cxx_is_partially_ordered = cxx_has_operator_three_way_comparison<T>;

template<typename T>
concept cxx_is_totally_ordered = ::std::totally_ordered<T>;

template<typename T>
concept cxx_is_hashable = detection::is_std_hashable<T>;

template<typename T>
concept cxx_is_debuggable = detection::has_operator_ostream_left_shift<T>;

template<typename T>
concept cxx_is_displayable = detection::has_operator_std_string_view<T>
                          or detection::has_operator_std_string<T>
                          or detection::has_to_string<T>;
} // namespace cxx_auto::satisfy

namespace cxx_auto::derive {
template<typename T>
concept rust_should_impl_cxx_extern_type_trivial = satisfy::cxx_is_trivially_movable<T>;

template<typename T>
concept rust_should_impl_unpin = satisfy::cxx_is_trivially_movable<T>;

template<typename T>
concept rust_should_impl_send = false;

template<typename T>
concept rust_should_impl_sync = false;

template<typename T>
concept rust_should_impl_drop = satisfy::cxx_is_destructible<T>
                            and not satisfy::cxx_is_trivially_destructible<T>;

template<typename T>
concept rust_should_impl_copy = satisfy::cxx_is_trivially_copyable<T>
                            and satisfy::cxx_is_trivially_movable<T>
                            and not rust_should_impl_drop<T>;

template<typename T>
concept rust_should_impl_default = satisfy::cxx_is_default_constructible<T>;

template<typename T>
concept rust_should_impl_moveref_copy_new = satisfy::cxx_is_copy_constructible<T>;

template<typename T>
concept rust_should_impl_moveref_move_new = satisfy::cxx_is_move_constructible<T>;

template<typename T>
concept rust_should_impl_eq = satisfy::cxx_is_equality_comparable<T>;

template<typename T>
concept rust_should_impl_partial_eq = satisfy::cxx_has_operator_equal<T>;

template<typename T>
concept rust_should_impl_partial_ord = satisfy::cxx_has_operator_three_way_comparison<T>
                                    or (satisfy::cxx_has_operator_less_than<T>
                                        and satisfy::cxx_has_operator_equal<T>);

template<typename T>
concept rust_should_impl_ord = satisfy::cxx_is_totally_ordered<T>;

template<typename T>
concept rust_should_impl_hash = satisfy::cxx_is_hashable<T>;

template<typename T>
concept rust_should_impl_debug = satisfy::cxx_is_debuggable<T>;

template<typename T>
concept rust_should_impl_display = satisfy::cxx_is_displayable<T>;
} // namespace cxx_auto::derive

namespace cxx_auto {
struct alignas(32) TypeSpecLifetimeFFI
{
  char8_t const* name;
  char8_t const* const* bounds_data;
  size_t bounds_len;
};
static_assert(derive::rust_should_impl_cxx_extern_type_trivial<TypeSpecLifetimeFFI>);

// NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
struct alignas(32) TypeSpecLifetime
{
  char8_t const* name;
  std::initializer_list<char8_t const*> bounds = {}; // NOLINT(readability-redundant-member-init)
};

// NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
struct alignas(64) TypeSpec
{
  char8_t const* cc_name;
  char8_t const* cc_namespace;
  char8_t const* rs_name = cc_name;
  char8_t const* rs_namespace = cc_namespace;
  std::initializer_list<TypeSpecLifetime> rs_lifetimes = {}; // NOLINT(readability-redundant-member-init)
};

struct alignas(64) TypeSpecFFI
{
  char8_t const* cc_name;
  char8_t const* cc_namespace;
  char8_t const* rs_name;
  char8_t const* rs_namespace;
  TypeSpecLifetimeFFI const* rs_lifetimes_data;
  size_t rs_lifetimes_len;
};
static_assert(derive::rust_should_impl_cxx_extern_type_trivial<TypeSpecFFI>);

template<size_t len>
struct alignas(128) TypeSpecStorage
{
  explicit consteval TypeSpecStorage(TypeSpec const& init)
    : cc_name(init.cc_name)
    , cc_namespace(init.cc_namespace)
    , rs_name(init.rs_name)
    , rs_namespace(init.rs_namespace)
    , rs_lifetimes(init.rs_lifetimes)
  {
    std::transform(
      rs_lifetimes.begin(),
      rs_lifetimes.end(),
      rs_lifetimes_ffi.begin(),
      [&](TypeSpecLifetime const& elem) -> TypeSpecLifetimeFFI {
        return {
          .name = elem.name,
          .bounds_data = elem.bounds.begin(),
          .bounds_len = elem.bounds.size(),
        };
      });
  }

  explicit consteval operator TypeSpecFFI() const
  {
    return {
      .cc_name = this->cc_name,
      .cc_namespace = this->cc_namespace,
      .rs_name = this->rs_name,
      .rs_namespace = this->rs_namespace,
      .rs_lifetimes_data = this->rs_lifetimes_ffi.data(),
      .rs_lifetimes_len = this->rs_lifetimes_ffi.size(),
    };
  }

private:
  char8_t const* cc_name;
  char8_t const* cc_namespace;
  char8_t const* rs_name = cc_name;
  char8_t const* rs_namespace = cc_namespace;
  std::initializer_list<TypeSpecLifetime> rs_lifetimes;
  std::array<TypeSpecLifetimeFFI, len> rs_lifetimes_ffi;
};

struct alignas(32) TypeElabFFI
{
  size_t cxx_abi_align;
  size_t cxx_abi_size;
  bool rust_should_impl_cxx_extern_type_trivial;
  bool rust_should_impl_unpin;
  bool rust_should_impl_send;
  bool rust_should_impl_sync;
  bool rust_should_impl_drop;
  bool rust_should_impl_copy;
  bool rust_should_impl_default;
  bool rust_should_impl_moveref_copy_new;
  bool rust_should_impl_moveref_move_new;
  bool rust_should_impl_eq;
  bool rust_should_impl_partial_eq;
  bool rust_should_impl_partial_ord;
  bool rust_should_impl_ord;
  bool rust_should_impl_hash;
  bool rust_should_impl_debug;
  bool rust_should_impl_display;

  template<typename Self>
  static consteval auto elab() -> TypeElabFFI
  {
    return {
      .cxx_abi_align = alignof(Self),
      .cxx_abi_size = sizeof(Self),
      .rust_should_impl_cxx_extern_type_trivial = ::cxx_auto::derive::rust_should_impl_cxx_extern_type_trivial<Self>,
      .rust_should_impl_unpin = ::cxx_auto::derive::rust_should_impl_unpin<Self>,
      .rust_should_impl_send = ::cxx_auto::derive::rust_should_impl_send<Self>,
      .rust_should_impl_sync = ::cxx_auto::derive::rust_should_impl_sync<Self>,
      .rust_should_impl_drop = ::cxx_auto::derive::rust_should_impl_drop<Self>,
      .rust_should_impl_copy = ::cxx_auto::derive::rust_should_impl_copy<Self>,
      .rust_should_impl_default = ::cxx_auto::derive::rust_should_impl_default<Self>,
      .rust_should_impl_moveref_copy_new = ::cxx_auto::derive::rust_should_impl_moveref_copy_new<Self>,
      .rust_should_impl_moveref_move_new = ::cxx_auto::derive::rust_should_impl_moveref_move_new<Self>,
      .rust_should_impl_eq = ::cxx_auto::derive::rust_should_impl_eq<Self>,
      .rust_should_impl_partial_eq = ::cxx_auto::derive::rust_should_impl_partial_eq<Self>,
      .rust_should_impl_partial_ord = ::cxx_auto::derive::rust_should_impl_partial_ord<Self>,
      .rust_should_impl_ord = ::cxx_auto::derive::rust_should_impl_ord<Self>,
      .rust_should_impl_hash = ::cxx_auto::derive::rust_should_impl_hash<Self>,
      .rust_should_impl_debug = ::cxx_auto::derive::rust_should_impl_debug<Self>,
      .rust_should_impl_display = ::cxx_auto::derive::rust_should_impl_display<Self>,
    };
  }
};

} // namespace cxx_auto

// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define CXX_AUTO_PRELUDE_HEADER(SELF)                                                               \
  extern "C" ::cxx_auto::TypeSpecFFI const type_spec;                                               \
  extern "C" ::cxx_auto::TypeElabFFI const type_elab;                                               \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void                                \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_default_constructible<T>;                                  \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_copy_new(T* This [[clang::lifetimebound]],                                                    \
               T const& that [[clang::lifetimebound]]) noexcept -> void                             \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_copy_constructible<T>;                                     \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_move_new(T* This [[clang::lifetimebound]], T* that [[clang::lifetimebound]]) noexcept -> void \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_move_constructible<T>;                                     \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_destruct(T* This [[clang::lifetimebound]]) noexcept -> void                                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_destructible<T>;                                           \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_equal(T const& This [[clang::lifetimebound]],                                        \
                     T const& That [[clang::lifetimebound]]) noexcept -> bool                       \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_equal<T>;                                        \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_not_equal(T const& This [[clang::lifetimebound]],                                    \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_not_equal<T>;                                    \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_less_than(T const& This [[clang::lifetimebound]],                                    \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than<T>;                                    \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_less_than_or_equal(T const& This [[clang::lifetimebound]],                           \
                                  T const& That [[clang::lifetimebound]]) noexcept -> bool          \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than_or_equal<T>;                           \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_greater_than(T const& This [[clang::lifetimebound]],                                 \
                            T const& That [[clang::lifetimebound]]) noexcept -> bool                \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than<T>;                                 \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_greater_than_or_equal(T const& This [[clang::lifetimebound]],                        \
                                     T const& That [[clang::lifetimebound]]) noexcept -> bool       \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than_or_equal<T>;                        \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                         \
                                    T const& That [[clang::lifetimebound]]) noexcept -> int8_t      \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_three_way_comparison<T>;                         \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> size_t                               \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_hashable<T>;                                               \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                      \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_debuggable<T>;                                             \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                    \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_displayable<T>;

// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define CXX_AUTO_PRELUDE_SOURCE(SELF, SPEC)                                                               \
  namespace {                                                                                             \
  constexpr auto const type_spec_storage = ::cxx_auto::TypeSpecStorage<(SPEC).rs_lifetimes.size()>(spec); \
  }                                                                                                       \
  constexpr auto const type_spec = ::cxx_auto::TypeSpecFFI(type_spec_storage);                            \
  constexpr ::cxx_auto::TypeElabFFI const type_elab = ::cxx_auto::TypeElabFFI::elab<SELF>();              \
                                                                                                          \
  template<typename T, typename... Args>                                                                  \
  auto                                                                                                    \
  cxx_placement_new(T* This [[clang::lifetimebound]], Args&&... args) noexcept -> void                    \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_constructible<T, Args...>                                        \
  {                                                                                                       \
    new (This) T(::std::forward<Args>(args)...);                                                          \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  auto                                                                                                    \
  cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void                                      \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_default_constructible<T>                                         \
  {                                                                                                       \
    cxx_placement_new(This);                                                                              \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  auto                                                                                                    \
  cxx_copy_new(T* This [[clang::lifetimebound]],                                                          \
               T const& that [[clang::lifetimebound]]) noexcept -> void                                   \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_copy_constructible<T>                                            \
  {                                                                                                       \
    new (This) T(that);                                                                                   \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  auto                                                                                                    \
  cxx_move_new(T* This [[clang::lifetimebound]], T* that [[clang::lifetimebound]]) noexcept -> void       \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_move_constructible<T>                                            \
  {                                                                                                       \
    new (This) T(::std::forward<T>(that));                                                                \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  auto                                                                                                    \
  cxx_destruct(T* This [[clang::lifetimebound]]) noexcept -> void                                         \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_destructible<T>                                                  \
  {                                                                                                       \
    ::std::destroy_at(This);                                                                              \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_operator_equal(T const& This [[clang::lifetimebound]],                                              \
                     T const& That [[clang::lifetimebound]]) noexcept -> bool                             \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_has_operator_equal<T>                                               \
  {                                                                                                       \
    return (This == That);                                                                                \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_operator_not_equal(T const& This [[clang::lifetimebound]],                                          \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                         \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_has_operator_not_equal<T>                                           \
  {                                                                                                       \
    return (This != That);                                                                                \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_operator_less_than(T const& This [[clang::lifetimebound]],                                          \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                         \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than<T>                                           \
  {                                                                                                       \
    return (This < That);                                                                                 \
  }                                                                                                       \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_operator_less_than_or_equal(T const& This [[clang::lifetimebound]],                                 \
                                  T const& That [[clang::lifetimebound]]) noexcept -> bool                \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than_or_equal<T>                                  \
  {                                                                                                       \
    return (This <= That);                                                                                \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_operator_greater_than(T const& This [[clang::lifetimebound]],                                       \
                            T const& That [[clang::lifetimebound]]) noexcept -> bool                      \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than<T>                                        \
  {                                                                                                       \
    return (This > That);                                                                                 \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_operator_greater_than_or_equal(T const& This [[clang::lifetimebound]],                              \
                                     T const& That [[clang::lifetimebound]]) noexcept -> bool             \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than_or_equal<T>                               \
  {                                                                                                       \
    return (This >= That);                                                                                \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]] [[gnu::always_inline]]                                                                    \
  inline static auto                                                                                      \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                               \
                                    T const& That [[clang::lifetimebound]]) noexcept -> int8_t            \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::detection::has_operator_three_way_comparison<T>                                  \
  {                                                                                                       \
    auto result = (This <=> That);                                                                        \
    if (result < 0) {                                                                                     \
      return -1;                                                                                          \
    } else if (result > 0) { /* NOLINT(llvm-else-after-return, readability-else-after-return) */          \
      return 1;                                                                                           \
    } else if (result == 0) {                                                                             \
      return 0;                                                                                           \
    } else {                                                                                              \
      return ::std::numeric_limits<int8_t>::max();                                                        \
    }                                                                                                     \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]] [[gnu::always_inline]]                                                                    \
  inline static auto                                                                                      \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                               \
                                    T const& That [[clang::lifetimebound]]) noexcept -> int8_t            \
    requires ::std::same_as<T, SELF>                                                                      \
         and (not ::cxx_auto::detection::has_operator_three_way_comparison<T>)                            \
         and ::cxx_auto::detection::has_operator_less_than<T>                                             \
         and ::cxx_auto::detection::has_operator_equal<T>                                                 \
  {                                                                                                       \
    auto le = (This < That);                                                                              \
    auto eq = (This == That);                                                                             \
    if (le and not eq) {                                                                                  \
      return -1;                                                                                          \
    } else if (not le and not eq) { /* NOLINT(llvm-else-after-return, readability-else-after-return) */   \
      return 1;                                                                                           \
    } else if (not le and eq) {                                                                           \
      return 0;                                                                                           \
    } else {                                                                                              \
      return ::std::numeric_limits<int8_t>::max();                                                        \
    }                                                                                                     \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> size_t                                     \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_hashable<T>                                                      \
  {                                                                                                       \
    return ::std::hash<T> {}(This);                                                                       \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::string                            \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_debuggable<T>                                                    \
  {                                                                                                       \
    ::std::ostringstream os;                                                                              \
    os << This;                                                                                           \
    return ::rust::String::lossy(os.str());                                                               \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                          \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::detection::has_operator_std_string_view<T>                                       \
  {                                                                                                       \
    return ::rust::String::lossy(::std::string { This.operator ::std::string_view() });                   \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                          \
    requires ::std::same_as<T, SELF>                                                                      \
         and (not ::cxx_auto::detection::has_operator_std_string_view<T>)                                 \
         and ::cxx_auto::detection::has_operator_std_string<T>                                            \
  {                                                                                                       \
    return ::rust::String::lossy(This.operator ::std::string());                                          \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                          \
    requires ::std::same_as<T, SELF>                                                                      \
         and (not ::cxx_auto::detection::has_operator_std_string_view<T>)                                 \
         and (not ::cxx_auto::detection::has_operator_std_string<T>)                                      \
         and ::cxx_auto::detection::has_to_string<T>                                                      \
  {                                                                                                       \
    return ::rust::String::lossy(::std::to_string(This));                                                 \
  }                                                                                                       \
                                                                                                          \
  template<typename T>                                                                                    \
  [[nodiscard]]                                                                                           \
  auto                                                                                                    \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::string                          \
    requires ::std::same_as<T, SELF>                                                                      \
         and ::cxx_auto::satisfy::cxx_is_displayable<T>;
