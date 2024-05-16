module;

export module cxx_auto;

import cxx_auto.std;

namespace cxx_auto::detection {
export template<typename T, typename... U>
concept same_as_any_of = (::std::same_as<T, U> or ...);

export template<typename T>
concept has_operator_equal = requires(T const& lhs, T const& rhs) {
  { lhs == rhs } -> ::std::same_as<bool>;
};

export template<typename T>
concept has_operator_not_equal = requires(T const& lhs, T const& rhs) {
  { lhs != rhs } -> ::std::same_as<bool>;
};

export template<typename T>
concept has_operator_less_than = requires(T const& lhs, T const& rhs) {
  { lhs < rhs } -> ::std::same_as<bool>;
};

export template<typename T>
concept has_operator_less_than_or_equal = requires(T const& lhs, T const& rhs) {
  { lhs <= rhs } -> ::std::same_as<bool>;
};

export template<typename T>
concept has_operator_greater_than = requires(T const& lhs, T const& rhs) {
  { lhs > rhs } -> ::std::same_as<bool>;
};

export template<typename T>
concept has_operator_greater_than_or_equal = requires(T const& lhs, T const& rhs) {
  { lhs >= rhs } -> ::std::same_as<bool>;
};

export template<typename T>
concept has_operator_three_way_comparison = requires(T const& lhs, T const& rhs) {
  requires same_as_any_of<decltype(lhs <=> rhs), ::std::partial_ordering, ::std::strong_ordering>;
};

export template<typename T>
concept is_std_hashable = requires(T const& arg) {
  { ::std::hash<T> {}(arg) } -> ::std::same_as<::std::size_t>;
};

export template<typename T>
concept has_operator_std_string = requires(T const& arg) {
  { arg.operator ::std::string() } -> ::std::same_as<::std::string>;
};

export template<typename T>
concept has_operator_std_string_view = requires(T const& arg) {
  { arg.operator ::std::string_view() } -> ::std::same_as<::std::string_view>;
};

export template<typename T>
concept has_to_string = requires(T const& arg) {
  { ::std::to_string(arg) } -> ::std::same_as<::std::string>;
};

export template<typename T>
concept has_operator_ostream_left_shift = requires(T const& arg, ::std::ostream& os) {
  { os << arg } -> ::std::same_as<::std::ostream&>;
};

export template<typename T, typename It>
concept is_constructible_from_iterator = requires(It first, It last) {
  requires ::std::input_iterator<It>;
  { T { first, last } } -> ::std::same_as<T>;
};

export template<typename T>
concept is_iterable = ::std::ranges::range<T>;

export template<typename T, typename V>
concept is_input_iterable = requires {
  requires ::std::ranges::input_range<T>;
  requires ::std::same_as<::std::iter_value_t<::std::ranges::iterator_t<T>>, ::std::remove_reference_t<V>>;
};

export template<typename T>
concept is_input_copy_iterator = requires {
  requires ::std::input_iterator<T>;
  requires ::std::same_as<::std::iter_reference_t<T>, ::std::add_lvalue_reference_t<::std::iter_value_t<T>>>;
};

export template<typename T>
concept is_input_move_iterator = requires {
  requires ::std::input_iterator<T>;
  requires ::std::same_as<::std::iter_reference_t<T>, ::std::add_rvalue_reference_t<::std::iter_value_t<T>>>;
};
} // namespace cxx_auto::detection

namespace cxx_auto::satisfy {
export template<typename T, typename... Args>
concept cxx_is_constructible = ::std::is_constructible_v<T, Args...>;

export template<typename T>
concept cxx_is_default_constructible = ::std::default_initializable<T>;

export template<typename T>
concept cxx_is_copy_constructible = ::std::copy_constructible<T>;

export template<typename T>
concept cxx_is_move_constructible = ::std::move_constructible<T>;

export template<typename T>
concept cxx_is_destructible = ::std::destructible<T>;

export template<typename T>
concept cxx_is_trivially_copyable = ::std::is_trivially_copyable_v<T>;

export template<typename T>
concept cxx_is_trivially_movable = ::std::is_trivially_move_constructible_v<T>
                               and ::std::is_trivially_destructible_v<T>;

export template<typename T>
concept cxx_is_trivially_destructible = ::std::is_trivially_destructible_v<T>;

export template<typename T>
concept cxx_is_equality_comparable = ::std::equality_comparable<T>;

export template<typename T>
concept cxx_has_operator_equal = detection::has_operator_equal<T>;

export template<typename T>
concept cxx_has_operator_not_equal = detection::has_operator_not_equal<T>;

export template<typename T>
concept cxx_has_operator_less_than = detection::has_operator_less_than<T>;

export template<typename T>
concept cxx_has_operator_less_than_or_equal = detection::has_operator_less_than_or_equal<T>;

export template<typename T>
concept cxx_has_operator_greater_than = detection::has_operator_greater_than<T>;

export template<typename T>
concept cxx_has_operator_greater_than_or_equal = detection::has_operator_greater_than_or_equal<T>;

export template<typename T>
concept cxx_has_operator_three_way_comparison = detection::has_operator_three_way_comparison<T>
                                             or (not detection::has_operator_three_way_comparison<T>
                                                 and detection::has_operator_less_than<T>
                                                 and detection::has_operator_equal<T>);

export template<typename T>
concept cxx_is_partially_ordered = cxx_has_operator_three_way_comparison<T>;

export template<typename T>
concept cxx_is_totally_ordered = ::std::totally_ordered<T>;

export template<typename T>
concept cxx_is_hashable = detection::is_std_hashable<T>;

export template<typename T>
concept cxx_is_debuggable = detection::has_operator_ostream_left_shift<T>;

export template<typename T>
concept cxx_is_displayable = detection::has_operator_std_string_view<T>
                          or detection::has_operator_std_string<T>
                          or detection::has_to_string<T>;
} // namespace cxx_auto::satisfy

