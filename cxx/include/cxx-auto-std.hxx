#ifndef CXX_AUTO_STD_HXX
#define CXX_AUTO_STD_HXX

// Both the module's global fragment and its importing macro header include this
// prelude. GCC must see these declarations textually before the module import;
// otherwise a consumer's later standard-header include can redeclare imported
// implementation types. Keep the module's standard-library dependencies here.
#include <array>
#include <compare>
#include <concepts>
#include <cstddef>
#include <cstdint>
#include <exception>
#include <functional>
#include <iterator>
#include <limits>
#include <memory>
#include <ostream>
#include <ranges>
#include <sstream>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

#endif // CXX_AUTO_STD_HXX
