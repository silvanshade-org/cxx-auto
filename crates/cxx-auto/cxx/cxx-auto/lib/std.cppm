module;

// #include "sys/types.h"

#include <algorithm>
#include <array>
#include <compare>
#include <concepts>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <iterator>
#include <limits>
#include <memory>
#include <ostream>
#include <ranges>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

export module cxx_auto.std;

// NOLINTBEGIN(cert-dcl58-cpp, misc-unused-using-decls)
export namespace std {
// <algorithm>
using ::std::transform;

// <array>
using ::std::array;

// <compare>
using ::std::partial_ordering;
using ::std::strong_ordering;

// <concepts>
using ::std::copy_constructible;
using ::std::default_initializable;
using ::std::destructible;
using ::std::equality_comparable;
using ::std::move_constructible;
using ::std::same_as;
using ::std::totally_ordered;

// <cstddef>
using ::std::size_t;

// <cstdint>
using ::std::int8_t;

// <functional>
using ::std::hash;

// <iterator>
using ::std::input_iterator;
using ::std::iter_reference_t;
using ::std::iter_value_t;

// <limits>
using ::std::numeric_limits;

// <memory>
using ::std::destroy_at;

// <ostream>
using ::std::ostream;

// <ranges>
namespace ranges {
using ::std::ranges::input_range;
using ::std::ranges::iterator_t;
using ::std::ranges::range;
} // namespace ranges

// <stdexcept>
using ::std::runtime_error;

// <sstream>
using ::std::basic_ostringstream;
using ::std::ostringstream;

// <string>
using ::std::string;
using ::std::to_string;

// <string_view>
using ::std::string_view;

// <type_traits>
using ::std::add_lvalue_reference_t;
using ::std::add_rvalue_reference_t;
using ::std::is_constructible_v;
using ::std::is_trivially_copyable_v;
using ::std::is_trivially_destructible_v;
using ::std::is_trivially_move_constructible_v;
using ::std::remove_reference_t;

// <utility>
using ::std::forward;
} // namespace std
// NOLINTEND(cert-dcl58-cpp, misc-unused-using-decls)