namespace cxx_auto::derive {
export template<typename T>
concept rust_should_impl_cxx_extern_type_trivial = satisfy::cxx_is_trivially_movable<T>;

export template<typename T>
concept rust_should_impl_unpin = satisfy::cxx_is_trivially_movable<T>;

export template<typename T>
concept rust_should_impl_send = false;

export template<typename T>
concept rust_should_impl_sync = false;

export template<typename T>
concept rust_should_impl_drop = satisfy::cxx_is_destructible<T>
                            and not satisfy::cxx_is_trivially_destructible<T>;

export template<typename T>
concept rust_should_impl_copy = satisfy::cxx_is_trivially_copyable<T>
                            and satisfy::cxx_is_trivially_movable<T>
                            and not rust_should_impl_drop<T>;

export template<typename T>
concept rust_should_impl_default = satisfy::cxx_is_default_constructible<T>;

export template<typename T>
concept rust_should_impl_moveref_copy_new = satisfy::cxx_is_copy_constructible<T>;

export template<typename T>
concept rust_should_impl_moveref_move_new = satisfy::cxx_is_move_constructible<T>;

export template<typename T>
concept rust_should_impl_eq = satisfy::cxx_is_equality_comparable<T>;

export template<typename T>
concept rust_should_impl_partial_eq = satisfy::cxx_has_operator_equal<T>;

export template<typename T>
concept rust_should_impl_partial_ord = satisfy::cxx_has_operator_three_way_comparison<T>
                                    or (satisfy::cxx_has_operator_less_than<T>
                                        and satisfy::cxx_has_operator_equal<T>);

export template<typename T>
concept rust_should_impl_ord = satisfy::cxx_is_totally_ordered<T>;

export template<typename T>
concept rust_should_impl_hash = satisfy::cxx_is_hashable<T>;

export template<typename T>
concept rust_should_impl_debug = satisfy::cxx_is_debuggable<T>;

export template<typename T>
concept rust_should_impl_display = satisfy::cxx_is_displayable<T>;
} // namespace cxx_auto::derive

namespace cxx_auto {
export struct alignas(32) TypeSpecLifetimeFFI
{
  char8_t const* name;
  char8_t const* const* bounds_data;
  std::size_t bounds_len;
};
static_assert(derive::rust_should_impl_cxx_extern_type_trivial<TypeSpecLifetimeFFI>);

// NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
export struct alignas(32) TypeSpecLifetime
{
  char8_t const* name;
  std::initializer_list<char8_t const*> bounds = {}; // NOLINT(readability-redundant-member-init)
};

// NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
export struct alignas(64) TypeSpec
{
  char8_t const* cc_name;
  char8_t const* cc_namespace;
  char8_t const* rs_name = u8"";
  char8_t const* rs_namespace = u8"";
  std::initializer_list<TypeSpecLifetime> rs_lifetimes = {}; // NOLINT(readability-redundant-member-init)
};

export struct alignas(64) TypeSpecFFI
{
  char8_t const* cc_name;
  char8_t const* cc_namespace;
  char8_t const* rs_name;
  char8_t const* rs_namespace;
  TypeSpecLifetimeFFI const* rs_lifetimes_data;
  std::size_t rs_lifetimes_len;
};
static_assert(derive::rust_should_impl_cxx_extern_type_trivial<TypeSpecFFI>);

export template<std::size_t len>
struct alignas(128) TypeSpecStorage
{
  explicit inline consteval TypeSpecStorage(TypeSpec const& init)
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

  explicit inline consteval operator TypeSpecFFI() const
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
  char8_t const* rs_name = u8"";
  char8_t const* rs_namespace = u8"";
  std::initializer_list<TypeSpecLifetime> rs_lifetimes;
  std::array<TypeSpecLifetimeFFI, len> rs_lifetimes_ffi;
};

export struct alignas(32) TypeElabFFI
{
  std::size_t cxx_abi_align;
  std::size_t cxx_abi_size;
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
  static inline consteval auto elab() -> TypeElabFFI
  {
    return {
      .cxx_abi_align = alignof(Self),
      .cxx_abi_size = sizeof(Self),
      .rust_should_impl_cxx_extern_type_trivial = derive::rust_should_impl_cxx_extern_type_trivial<Self>,
      .rust_should_impl_unpin = derive::rust_should_impl_unpin<Self>,
      .rust_should_impl_send = derive::rust_should_impl_send<Self>,
      .rust_should_impl_sync = derive::rust_should_impl_sync<Self>,
      .rust_should_impl_drop = derive::rust_should_impl_drop<Self>,
      .rust_should_impl_copy = derive::rust_should_impl_copy<Self>,
      .rust_should_impl_default = derive::rust_should_impl_default<Self>,
      .rust_should_impl_moveref_copy_new = derive::rust_should_impl_moveref_copy_new<Self>,
      .rust_should_impl_moveref_move_new = derive::rust_should_impl_moveref_move_new<Self>,
      .rust_should_impl_eq = derive::rust_should_impl_eq<Self>,
      .rust_should_impl_partial_eq = derive::rust_should_impl_partial_eq<Self>,
      .rust_should_impl_partial_ord = derive::rust_should_impl_partial_ord<Self>,
      .rust_should_impl_ord = derive::rust_should_impl_ord<Self>,
      .rust_should_impl_hash = derive::rust_should_impl_hash<Self>,
      .rust_should_impl_debug = derive::rust_should_impl_debug<Self>,
      .rust_should_impl_display = derive::rust_should_impl_display<Self>,
    };
  }
};
} // namespace cxx_auto
