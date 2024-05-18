export module cxx_auto:satisfy;
import :detection;

import cxx_auto.std;

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
