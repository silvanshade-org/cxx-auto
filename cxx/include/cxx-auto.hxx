#pragma once

#include "rust/cxx.h"
#include "sys/types.h"

#include <array>
#include <compare>
#include <concepts>
#include <cstddef>
#include <cstdint>
#include <iterator>
#include <limits>
#include <memory>
#include <ranges>
#include <sstream>
#include <string_view>
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
/// Test exact membership in a finite type pack.
/// # Specification
/// trivial.
template<typename T, typename... U>
concept same_as_any_of = (std::same_as<T, U> or ...);

/// Detect equality on const lvalues.
/// # Specification
/// - provides: true exactly when const T lvalues support == with a bool result.
/// - panics: none.
/// # Adequacy
/// - hypothesis: a mutable-only overload and a bool-like proxy must both differ from a const bool overload.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
concept has_operator_equal = requires(T const& lhs, T const& rhs) { //
  { lhs == rhs } -> std::same_as<bool>;
};

/// Detect inequality on const lvalues.
/// # Specification
/// - provides: true exactly when const T lvalues support != with a bool result.
/// - panics: none.
template<typename T>
concept has_operator_not_equal = requires(T const& lhs, T const& rhs) { //
  { lhs != rhs } -> std::same_as<bool>;
};

/// Detect strict less-than on const lvalues.
/// # Specification
/// - provides: true exactly when const T lvalues support < with a bool result.
/// - panics: none.
template<typename T>
concept has_operator_less_than = requires(T const& lhs, T const& rhs) { //
  { lhs < rhs } -> std::same_as<bool>;
};

/// Detect a standard three-way ordering category.
/// # Specification
/// - provides: true exactly when const T lvalues support <=> yielding strong, weak, or partial ordering.
/// - panics: none.
/// # Adequacy
/// - hypothesis: weak and partial categories must be accepted while an unrelated result is rejected.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
concept has_operator_three_way_comparison = requires(T const& lhs, T const& rhs) { //
  requires same_as_any_of<decltype(lhs <=> rhs), std::strong_ordering, std::weak_ordering, std::partial_ordering>;
};

/// Detect a total three-way ordering, C++'s type-level claim that `<=>` is a
/// total order consistent with substitutable equality.
/// # Specification
/// - provides: true exactly when const T lvalues support <=> yielding std::strong_ordering.
/// - panics: none.
template<typename T>
concept has_strong_ordering = requires(T const& lhs, T const& rhs) { //
  { lhs <=> rhs } -> std::same_as<std::strong_ordering>;
};

/// Detect a standard hash of a const value.
/// # Specification
/// - provides: true exactly when std::hash<T> is invocable on const T and yields std::size_t.
/// - panics: none.
template<typename T>
concept is_std_hashable = requires(T const& arg) { //
  { std::hash<T>{}(arg) } -> std::same_as<std::size_t>;
};

/// Detect an explicit conversion operator yielding std::string.
/// # Specification
/// - provides: true exactly when a const T has a callable operator std::string() returning std::string.
/// - panics: none.
template<typename T>
concept has_operator_std_string = requires(T const& arg) { //
  { arg.operator std::string() } -> std::same_as<std::string>;
};

/// Detect an explicit conversion operator yielding std::string_view.
/// # Specification
/// - provides: true exactly when a const T has a callable operator std::string_view() returning std::string_view.
/// - panics: none.
template<typename T>
concept has_operator_std_string_view = requires(T const& arg) { //
  { arg.operator std::string_view() } -> std::same_as<std::string_view>;
};

/// Detect std::to_string for a const value.
/// # Specification
/// - provides: true exactly when std::to_string(T const&) is valid and yields std::string.
/// - panics: none.
template<typename T>
concept has_to_string = requires(T const& arg) { //
  { std::to_string(arg) } -> std::same_as<std::string>;
};

/// Detect insertion of a const value into an output stream.
/// # Specification
/// - provides: true exactly when inserting const T into std::ostream yields std::ostream&.
/// - panics: none.
template<typename T>
concept has_operator_ostream_left_shift = requires(T const& arg, std::ostream& os) { //
  { os << arg } -> std::same_as<std::ostream&>;
};

