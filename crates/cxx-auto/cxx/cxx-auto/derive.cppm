export module cxx_auto:derive;
import :satisfy;

import cxx_auto.std;

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
