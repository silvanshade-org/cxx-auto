#pragma once

// The cxx-auto macros. The library itself is the `cxx_auto` module
// (`cxx/module/cxx_auto.cppm`), which a build compiles before any translation
// unit including this header; see `cxx_auto::modules`. Macro expansions name
// the declarations below directly, and a module's global fragment is not
// visible to its importers, so this header includes them itself.
#include "cxx-auto-ctypes.hxx"
#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <string>
#include <string_view>
#include <utility>

import cxx_auto;

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
    return ::rust::String::lossy(::cxx_auto::cxx_debug(This));                                                         \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_displayable<T>())                                            \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_display(T const& This) noexcept -> rust::string                                               \
  {                                                                                                                    \
    return ::rust::String::lossy(::cxx_auto::cxx_display(This));                                                       \
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