/// Detect construction from a pair of input iterators.
/// # Specification
/// - provides: true exactly when It is an input iterator and T{first, last} is valid with type T.
/// - panics: none.
template<typename T, typename It>
concept is_constructible_from_iterator = requires(It first, It last) { //
  requires std::input_iterator<It>;
  { T{ first, last } } -> std::same_as<T>;
};

/// Detect the standard range concept.
/// # Specification
/// trivial.
template<typename T>
concept is_iterable = std::ranges::range<T>;

/// Detect an input range whose value type matches V.
/// # Specification
/// - provides: true exactly when T is an input range and its iterator value type equals remove_reference_t<V>.
/// - panics: none.
template<typename T, typename V>
concept is_input_iterable = requires { //
  requires std::ranges::input_range<T>;
  requires std::same_as<std::iter_value_t<std::ranges::iterator_t<T>>, std::remove_reference_t<V>>;
};

/// Detect a mutable-lvalue input iterator.
/// # Specification
/// - provides: true exactly when T is an input iterator whose dereference type is value_type&.
/// - panics: none.
/// # Adequacy
/// - hypothesis: a mutable iterator differs from a const iterator and a value-producing iterator.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
concept is_input_copy_iterator = requires { //
  requires std::input_iterator<T>;
  requires std::same_as<std::iter_reference_t<T>, std::add_lvalue_reference_t<std::iter_value_t<T>>>;
};

/// Detect an rvalue input iterator.
/// # Specification
/// - provides: true exactly when T is an input iterator whose dereference type is value_type&&.
/// - panics: none.
/// # Adequacy
/// - hypothesis: a move iterator differs from a mutable-lvalue and const-lvalue iterator.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
concept is_input_move_iterator = requires { //
  requires std::input_iterator<T>;
  requires std::same_as<std::iter_reference_t<T>, std::add_rvalue_reference_t<std::iter_value_t<T>>>;
};
} // namespace cxx_auto::detection

namespace cxx_auto {
template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return alignof(T);
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return sizeof(T);
}

