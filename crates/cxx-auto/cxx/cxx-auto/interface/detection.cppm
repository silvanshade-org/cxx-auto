export module cxx_auto:detection;

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
