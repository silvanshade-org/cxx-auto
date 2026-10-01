// The cxx-auto C++ library as a named module. Build systems compile it before
// any translation unit that includes `cxx-auto.hxx`, which imports it.
module;

#include "../include/cxx-auto-results.hxx"
#include "../include/cxx-auto-std.hxx"

export module cxx_auto;

export namespace cxx_auto::detection {
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

export namespace cxx_auto {
template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_abi_align() noexcept -> size_t
{
  return alignof(T);
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_abi_size() noexcept -> size_t
{
  return sizeof(T);
}

template<typename T, typename... Args>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_constructible() noexcept -> bool
{
  return std::is_constructible_v<T, Args...>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_default_constructible() noexcept -> bool
{
  return std::is_default_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_copy_constructible() noexcept -> bool
{
  return std::is_copy_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_move_constructible() noexcept -> bool
{
  return std::is_move_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_nothrow_default_constructible() noexcept -> bool
{
  return std::is_nothrow_default_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_nothrow_copy_constructible() noexcept -> bool
{
  return std::is_nothrow_copy_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_nothrow_move_constructible() noexcept -> bool
{
  return std::is_nothrow_move_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_copy_assignable() noexcept -> bool
{
  return std::is_copy_assignable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_move_assignable() noexcept -> bool
{
  return std::is_move_assignable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_nothrow_copy_assignable() noexcept -> bool
{
  return std::is_nothrow_copy_assignable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_nothrow_move_assignable() noexcept -> bool
{
  return std::is_nothrow_move_assignable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_destructible() noexcept -> bool
{
  return std::is_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return std::is_trivially_copyable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_trivially_movable() noexcept -> bool
{
  return std::is_trivially_move_constructible_v<T> and std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_has_operator_equal() noexcept -> bool
{
  return detection::has_operator_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_has_operator_not_equal() noexcept -> bool
{
  return detection::has_operator_not_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_has_operator_less_than() noexcept -> bool
{
  return detection::has_operator_less_than<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_has_operator_three_way_comparison() noexcept -> bool
{
  return detection::has_operator_three_way_comparison<T>
      or (not detection::has_operator_three_way_comparison<T>
          and detection::has_operator_less_than<T>
          and detection::has_operator_equal<T>);
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_hashable() noexcept -> bool
{
  return detection::is_std_hashable<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_debuggable() noexcept -> bool
{
  return detection::has_operator_ostream_left_shift<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
cxx_is_displayable() noexcept -> bool
{
  return detection::has_to_string<T>
      or detection::has_operator_std_string<T>
      or detection::has_operator_std_string_view<T>;
}

// Claims only a type's author can make. Specialize one beside the type's
// declaration, in the header or module interface that the `CXX_AUTO_EXPORT`
// translation unit includes before invoking the macro; a specialization
// declared after that point is ill-formed and may be silently ignored.
//
// - `rust_send`, `rust_sync`: the type may cross, or be shared across,
//   threads. C++ has no trait for either, so both default to false.
// - `rust_relocatable`: an object may be moved to new storage by copying its
//   bytes, with the old storage then abandoned without running its
//   destructor. Types that are trivially move-constructible and trivially
//   destructible already are; C++26 has no standard trait for the rest, so
//   this defaults to false. Specialize it only for a type that holds no
//   pointer into itself and that nothing else tracks by address, such as a
//   class declared `[[clang::trivial_abi]]` or libc++'s `std::unique_ptr`.
// - `rust_assignable`: assigning through a `Pin<&mut T>` preserves every
//   invariant of whatever object that reference is part of. Safe Rust can
//   reach a `Pin<&mut Base>` for the base subobject of a more-derived object,
//   and assigning to it rewrites the base's state beneath the derived class's
//   invariants, polymorphic or not. The default is `std::is_final_v<T>`, since
//   nothing derives from a final class. Specialize it for a type no class
//   derives from, or whose derived classes keep no invariant over its state.
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
inline constexpr bool rust_relocatable = false;

template<typename T>
inline constexpr bool rust_assignable = std::is_final_v<T>;

template<typename T>
inline constexpr bool rust_eq = detection::has_strong_ordering<T>;

template<typename T>
inline constexpr bool rust_ord = detection::has_strong_ordering<T>;

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_cxx_extern_type_trivial() noexcept -> bool
{
  return cxx_is_trivially_movable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_unpin() noexcept -> bool
{
  return cxx_is_trivially_movable<T>() or rust_relocatable<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_send() noexcept -> bool
{
  return rust_send<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_sync() noexcept -> bool
{
  return rust_sync<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_drop() noexcept -> bool
{
  return cxx_is_destructible<T>() and not cxx_is_trivially_destructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_copy() noexcept -> bool
{
  return cxx_is_trivially_copyable<T>() and cxx_is_trivially_movable<T>() and not rust_should_impl_drop<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_default() noexcept -> bool
{
  return cxx_is_default_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_copy_new() noexcept -> bool
{
  return cxx_is_copy_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_move_new() noexcept -> bool
{
  return cxx_is_move_constructible<T>();
}

// Assignment is offered only where `rust_assignable` holds: a `Pin<&mut T>`
// may refer to the base subobject of a more-derived object.
template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_copy_assign() noexcept -> bool
{
  return cxx_is_copy_assignable<T>() and rust_assignable<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_move_assign() noexcept -> bool
{
  return cxx_is_move_assignable<T>() and rust_assignable<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_partial_eq() noexcept -> bool
{
  return cxx_has_operator_equal<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_eq() noexcept -> bool
{
  return rust_eq<T> and rust_should_impl_partial_eq<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_partial_ord() noexcept -> bool
{
  return cxx_has_operator_three_way_comparison<T>()
      or (cxx_has_operator_less_than<T>() and cxx_has_operator_equal<T>());
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_ord() noexcept -> bool
{
  return rust_ord<T> and rust_should_impl_eq<T>() and rust_should_impl_partial_ord<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_hash() noexcept -> bool
{
  return cxx_is_hashable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_debug() noexcept -> bool
{
  return cxx_is_debuggable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr auto
rust_should_impl_display() noexcept -> bool
{
  return cxx_is_displayable<T>();
}

} // namespace cxx_auto

export namespace cxx_auto {
// A type record is the build script's only view of a bridged type. The build
// script reads it from the compiled object file (`src/record.rs`); nothing in
// it is executed or relocated, so it holds no pointers. Its layout is the wire
// format below, and any change to it bumps `record_version`.

// The bytes "cxx_auto" read as a little-endian integer.
inline constexpr std::uint64_t record_magic = 0x6f74'7561'5f78'7863ULL;
inline constexpr std::uint32_t record_version = 2;

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
inline constexpr unsigned default_new_nothrow = 17;
inline constexpr unsigned copy_new_nothrow = 18;
inline constexpr unsigned move_new_nothrow = 19;
inline constexpr unsigned copy_assign = 20;
inline constexpr unsigned move_assign = 21;
inline constexpr unsigned copy_assign_nothrow = 22;
inline constexpr unsigned move_assign_nothrow = 23;
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
  // NOLINTNEXTLINE(readability-redundant-member-init): Clang 22 needs a default for omitted designated fields.
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
  return spec.rust_path.size()
       + spec.rust_name.size()
       + spec.rust_lifetimes.size()
       + spec.cxx_name.size()
       + spec.cxx_namespace.size()
       + proxy_namespace.size()
       + spec.cxx_proxy_include.size()
       + 7;
}

// The capability flags of `Self`, one `record_bit` each.
template<typename Self>
[[nodiscard]]
consteval auto
record_flags() noexcept -> std::uint64_t
{
  using detail::record_bit_if;
  return record_bit_if<rust_should_impl_cxx_extern_type_trivial<Self>()>(record_bit::cxx_extern_type_trivial)
       | record_bit_if<rust_should_impl_unpin<Self>()>(record_bit::unpin)
       | record_bit_if<rust_should_impl_send<Self>()>(record_bit::send)
       | record_bit_if<rust_should_impl_sync<Self>()>(record_bit::sync)
       | record_bit_if<rust_should_impl_drop<Self>()>(record_bit::drop)
       | record_bit_if<rust_should_impl_copy<Self>()>(record_bit::copy)
       | record_bit_if<rust_should_impl_default<Self>()>(record_bit::default_new)
       | record_bit_if<rust_should_impl_copy_new<Self>()>(record_bit::copy_new)
       | record_bit_if<rust_should_impl_move_new<Self>()>(record_bit::move_new)
       | record_bit_if<rust_should_impl_eq<Self>()>(record_bit::eq)
       | record_bit_if<rust_should_impl_partial_eq<Self>()>(record_bit::partial_eq)
       | record_bit_if<rust_should_impl_partial_ord<Self>()>(record_bit::partial_ord)
       | record_bit_if<rust_should_impl_ord<Self>()>(record_bit::ord)
       | record_bit_if<rust_should_impl_hash<Self>()>(record_bit::hash)
       | record_bit_if<rust_should_impl_debug<Self>()>(record_bit::debug)
       | record_bit_if<rust_should_impl_display<Self>()>(record_bit::display)
       | record_bit_if<cxx_has_operator_not_equal<Self>()>(record_bit::operator_not_equal)
       | record_bit_if<cxx_is_nothrow_default_constructible<Self>()>(record_bit::default_new_nothrow)
       | record_bit_if<cxx_is_nothrow_copy_constructible<Self>()>(record_bit::copy_new_nothrow)
       | record_bit_if<cxx_is_nothrow_move_constructible<Self>()>(record_bit::move_new_nothrow)
       | record_bit_if<rust_should_impl_copy_assign<Self>()>(record_bit::copy_assign)
       | record_bit_if<rust_should_impl_move_assign<Self>()>(record_bit::move_assign)
       | record_bit_if<cxx_is_nothrow_copy_assignable<Self>()>(record_bit::copy_assign_nothrow)
       | record_bit_if<cxx_is_nothrow_move_assignable<Self>()>(record_bit::move_assign_nothrow);
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
  for (
    std::string_view const field : {
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

export namespace cxx_auto {
template<typename T, typename... Args>
  requires(cxx_is_constructible<T, Args...>())
[[gnu::always_inline]]
inline auto
cxx_placement_new(T* This, Args&&... args) noexcept -> void
{
  new (This) T(std::forward<Args>(args)...);
}

template<typename T>
  requires(cxx_is_default_constructible<T>())
[[gnu::always_inline]]
inline auto
cxx_default_new(T* This) noexcept -> void
{
  cxx_placement_new(This);
}

template<typename T>
  requires(cxx_is_copy_constructible<T>())
[[gnu::always_inline]]
inline auto
cxx_copy_new(T* This, T const& that) noexcept -> void
  requires std::is_lvalue_reference_v<decltype(that)>
{
  new (This) T(that);
}

template<typename T>
  requires(cxx_is_move_constructible<T>())
[[gnu::always_inline]]
inline auto
cxx_move_new(T* This, T&& that) noexcept -> void
  requires std::is_rvalue_reference_v<decltype(that)>
{
  new (This) T(std::forward<T>(that));
}

template<typename T>
  requires(cxx_is_copy_assignable<T>())
[[gnu::always_inline]]
inline auto
cxx_copy_assign(T* This, T const& that) noexcept -> void
  requires std::is_lvalue_reference_v<decltype(that)>
{
  *This = that;
}

template<typename T>
  requires(cxx_is_move_assignable<T>())
[[gnu::always_inline]]
inline auto
cxx_move_assign(T* This, T&& that) noexcept -> void
  requires std::is_rvalue_reference_v<decltype(that)>
{
  *This = std::forward<T>(that);
}

template<typename T>
  requires(cxx_is_destructible<T>())
[[gnu::always_inline]]
inline auto
cxx_destruct(T* This) noexcept -> void
{
  std::destroy_at(This);
}

/// Evaluate the selected C++ equality relation.
/// # Specification
/// - provides: Equal when This == That, otherwise NotEqual.
/// - panics: a throwing operator terminates at this noexcept boundary.
template<typename T>
  requires(cxx_has_operator_equal<T>())
[[gnu::always_inline]]
inline auto
cxx_operator_equal(T const& This, T const& That) noexcept -> Equality
{
  return This == That ? Equality::Equal : Equality::NotEqual;
}

/// Evaluate the selected C++ inequality relation.
/// # Specification
/// - provides: NotEqual when This != That, otherwise Equal.
/// - panics: a throwing operator terminates at this noexcept boundary.
template<typename T>
  requires(cxx_has_operator_not_equal<T>())
[[gnu::always_inline]]
inline auto
cxx_operator_not_equal(T const& This, T const& That) noexcept -> Equality
{
  return This != That ? Equality::NotEqual : Equality::Equal;
}

/// Classify a standard three-way comparison for the Rust bridge.
/// # Specification
/// - provides: Less, Equivalent, Greater, or Unordered according to C++.
/// - panics: a throwing comparison terminates at this noexcept boundary.
/// # Adequacy
/// - hypothesis: weak equivalence and partial unordered results must differ from greater.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
  requires(detection::has_operator_three_way_comparison<T>)
[[gnu::always_inline]]
inline auto
cxx_operator_three_way_comparison(T const& This, T const& That) noexcept -> Comparison
{
  auto result = (This <=> That);
  // Comparison categories compare with literal zero, not nullptr.
  // NOLINTNEXTLINE(hicpp-use-nullptr,modernize-use-nullptr)
  if (result < 0) {
    return Comparison::Less;
  }
  // NOLINTNEXTLINE(hicpp-use-nullptr,modernize-use-nullptr): zero is the ordering operand.
  if (result > 0) {
    return Comparison::Greater;
  }
  // NOLINTNEXTLINE(hicpp-use-nullptr,modernize-use-nullptr): zero is the ordering operand.
  if (result == 0) {
    return Comparison::Equivalent;
  }
  return Comparison::Unordered;
}

/// Classify legacy less-than and equality for the Rust bridge.
/// # Specification
/// - provides: Less for This < That, Greater for That < This, Equivalent for
///   equality, and Unordered otherwise.
/// - panics: a throwing comparison terminates at this noexcept boundary.
/// # Adequacy
/// - hypothesis: incomparable unequal values must not be confused with greater values.
/// - witness: cxx/tests/probes.cxx (probes).
template<typename T>
  requires(
    not detection::has_operator_three_way_comparison<T>
    and detection::has_operator_less_than<T>
    and detection::has_operator_equal<T>
  )
[[gnu::always_inline]]
inline auto
cxx_operator_three_way_comparison(T const& This, T const& That) noexcept -> Comparison
{
  if (This < That) {
    return Comparison::Less;
  }
  if (That < This) {
    return Comparison::Greater;
  }
  if (This == That) {
    return Comparison::Equivalent;
  }
  return Comparison::Unordered;
}

/// Compute the selected C++ hash projection.
/// # Specification
/// - provides: the std::hash result as a nominal pointer-sized word.
/// - panics: a throwing hash function terminates at this noexcept boundary.
template<typename T>
  requires(detection::is_std_hashable<T>)
[[gnu::always_inline]]
inline auto
cxx_hash(T const& This) noexcept -> HashValue
{
  return HashValue{ std::hash<T>{}(This) };
}

/// Render the C++ debug representation.
/// # Specification
/// - provides: the streamed representation as an owned string.
/// - panics: rendering or string-allocation exceptions terminate at this noexcept boundary.
template<typename T>
  requires(detection::has_operator_ostream_left_shift<T>)
[[gnu::always_inline]]
inline auto
cxx_debug(T const& This) noexcept -> std::string
{
  std::ostringstream os;
  os << This;
  return std::move(os).str();
}

/// Render the standard string conversion.
/// # Specification
/// - provides: the std::to_string representation.
/// - panics: rendering or string-allocation exceptions terminate at this noexcept boundary.
template<typename T>
  requires(detection::has_to_string<T>)
[[gnu::always_inline]]
inline auto
cxx_display(T const& This) noexcept -> std::string
{
  return std::to_string(This);
}

/// Render the type's owned-string conversion.
/// # Specification
/// - provides: the operator std::string representation.
/// - panics: rendering or string-allocation exceptions terminate at this noexcept boundary.
template<typename T>
  requires(not detection::has_to_string<T> and detection::has_operator_std_string<T>)
[[gnu::always_inline]]
inline auto
cxx_display(T const& This) noexcept -> std::string
{
  return This.operator std::string();
}

/// Copy the type's string-view conversion.
/// # Specification
/// - provides: an owned copy of the operator std::string_view representation.
/// - panics: conversion or string-allocation exceptions terminate at this noexcept boundary.
// FIXME: optimize this to use `&str` instead of `String`
template<typename T>
  requires(
    not detection::has_to_string<T>
    and not detection::has_operator_std_string<T>
    and detection::has_operator_std_string_view<T>
  )
[[gnu::always_inline]]
inline auto
cxx_display(T const& This) noexcept -> std::string
{
  return std::string{ This.operator std::string_view() };
}

} // namespace cxx_auto