template<typename T, typename... Args>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_constructible() noexcept -> bool
{
  return std::is_constructible_v<T, Args...>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return std::is_default_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return std::is_copy_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return std::is_move_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return std::is_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return std::is_trivially_copyable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return std::is_trivially_move_constructible_v<T> and std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_equal() noexcept -> bool
{
  return detection::has_operator_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_not_equal() noexcept -> bool
{
  return detection::has_operator_not_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_less_than() noexcept -> bool
{
  return detection::has_operator_less_than<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_three_way_comparison() noexcept -> bool
{
  return detection::has_operator_three_way_comparison<T> or
         (not detection::has_operator_three_way_comparison<T> and detection::has_operator_less_than<T> and
          detection::has_operator_equal<T>);
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_hashable() noexcept -> bool
{
  return detection::is_std_hashable<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_debuggable() noexcept -> bool
{
  return detection::has_operator_ostream_left_shift<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_displayable() noexcept -> bool
{
  return detection::has_to_string<T> or detection::has_operator_std_string<T> or
         detection::has_operator_std_string_view<T>;
}

// Claims only a type's author can make. Specialize one beside the type's
// declaration, in the header or module interface that the `CXX_AUTO_EXPORT`
// translation unit includes before invoking the macro; a specialization
// declared after that point is ill-formed and may be silently ignored.
//
// - `rust_send`, `rust_sync`: the type may cross, or be shared across,
//   threads. C++ has no trait for either, so both default to false.
// - `rust_eq`: `==` is an equivalence relation. The default is C++'s own
//   claim, a `<=>` yielding `std::strong_ordering`; specialize it for a type
//   with a lawful `==` but no such `<=>`.
// - `rust_ord`: `<=>` is a total order agreeing with `==`, again claimed by
//   `std::strong_ordering`.
template<typename T>
inline constexpr bool rust_send = false;

template<typename T>
inline constexpr bool rust_sync = false;

template<typename T>
inline constexpr bool rust_eq = detection::has_strong_ordering<T>;

template<typename T>
inline constexpr bool rust_ord = detection::has_strong_ordering<T>;

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_cxx_extern_type_trivial() noexcept -> bool
{
  return cxx_is_trivially_movable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_unpin() noexcept -> bool
{
  return cxx_is_trivially_movable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_send() noexcept -> bool
{
  return rust_send<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_sync() noexcept -> bool
{
  return rust_sync<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_drop() noexcept -> bool
{
  return cxx_is_destructible<T>() and not cxx_is_trivially_destructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_copy() noexcept -> bool
{
  return cxx_is_trivially_copyable<T>() and cxx_is_trivially_movable<T>() and not rust_should_impl_drop<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_default() noexcept -> bool
{
  return cxx_is_default_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_moveref_copy_new() noexcept -> bool
{
  return cxx_is_copy_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_moveref_move_new() noexcept -> bool
{
  return cxx_is_move_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_partial_eq() noexcept -> bool
{
  return cxx_has_operator_equal<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_eq() noexcept -> bool
{
  return rust_eq<T> and rust_should_impl_partial_eq<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_partial_ord() noexcept -> bool
{
  return cxx_has_operator_three_way_comparison<T>() or
         (cxx_has_operator_less_than<T>() and cxx_has_operator_equal<T>());
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_ord() noexcept -> bool
{
  return rust_ord<T> and rust_should_impl_eq<T>() and rust_should_impl_partial_ord<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_hash() noexcept -> bool
{
  return cxx_is_hashable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_debug() noexcept -> bool
{
  return cxx_is_debuggable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_display() noexcept -> bool
{
  return cxx_is_displayable<T>();
}

} // namespace cxx_auto

namespace cxx_auto {
// A type record is the build script's only view of a bridged type. The build
// script reads it from the compiled object file (`src/record.rs`); nothing in
// it is executed or relocated, so it holds no pointers. Its layout is the wire
// format below, and any change to it bumps `record_version`.

// The bytes "cxx_auto" read as a little-endian integer.
inline constexpr std::uint64_t record_magic = 0x6f74'7561'5f78'7863ULL;
inline constexpr std::uint32_t record_version = 1;

// Bit positions of `TypeRecord::flags`, mirrored by `src/record.rs`.
namespace record_bit {
inline constexpr unsigned cxx_extern_type_trivial = 0;
inline constexpr unsigned unpin = 1;
inline constexpr unsigned send = 2;
inline constexpr unsigned sync = 3;
inline constexpr unsigned drop = 4;
inline constexpr unsigned copy = 5;
inline constexpr unsigned default_new = 6;
inline constexpr unsigned copy_new = 7;
inline constexpr unsigned move_new = 8;
inline constexpr unsigned eq = 9;
inline constexpr unsigned partial_eq = 10;
inline constexpr unsigned partial_ord = 11;
inline constexpr unsigned ord = 12;
inline constexpr unsigned hash = 13;
inline constexpr unsigned debug = 14;
inline constexpr unsigned display = 15;
inline constexpr unsigned operator_not_equal = 16;
} // namespace record_bit

// The names a bridged type needs on both sides of the bridge. Designated
// initializers follow this declaration order; `rust_lifetimes` may be omitted.
//
// - `rust_path`: the generated Rust module holding the type, `::`-separated
//   beneath the generated `auto` module (for example `number` or `geo::point`).
// - `rust_name`: the Rust type name.
// - `rust_lifetimes`: Rust lifetime parameters with their bounds, written as
//   they appear between angle brackets (for example `'a, 'b: 'a`).
// - `cxx_name`, `cxx_namespace`: the C++ type's name and enclosing namespace.
// - `cxx_proxy_include`: the header declaring the `CXX_AUTO_PRELUDE` proxy,
//   which the generated bridge includes.
struct TypeSpec final
{
  // NOLINTBEGIN(misc-non-private-member-variables-in-classes)
  std::string_view rust_path;
  std::string_view rust_name;
  std::string_view rust_lifetimes{};
  std::string_view cxx_name;
  std::string_view cxx_namespace;
  std::string_view cxx_proxy_include;
  // NOLINTEND(misc-non-private-member-variables-in-classes)
};

// The encoded record: a fixed header followed by `spec_len` bytes holding, in
// order, `rust_path`, `rust_name`, `rust_lifetimes`, `cxx_name`,
// `cxx_namespace`, the proxy namespace, and `cxx_proxy_include`, each followed
// by one NUL byte. Integers use the target's byte order.
template<std::size_t N>
struct TypeRecord final
{
  // NOLINTBEGIN(misc-non-private-member-variables-in-classes)
  std::uint64_t magic;
  std::uint32_t version;
  std::uint32_t spec_len;
  std::uint64_t abi_size;
  std::uint64_t abi_align;
  std::uint64_t flags;
  std::array<char, N> spec;
  // NOLINTEND(misc-non-private-member-variables-in-classes)
};
static_assert(std::is_standard_layout_v<TypeRecord<1>>);
static_assert(offsetof(TypeRecord<1>, magic) == 0);
static_assert(offsetof(TypeRecord<1>, version) == 8);
static_assert(offsetof(TypeRecord<1>, spec_len) == 12);
static_assert(offsetof(TypeRecord<1>, abi_size) == 16);
static_assert(offsetof(TypeRecord<1>, abi_align) == 24);
static_assert(offsetof(TypeRecord<1>, flags) == 32);
static_assert(offsetof(TypeRecord<1>, spec) == 40);

namespace detail {
// Deliberately neither `constexpr` nor defined: reaching a call during constant
// evaluation stops compilation, and the diagnostic quotes the reason.
auto
invalid_type_spec(char const* reason) noexcept -> void;

consteval auto
check_spec_field(std::string_view field, bool required) noexcept -> void
{
  if (required and field.empty()) {
    invalid_type_spec("a required TypeSpec field is empty");
  }
  for (char const byte : field) {
    if (byte == '\0') {
      invalid_type_spec("a TypeSpec field contains a NUL byte");
    }
  }
}

consteval auto
check_proxy_namespace(std::string_view proxy_namespace) noexcept -> void
{
  check_spec_field(proxy_namespace, true);
  if (proxy_namespace.starts_with("::")) {
    invalid_type_spec("write the proxy namespace without a leading `::`");
  }
  for (char const byte : proxy_namespace) {
    if (byte == ' ') {
      invalid_type_spec("write the proxy namespace without spaces");
    }
  }
}

template<bool Enabled>
consteval auto
record_bit_if(unsigned bit) noexcept -> std::uint64_t
{
  if constexpr (Enabled) {
    return std::uint64_t{ 1 } << bit;
  } else {
    return 0;
  }
}
} // namespace detail

// The byte length of the encoded `spec` together with its proxy namespace.
[[nodiscard]]
consteval auto
encoded_spec_size(TypeSpec const& spec, std::string_view proxy_namespace) noexcept -> std::size_t
{
  return spec.rust_path.size() + spec.rust_name.size() + spec.rust_lifetimes.size() + spec.cxx_name.size() +
         spec.cxx_namespace.size() + proxy_namespace.size() + spec.cxx_proxy_include.size() + 7;
}

// The capability flags of `Self`, one `record_bit` each.
template<typename Self>
[[nodiscard]]
consteval auto
record_flags() noexcept -> std::uint64_t
{
  using detail::record_bit_if;
  return record_bit_if<rust_should_impl_cxx_extern_type_trivial<Self>()>(record_bit::cxx_extern_type_trivial) |
         record_bit_if<rust_should_impl_unpin<Self>()>(record_bit::unpin) |
         record_bit_if<rust_should_impl_send<Self>()>(record_bit::send) |
         record_bit_if<rust_should_impl_sync<Self>()>(record_bit::sync) |
         record_bit_if<rust_should_impl_drop<Self>()>(record_bit::drop) |
         record_bit_if<rust_should_impl_copy<Self>()>(record_bit::copy) |
         record_bit_if<rust_should_impl_default<Self>()>(record_bit::default_new) |
         record_bit_if<rust_should_impl_moveref_copy_new<Self>()>(record_bit::copy_new) |
         record_bit_if<rust_should_impl_moveref_move_new<Self>()>(record_bit::move_new) |
         record_bit_if<rust_should_impl_eq<Self>()>(record_bit::eq) |
         record_bit_if<rust_should_impl_partial_eq<Self>()>(record_bit::partial_eq) |
         record_bit_if<rust_should_impl_partial_ord<Self>()>(record_bit::partial_ord) |
         record_bit_if<rust_should_impl_ord<Self>()>(record_bit::ord) |
         record_bit_if<rust_should_impl_hash<Self>()>(record_bit::hash) |
         record_bit_if<rust_should_impl_debug<Self>()>(record_bit::debug) |
         record_bit_if<rust_should_impl_display<Self>()>(record_bit::display) |
         record_bit_if<cxx_has_operator_not_equal<Self>()>(record_bit::operator_not_equal);
}

// Encode the record for `Self`. `N` must equal `encoded_spec_size(spec,
// proxy_namespace)`; `CXX_AUTO_EXPORT` supplies both.
template<typename Self, std::size_t N>
[[nodiscard]]
consteval auto
type_record(TypeSpec const& spec, std::string_view proxy_namespace) noexcept -> TypeRecord<N>
{
  detail::check_spec_field(spec.rust_path, true);
  detail::check_spec_field(spec.rust_name, true);
  detail::check_spec_field(spec.rust_lifetimes, false);
  detail::check_spec_field(spec.cxx_name, true);
  detail::check_spec_field(spec.cxx_namespace, true);
  detail::check_spec_field(spec.cxx_proxy_include, true);
  detail::check_proxy_namespace(proxy_namespace);
  static_assert(N <= std::numeric_limits<std::uint32_t>::max());
  if (N != encoded_spec_size(spec, proxy_namespace)) {
    detail::invalid_type_spec("the record size disagrees with the TypeSpec");
  }
  TypeRecord<N> record{
    .magic = record_magic,
    .version = record_version,
    .spec_len = static_cast<std::uint32_t>(N),
    .abi_size = sizeof(Self),
    .abi_align = alignof(Self),
    .flags = record_flags<Self>(),
    .spec = {},
  };
  std::size_t offset = 0;
  for (std::string_view const field : {
         spec.rust_path,
         spec.rust_name,
         spec.rust_lifetimes,
         spec.cxx_name,
         spec.cxx_namespace,
         proxy_namespace,
         spec.cxx_proxy_include,
       }) {
    for (char const byte : field) {
      record.spec.at(offset) = byte;
      ++offset;
    }
    record.spec.at(offset) = '\0';
    ++offset;
  }
  return record;
}
} // namespace cxx_auto

namespace cxx_auto {
template<typename T, typename... Args>
requires(cxx_is_constructible<T, Args...>())
[[gnu::always_inline]]
static inline auto
cxx_placement_new(T* This, Args&&... args) noexcept -> void
{
  new (This) T(std::forward<Args>(args)...);
}

template<typename T>
requires(cxx_is_default_constructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_default_new(T* This) noexcept -> void
{
  cxx_placement_new(This);
}

template<typename T>
requires(cxx_is_copy_constructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_copy_new(T* This, T const& that) noexcept -> void
requires std::is_lvalue_reference_v<decltype(that)>
{
  new (This) T(that);
}

template<typename T>
requires(cxx_is_move_constructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_move_new(T* This, T&& that) noexcept -> void
requires std::is_rvalue_reference_v<decltype(that)>
{
  new (This) T(std::forward<T>(that));
}

template<typename T>
requires(cxx_is_destructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_destruct(T* This) noexcept -> void
{
  std::destroy_at(This);
}

template<typename T>
requires(cxx_has_operator_equal<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_equal(T const& This, T const& That) noexcept -> bool
{
  return (This == That);
}

template<typename T>
requires(cxx_has_operator_not_equal<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_not_equal(T const& This, T const& That) noexcept -> bool
{
  return (This != That);
}

/// Translate a standard three-way comparison into the Rust-facing ordering code.
/// # Specification
/// - provides: -1, 0, 1 for less, equivalent, greater; int8_t maximum for unordered.
/// - panics: a throwing comparison terminates at this noexcept boundary.
/// # Adequacy
/// - hypothesis: weak equivalence and partial unordered results must differ from greater.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
requires(detection::has_operator_three_way_comparison<T>)
[[gnu::always_inline]]
static inline auto
cxx_operator_three_way_comparison(T const& This, T const& That) noexcept -> int8_t
{
  auto result = (This <=> That);
  if (result < 0) {
    return -1;
  } else if (result > 0) { // NOLINT(llvm-else-after-return, readability-else-after-return)
    return 1;
  } else if (result == 0) {
    return 0;
  } else {
    return std::numeric_limits<int8_t>::max();
  }
}

/// Translate legacy less-than and equality into the Rust-facing ordering code.
/// # Specification
/// - provides: -1 for This < That, 1 for That < This, 0 for equality, int8_t maximum otherwise.
/// - panics: a throwing comparison terminates at this noexcept boundary.
/// # Adequacy
/// - hypothesis: incomparable unequal values must not be confused with greater values.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
requires(
  not detection::has_operator_three_way_comparison<T> and detection::has_operator_less_than<T> and
  detection::has_operator_equal<T>
)
[[gnu::always_inline]]
static inline auto
cxx_operator_three_way_comparison(T const& This, T const& That) noexcept -> int8_t
{
  if (This < That) {
    return -1;
  }
  if (That < This) {
    return 1;
  }
  if (This == That) {
    return 0;
  }
  return std::numeric_limits<int8_t>::max();
}

template<typename T>
requires(detection::is_std_hashable<T>)
[[gnu::always_inline]]
static inline auto
cxx_hash(T const& This) noexcept -> size_t
{
  return std::hash<T>{}(This);
}

template<typename T>
requires(detection::has_operator_ostream_left_shift<T>)
[[gnu::always_inline]]
static inline auto
cxx_debug(T const& This) noexcept -> rust::String
{
  std::ostringstream os;
  os << This;
  return rust::String::lossy(os.str());
}

template<typename T>
requires(detection::has_to_string<T>)
[[gnu::always_inline]]
static inline auto
cxx_display(T const& This) noexcept -> rust::String
{
  return rust::String::lossy(std::to_string(This));
}

template<typename T>
requires(not detection::has_to_string<T> and detection::has_operator_std_string<T>)
[[gnu::always_inline]]
static inline auto
cxx_display(T const& This) noexcept -> rust::String
{
  return rust::String::lossy(This.operator std::string());
}

// FIXME: optimize this to use `&str` instead of `String`
template<typename T>
requires(
  not detection::has_to_string<T> and not detection::has_operator_std_string<T> and
  detection::has_operator_std_string_view<T>
)
[[gnu::always_inline]]
static inline auto
cxx_display(T const& This) noexcept -> rust::String
{
  return rust::String::lossy(std::string{ This.operator std::string_view() });
}

}; // namespace cxx_auto

// NOLINTBEGIN(cppcoreguidelines-macro-usage, bugprone-macro-parentheses)
#define CXX_AUTO_PRELUDE_SELECT_MACRO(_0, _1, _2, NAME, ...) NAME

#define CXX_AUTO_PRELUDE_TY_CON_DEFINE_1(TY_CON) using TyCon = TY_CON;
#define CXX_AUTO_PRELUDE_TY_CON_DEFINE_2(TY_CON, TY_ARG0)                                                              \
  template<typename TyArg0>                                                                                            \
  using TyCon = TY_CON<TyArg0>;
#define CXX_AUTO_PRELUDE_TY_CON_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1)                                                     \
  template<typename TyArg0, typename TyArg1>                                                                           \
  using TyCon = TY_CON<TyArg0, TyArg1>;
#define CXX_AUTO_PRELUDE_TY_CON_DEFINE(...)                                                                            \
  CXX_AUTO_PRELUDE_SELECT_MACRO(                                                                                       \
    __VA_ARGS__, CXX_AUTO_PRELUDE_TY_CON_DEFINE_3, CXX_AUTO_PRELUDE_TY_CON_DEFINE_2, CXX_AUTO_PRELUDE_TY_CON_DEFINE_1, \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_1(TY_CON)
#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_2(TY_CON, TY_ARG0) using TyArg0 = TY_ARG0;
#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1)                                                    \
  using TyArg0 = TY_ARG0;                                                                                              \
  using TyArg1 = TY_ARG1;
#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE(...)                                                                           \
  CXX_AUTO_PRELUDE_SELECT_MACRO(                                                                                       \
    __VA_ARGS__,                                                                                                       \
    CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_3,                                                                                 \
    CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_2,                                                                                 \
    CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_1,                                                                                 \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_AUTO_PRELUDE_TYPE_DEFINE_1(TY_CON) using Self = TY_CON;
#define CXX_AUTO_PRELUDE_TYPE_DEFINE_2(TY_CON, TY_ARG0) using Self = TY_CON<TY_ARG0>;
#define CXX_AUTO_PRELUDE_TYPE_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1) using Self = TY_CON<TY_ARG0, TY_ARG1>;

#define CXX_AUTO_PRELUDE_TYPE_DEFINE(...)                                                                              \
  CXX_AUTO_PRELUDE_SELECT_MACRO(                                                                                       \
    __VA_ARGS__, CXX_AUTO_PRELUDE_TYPE_DEFINE_3, CXX_AUTO_PRELUDE_TYPE_DEFINE_2, CXX_AUTO_PRELUDE_TYPE_DEFINE_1,       \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_AUTO_PRELUDE(CXX_NAME, ...)                                                                                \
  CXX_AUTO_PRELUDE_TY_CON_DEFINE(__VA_ARGS__)                                                                          \
  CXX_AUTO_PRELUDE_TY_ARGS_DEFINE(__VA_ARGS__)                                                                         \
  CXX_AUTO_PRELUDE_TYPE_DEFINE(__VA_ARGS__)                                                                            \
  using CXX_NAME = Self;                                                                                               \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_default_constructible<T>())                                  \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_default_new(T* This) noexcept -> void                                                         \
  {                                                                                                                    \
    return ::cxx_auto::cxx_default_new(This);                                                                          \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_copy_constructible<T>())                                     \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_copy_new(T* This, T const& that) noexcept -> void                                             \
  {                                                                                                                    \
    return ::cxx_auto::cxx_copy_new(This, that);                                                                       \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_move_constructible<T>())                                     \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_move_new(T* This, T* that) noexcept -> void                                                   \
  {                                                                                                                    \
    /* NOLINTNEXTLINE(hicpp-move-const-arg, performance-move-const-arg) */                                             \
    return ::cxx_auto::cxx_move_new(This, ::std::move(*that));                                                         \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_destructible<T>())                                           \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_destruct(T* This) noexcept -> void                                                            \
  {                                                                                                                    \
    return ::cxx_auto::cxx_destruct(This);                                                                             \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_equal<T>())                                        \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_operator_equal(T const& This, T const& That) noexcept -> bool                                 \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_equal(This, That);                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_not_equal<T>())                                    \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_operator_not_equal(T const& This, T const& That) noexcept -> bool                             \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_not_equal(This, That);                                                             \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_three_way_comparison<T>())                         \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_operator_three_way_comparison(T const& This, T const& That) noexcept -> int8_t                \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_three_way_comparison(This, That);                                                  \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_hashable<T>())                                               \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_hash(T const& This) noexcept -> size_t                                                        \
  {                                                                                                                    \
    return ::cxx_auto::cxx_hash(This);                                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_debuggable<T>())                                             \
  [[maybe_unused]] [[gnu::always_inline]]                                                                              \
  static inline auto cxx_debug(T const& This) noexcept -> rust::string                                                 \
  {                                                                                                                    \
    return ::cxx_auto::cxx_debug(This);                                                                                \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_displayable<T>())                                            \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_display(T const& This) noexcept -> rust::string                                               \
  {                                                                                                                    \
    return ::cxx_auto::cxx_display(This);                                                                              \
  }

// Export the type record of the type bound by `CXX_AUTO_PRELUDE` in namespace
// `PROXY`, whose name is written without a leading `::` or spaces. The
// remaining arguments are the `cxx_auto::TypeSpec` designated initializers.
// Invoke it once per type, at global scope, in a translation unit whose object
// file the build script passes to `cxx_auto::generate`; `ID` names the record's
// symbol and must be unique across that set.
#define CXX_AUTO_EXPORT(ID, PROXY, ...)                                                                                \
  inline constexpr ::cxx_auto::TypeSpec cxx_auto_spec_##ID{ __VA_ARGS__ };                                             \
  extern "C" [[gnu::used, gnu::retain]]                                                                                \
  constexpr ::cxx_auto::TypeRecord<::cxx_auto::encoded_spec_size(cxx_auto_spec_##ID, #PROXY)>                          \
    cxx_auto_type_##ID =                                                                                               \
      ::cxx_auto::type_record<PROXY::Self, ::cxx_auto::encoded_spec_size(cxx_auto_spec_##ID, #PROXY)>(                 \
        cxx_auto_spec_##ID, #PROXY                                                                                     \
      );

// NOLINTEND(cppcoreguidelines-macro-usage, bugprone-macro-parentheses)
