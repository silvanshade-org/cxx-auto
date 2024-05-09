#pragma once

#include "rust/cxx.h"
#include "sys/types.h"

#include <compare>
#include <concepts>
#include <iterator>
#include <limits>
#include <memory>
#include <ranges>
#include <sstream>
#include <type_traits>

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

namespace cxx_auto::detection {
template<typename T, typename... U>
concept same_as_any_of = (std::same_as<T, U> or ...);

template<typename T>
concept has_operator_equal = requires(T const& lhs, T const& rhs) {
  { lhs == rhs } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_not_equal = requires(T const& lhs, T const& rhs) {
  { lhs != rhs } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_less_than = requires(T const& lhs, T const& rhs) {
  { lhs < rhs } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_less_than_or_equal = requires(T const& lhs, T const& rhs) {
  { lhs <= rhs } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_greater_than = requires(T const& lhs, T const& rhs) {
  { lhs > rhs } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_greater_than_or_equal = requires(T const& lhs, T const& rhs) {
  { lhs >= rhs } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_three_way_comparison = requires(T const& lhs, T const& rhs) {
  requires same_as_any_of<decltype(lhs <=> rhs), std::partial_ordering, std::strong_ordering>;
};

template<typename T>
concept is_std_hashable = requires(T const& arg) {
  { std::hash<T> {}(arg) } -> std::same_as<std::size_t>;
};

template<typename T>
concept has_operator_std_string = requires(T const& arg) {
  { arg.operator std::string() } -> std::same_as<std::string>;
};

template<typename T>
concept has_operator_std_string_view = requires(T const& arg) {
  { arg.operator std::string_view() } -> std::same_as<std::string_view>;
};

template<typename T>
concept has_to_string = requires(T const& arg) {
  { std::to_string(arg) } -> std::same_as<std::string>;
};

template<typename T>
concept has_operator_ostream_left_shift = requires(T const& arg, std::ostream& os) {
  { os << arg } -> std::same_as<std::ostream&>;
};

template<typename T, typename It>
concept is_constructible_from_iterator = requires(It first, It last) {
  requires std::input_iterator<It>;
  { T { first, last } } -> std::same_as<T>;
};

template<typename T>
concept is_iterable = std::ranges::range<T>;

template<typename T, typename V>
concept is_input_iterable = requires {
  requires std::ranges::input_range<T>;
  requires std::same_as<std::iter_value_t<std::ranges::iterator_t<T>>, std::remove_reference_t<V>>;
};

template<typename T>
concept is_input_copy_iterator = requires {
  requires std::input_iterator<T>;
  requires std::same_as<std::iter_reference_t<T>, std::add_lvalue_reference_t<std::iter_value_t<T>>>;
};

template<typename T>
concept is_input_move_iterator = requires {
  requires std::input_iterator<T>;
  requires std::same_as<std::iter_reference_t<T>, std::add_rvalue_reference_t<std::iter_value_t<T>>>;
};
} // namespace cxx_auto::detection

namespace cxx_auto {
template<typename T, typename... Args>
concept cxx_is_constructible = std::is_constructible_v<T, Args...>;

template<typename T>
concept cxx_is_default_constructible = std::is_default_constructible_v<T>;

template<typename T>
concept cxx_is_copy_constructible = std::is_copy_constructible_v<T>;

template<typename T>
concept cxx_is_move_constructible = std::is_move_constructible_v<T>;

template<typename T>
concept cxx_is_destructible = std::is_destructible_v<T>;

template<typename T>
concept cxx_is_trivially_copyable = std::is_trivially_copyable_v<T>;

template<typename T>
concept cxx_is_trivially_movable = std::is_trivially_move_constructible_v<T>
                               and std::is_trivially_destructible_v<T>;

template<typename T>
concept cxx_is_trivially_destructible = std::is_trivially_destructible_v<T>;

template<typename T>
concept cxx_is_equality_comparable = std::equality_comparable<T>;

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
concept cxx_is_totally_ordered = std::totally_ordered<T>;

template<typename T>
concept cxx_is_hashable = detection::is_std_hashable<T>;

template<typename T>
concept cxx_is_debuggable = detection::has_operator_ostream_left_shift<T>;

template<typename T>
concept cxx_is_displayable = detection::has_operator_std_string_view<T>
                          or detection::has_operator_std_string<T>
                          or detection::has_to_string<T>;

template<typename T>
concept rust_should_impl_cxx_extern_type_trivial = cxx_is_trivially_movable<T>;

template<typename T>
concept rust_should_impl_unpin = cxx_is_trivially_movable<T>;

template<typename T>
concept rust_should_impl_send = false;

template<typename T>
concept rust_should_impl_sync = false;

template<typename T>
concept rust_should_impl_drop = cxx_is_destructible<T>
                            and not cxx_is_trivially_destructible<T>;

template<typename T>
concept rust_should_impl_copy = cxx_is_trivially_copyable<T>
                            and cxx_is_trivially_movable<T>
                            and not rust_should_impl_drop<T>;

template<typename T>
concept rust_should_impl_default = cxx_is_default_constructible<T>;

template<typename T>
concept rust_should_impl_moveref_copy_new = cxx_is_copy_constructible<T>;

template<typename T>
concept rust_should_impl_moveref_move_new = cxx_is_move_constructible<T>;

template<typename T>
concept rust_should_impl_eq = cxx_is_equality_comparable<T>;

template<typename T>
concept rust_should_impl_partial_eq = cxx_has_operator_equal<T>;

template<typename T>
concept rust_should_impl_partial_ord = cxx_has_operator_three_way_comparison<T>
                                    or (cxx_has_operator_less_than<T>
                                        and cxx_has_operator_equal<T>);

template<typename T>
concept rust_should_impl_ord = cxx_is_totally_ordered<T>;

template<typename T>
concept rust_should_impl_hash = cxx_is_hashable<T>;

template<typename T>
concept rust_should_impl_debug = cxx_is_debuggable<T>;

template<typename T>
concept rust_should_impl_display = cxx_is_displayable<T>;
} // namespace cxx_auto

// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define CXX_AUTO_PRELUDE_HEADER(NAMESPACE, SELF)                                                    \
  extern "C" size_t const NAMESPACE##$CXX_ABI_ALIGN;                                                \
  extern "C" size_t const NAMESPACE##$CXX_ABI_SIZE;                                                 \
  extern "C" bool const NAMESPACE##$CXX_IS_DEFAULT_CONSTRUCTIBLE;                                   \
  extern "C" bool const NAMESPACE##$CXX_IS_COPY_CONSTRUCTIBLE;                                      \
  extern "C" bool const NAMESPACE##$CXX_IS_MOVE_CONSTRUCTIBLE;                                      \
  extern "C" bool const NAMESPACE##$CXX_IS_DESTRUCTIBLE;                                            \
  extern "C" bool const NAMESPACE##$CXX_IS_TRIVIALLY_COPYABLE;                                      \
  extern "C" bool const NAMESPACE##$CXX_IS_TRIVIALLY_MOVABLE;                                       \
  extern "C" bool const NAMESPACE##$CXX_IS_TRIVIALLY_DESTRUCTIBLE;                                  \
  extern "C" bool const NAMESPACE##$CXX_IS_EQUALITY_COMPARABLE;                                     \
  extern "C" bool const NAMESPACE##$CXX_HAS_OPERATOR_EQUAL;                                         \
  extern "C" bool const NAMESPACE##$CXX_HAS_OPERATOR_NOT_EQUAL;                                     \
  extern "C" bool const NAMESPACE##$CXX_HAS_OPERATOR_LESS_THAN;                                     \
  extern "C" bool const NAMESPACE##$CXX_HAS_OPERATOR_LESS_THAN_OR_EQUAL;                            \
  extern "C" bool const NAMESPACE##$CXX_HAS_OPERATOR_GREATER_THAN;                                  \
  extern "C" bool const NAMESPACE##$CXX_HAS_OPERATOR_GREATER_THAN_OR_EQUAL;                         \
  extern "C" bool const NAMESPACE##$CXX_HAS_OPERATOR_THREE_WAY_COMPARISON;                          \
  extern "C" bool const NAMESPACE##$CXX_IS_PARTIALLY_ORDERED;                                       \
  extern "C" bool const NAMESPACE##$CXX_IS_TOTALLY_ORDERED;                                         \
  extern "C" bool const NAMESPACE##$CXX_IS_HASHABLE;                                                \
  extern "C" bool const NAMESPACE##$CXX_IS_DEBUGGABLE;                                              \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_CXX_EXTERN_TYPE_TRIVIAL;                       \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_UNPIN;                                         \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_SEND;                                          \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_SYNC;                                          \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_DROP;                                          \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_COPY;                                          \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_DEFAULT;                                       \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_MOVEREF_COPY_NEW;                              \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_MOVEREF_MOVE_NEW;                              \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_EQ;                                            \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_PARTIAL_EQ;                                    \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_PARTIAL_ORD;                                   \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_ORD;                                           \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_HASH;                                          \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_DEBUG;                                         \
  extern "C" bool const NAMESPACE##$RUST_SHOULD_IMPL_DISPLAY;                                       \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void                                \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_is_default_constructible<T>;                                           \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_copy_new(T* This [[clang::lifetimebound]],                                                    \
               T const& that [[clang::lifetimebound]]) noexcept -> void                             \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_is_copy_constructible<T>;                                              \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_move_new(T* This [[clang::lifetimebound]], T* that [[clang::lifetimebound]]) noexcept -> void \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_is_move_constructible<T>;                                              \
                                                                                                    \
  template<typename T>                                                                              \
  auto                                                                                              \
  cxx_destruct(T* This [[clang::lifetimebound]]) noexcept -> void                                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_is_destructible<T>;                                                    \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_equal(T const& This [[clang::lifetimebound]],                                        \
                     T const& That [[clang::lifetimebound]]) noexcept -> bool                       \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_has_operator_equal<T>;                                                 \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_not_equal(T const& This [[clang::lifetimebound]],                                    \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_has_operator_not_equal<T>;                                             \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_less_than(T const& This [[clang::lifetimebound]],                                    \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_has_operator_less_than<T>;                                             \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_less_than_or_equal(T const& This [[clang::lifetimebound]],                           \
                                  T const& That [[clang::lifetimebound]]) noexcept -> bool          \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_has_operator_less_than_or_equal<T>;                                    \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_greater_than(T const& This [[clang::lifetimebound]],                                 \
                            T const& That [[clang::lifetimebound]]) noexcept -> bool                \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_has_operator_greater_than<T>;                                          \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_greater_than_or_equal(T const& This [[clang::lifetimebound]],                        \
                                     T const& That [[clang::lifetimebound]]) noexcept -> bool       \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_has_operator_greater_than_or_equal<T>;                                 \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                         \
                                    T const& That [[clang::lifetimebound]]) noexcept -> int8_t      \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_has_operator_three_way_comparison<T>;                                  \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> size_t                               \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_is_hashable<T>;                                                        \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> rust::string                        \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_is_debuggable<T>;                                                      \
                                                                                                    \
  template<typename T>                                                                              \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::string                      \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::cxx_is_displayable<T>;

// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define CXX_AUTO_PRELUDE_SOURCE(NAMESPACE, SELF)                                                                                \
  size_t const NAMESPACE##$CXX_ABI_ALIGN = alignof(SELF);                                                                       \
  size_t const NAMESPACE##$CXX_ABI_SIZE = sizeof(SELF);                                                                         \
  bool const NAMESPACE##$CXX_IS_DEFAULT_CONSTRUCTIBLE = ::cxx_auto::cxx_is_default_constructible<SELF>;                         \
  bool const NAMESPACE##$CXX_IS_COPY_CONSTRUCTIBLE = ::cxx_auto::cxx_is_copy_constructible<SELF>;                               \
  bool const NAMESPACE##$CXX_IS_MOVE_CONSTRUCTIBLE = ::cxx_auto::cxx_is_move_constructible<SELF>;                               \
  bool const NAMESPACE##$CXX_IS_DESTRUCTIBLE = ::cxx_auto::cxx_is_destructible<SELF>;                                           \
  bool const NAMESPACE##$CXX_IS_TRIVIALLY_COPYABLE = ::cxx_auto::cxx_is_trivially_copyable<SELF>;                               \
  bool const NAMESPACE##$CXX_IS_TRIVIALLY_MOVABLE = ::cxx_auto::cxx_is_trivially_movable<SELF>;                                 \
  bool const NAMESPACE##$CXX_IS_TRIVIALLY_DESTRUCTIBLE = ::cxx_auto::cxx_is_trivially_destructible<SELF>;                       \
  bool const NAMESPACE##$CXX_IS_EQUALITY_COMPARABLE = ::cxx_auto::cxx_is_equality_comparable<SELF>;                             \
  bool const NAMESPACE##$CXX_HAS_OPERATOR_EQUAL = ::cxx_auto::cxx_has_operator_equal<SELF>;                                     \
  bool const NAMESPACE##$CXX_HAS_OPERATOR_NOT_EQUAL = ::cxx_auto::cxx_has_operator_not_equal<SELF>;                             \
  bool const NAMESPACE##$CXX_HAS_OPERATOR_LESS_THAN = ::cxx_auto::cxx_has_operator_less_than<SELF>;                             \
  bool const NAMESPACE##$CXX_HAS_OPERATOR_LESS_THAN_OR_EQUAL = ::cxx_auto::cxx_has_operator_less_than_or_equal<SELF>;           \
  bool const NAMESPACE##$CXX_HAS_OPERATOR_GREATER_THAN = ::cxx_auto::cxx_has_operator_greater_than<SELF>;                       \
  bool const NAMESPACE##$CXX_HAS_OPERATOR_GREATER_THAN_OR_EQUAL = ::cxx_auto::cxx_has_operator_greater_than_or_equal<SELF>;     \
  bool const NAMESPACE##$CXX_HAS_OPERATOR_THREE_WAY_COMPARISON = ::cxx_auto::cxx_has_operator_three_way_comparison<SELF>;       \
  bool const NAMESPACE##$CXX_IS_PARTIALLY_ORDERED = ::cxx_auto::cxx_is_partially_ordered<SELF>;                                 \
  bool const NAMESPACE##$CXX_IS_TOTALLY_ORDERED = ::cxx_auto::cxx_is_totally_ordered<SELF>;                                     \
  bool const NAMESPACE##$CXX_IS_HASHABLE = ::cxx_auto::cxx_is_hashable<SELF>;                                                   \
  bool const NAMESPACE##$CXX_IS_DEBUGGABLE = ::cxx_auto::cxx_is_debuggable<SELF>;                                               \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_CXX_EXTERN_TYPE_TRIVIAL = ::cxx_auto::rust_should_impl_cxx_extern_type_trivial<SELF>; \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_UNPIN = ::cxx_auto::rust_should_impl_unpin<SELF>;                                     \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_SEND = ::cxx_auto::rust_should_impl_send<SELF>;                                       \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_SYNC = ::cxx_auto::rust_should_impl_sync<SELF>;                                       \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_DROP = ::cxx_auto::rust_should_impl_drop<SELF>;                                       \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_COPY = ::cxx_auto::rust_should_impl_copy<SELF>;                                       \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_DEFAULT = ::cxx_auto::rust_should_impl_default<SELF>;                                 \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_MOVEREF_COPY_NEW = ::cxx_auto::rust_should_impl_moveref_copy_new<SELF>;               \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_MOVEREF_MOVE_NEW = ::cxx_auto::rust_should_impl_moveref_move_new<SELF>;               \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_EQ = ::cxx_auto::rust_should_impl_eq<SELF>;                                           \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_PARTIAL_EQ = ::cxx_auto::rust_should_impl_partial_eq<SELF>;                           \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_PARTIAL_ORD = ::cxx_auto::rust_should_impl_partial_ord<SELF>;                         \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_ORD = ::cxx_auto::rust_should_impl_ord<SELF>;                                         \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_HASH = ::cxx_auto::rust_should_impl_hash<SELF>;                                       \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_DEBUG = ::cxx_auto::rust_should_impl_debug<SELF>;                                     \
  bool const NAMESPACE##$RUST_SHOULD_IMPL_DISPLAY = ::cxx_auto::rust_should_impl_display<SELF>;                                 \
                                                                                                                                \
  template<typename T, typename... Args>                                                                                        \
  auto                                                                                                                          \
  cxx_placement_new(T* This [[clang::lifetimebound]], Args&&... args) noexcept -> void                                          \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_constructible<T, Args...>                                                                       \
  {                                                                                                                             \
    new (This) T(std::forward<Args>(args)...);                                                                                  \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  auto                                                                                                                          \
  cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void                                                            \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_default_constructible<T>                                                                        \
  {                                                                                                                             \
    cxx_placement_new(This);                                                                                                    \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  auto                                                                                                                          \
  cxx_copy_new(T* This [[clang::lifetimebound]],                                                                                \
               T const& that [[clang::lifetimebound]]) noexcept -> void                                                         \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_copy_constructible<T>                                                                           \
  {                                                                                                                             \
    new (This) T(that);                                                                                                         \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  auto                                                                                                                          \
  cxx_move_new(T* This [[clang::lifetimebound]], T* that [[clang::lifetimebound]]) noexcept -> void                             \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_move_constructible<T>                                                                           \
  {                                                                                                                             \
    new (This) T(std::forward<T>(that));                                                                                        \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  auto                                                                                                                          \
  cxx_destruct(T* This [[clang::lifetimebound]]) noexcept -> void                                                               \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_destructible<T>                                                                                 \
  {                                                                                                                             \
    std::destroy_at(This);                                                                                                      \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_operator_equal(T const& This [[clang::lifetimebound]],                                                                    \
                     T const& That [[clang::lifetimebound]]) noexcept -> bool                                                   \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_has_operator_equal<T>                                                                              \
  {                                                                                                                             \
    return (This == That);                                                                                                      \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_operator_not_equal(T const& This [[clang::lifetimebound]],                                                                \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                                               \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_has_operator_not_equal<T>                                                                          \
  {                                                                                                                             \
    return (This != That);                                                                                                      \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_operator_less_than(T const& This [[clang::lifetimebound]],                                                                \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                                               \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_has_operator_less_than<T>                                                                          \
  {                                                                                                                             \
    return (This < That);                                                                                                       \
  }                                                                                                                             \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_operator_less_than_or_equal(T const& This [[clang::lifetimebound]],                                                       \
                                  T const& That [[clang::lifetimebound]]) noexcept -> bool                                      \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_has_operator_less_than_or_equal<T>                                                                 \
  {                                                                                                                             \
    return (This <= That);                                                                                                      \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_operator_greater_than(T const& This [[clang::lifetimebound]],                                                             \
                            T const& That [[clang::lifetimebound]]) noexcept -> bool                                            \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_has_operator_greater_than<T>                                                                       \
  {                                                                                                                             \
    return (This > That);                                                                                                       \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_operator_greater_than_or_equal(T const& This [[clang::lifetimebound]],                                                    \
                                     T const& That [[clang::lifetimebound]]) noexcept -> bool                                   \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_has_operator_greater_than_or_equal<T>                                                              \
  {                                                                                                                             \
    return (This >= That);                                                                                                      \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]] [[gnu::always_inline]]                                                                                          \
  inline static auto                                                                                                            \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                                                     \
                                    T const& That [[clang::lifetimebound]]) noexcept -> int8_t                                  \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::detection::has_operator_three_way_comparison<T>                                                        \
  {                                                                                                                             \
    auto result = (This <=> That);                                                                                              \
    if (result < 0) {                                                                                                           \
      return -1;                                                                                                                \
    } else if (result > 0) { /* NOLINT(llvm-else-after-return, readability-else-after-return) */                                \
      return 1;                                                                                                                 \
    } else if (result == 0) {                                                                                                   \
      return 0;                                                                                                                 \
    } else {                                                                                                                    \
      return std::numeric_limits<int8_t>::max();                                                                                \
    }                                                                                                                           \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]] [[gnu::always_inline]]                                                                                          \
  inline static auto                                                                                                            \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                                                     \
                                    T const& That [[clang::lifetimebound]]) noexcept -> int8_t                                  \
    requires ::std::same_as<T, SELF>                                                                                            \
         and (not ::cxx_auto::detection::has_operator_three_way_comparison<T>)                                                  \
         and ::cxx_auto::detection::has_operator_less_than<T>                                                                   \
         and ::cxx_auto::detection::has_operator_equal<T>                                                                       \
  {                                                                                                                             \
    auto le = (This < That);                                                                                                    \
    auto eq = (This == That);                                                                                                   \
    if (le and not eq) {                                                                                                        \
      return -1;                                                                                                                \
    } else if (not le and not eq) { /* NOLINT(llvm-else-after-return, readability-else-after-return) */                         \
      return 1;                                                                                                                 \
    } else if (not le and eq) {                                                                                                 \
      return 0;                                                                                                                 \
    } else {                                                                                                                    \
      return std::numeric_limits<int8_t>::max();                                                                                \
    }                                                                                                                           \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> size_t                                                           \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_hashable<T>                                                                                     \
  {                                                                                                                             \
    return std::hash<T> {}(This);                                                                                               \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> rust::string                                                    \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_debuggable<T>                                                                                   \
  {                                                                                                                             \
    std::ostringstream os;                                                                                                      \
    os << This;                                                                                                                 \
    return rust::String::lossy(os.str());                                                                                       \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::String                                                  \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::detection::has_operator_std_string_view<T>                                                             \
  {                                                                                                                             \
    return rust::String::lossy(std::string { This.operator std::string_view() });                                               \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::String                                                  \
    requires ::std::same_as<T, SELF>                                                                                            \
         and (not ::cxx_auto::detection::has_operator_std_string_view<T>)                                                       \
         and ::cxx_auto::detection::has_operator_std_string<T>                                                                  \
  {                                                                                                                             \
    return rust::String::lossy(This.operator std::string());                                                                    \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::String                                                  \
    requires ::std::same_as<T, SELF>                                                                                            \
         and (not ::cxx_auto::detection::has_operator_std_string_view<T>)                                                       \
         and (not ::cxx_auto::detection::has_operator_std_string<T>)                                                            \
         and ::cxx_auto::detection::has_to_string<T>                                                                            \
  {                                                                                                                             \
    return rust::String::lossy(std::to_string(This));                                                                           \
  }                                                                                                                             \
                                                                                                                                \
  template<typename T>                                                                                                          \
  [[nodiscard]]                                                                                                                 \
  auto                                                                                                                          \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::string                                                  \
    requires ::std::same_as<T, SELF>                                                                                            \
         and ::cxx_auto::cxx_is_displayable<T>;
