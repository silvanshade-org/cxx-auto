#pragma once

// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define CXX_AUTO_PRELUDE_EXTERN_SPEC(SELF, SPEC)                 \
  extern "C" inline constexpr auto const type_spec = SPEC.ffi(); \
  extern "C" inline constexpr auto const type_elab = ::cxx_auto::TypeElabFFI::elab<SELF>();

// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define CXX_AUTO_PRELUDE_HEADER(SELF)                                                               \
  export template<typename T>                                                                       \
  auto                                                                                              \
  cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void                                \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_default_constructible<T>;                                  \
                                                                                                    \
  export template<typename T>                                                                       \
  auto                                                                                              \
  cxx_copy_new(T* This [[clang::lifetimebound]],                                                    \
               T const& that [[clang::lifetimebound]]) noexcept -> void                             \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_copy_constructible<T>;                                     \
                                                                                                    \
  export template<typename T>                                                                       \
  auto                                                                                              \
  cxx_move_new(T* This [[clang::lifetimebound]], T* that [[clang::lifetimebound]]) noexcept -> void \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_move_constructible<T>;                                     \
                                                                                                    \
  export template<typename T>                                                                       \
  auto                                                                                              \
  cxx_destruct(T* This [[clang::lifetimebound]]) noexcept -> void                                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_destructible<T>;                                           \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_equal(T const& This [[clang::lifetimebound]],                                        \
                     T const& That [[clang::lifetimebound]]) noexcept -> bool                       \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_equal<T>;                                        \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_not_equal(T const& This [[clang::lifetimebound]],                                    \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_not_equal<T>;                                    \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_less_than(T const& This [[clang::lifetimebound]],                                    \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                   \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than<T>;                                    \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_less_than_or_equal(T const& This [[clang::lifetimebound]],                           \
                                  T const& That [[clang::lifetimebound]]) noexcept -> bool          \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than_or_equal<T>;                           \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_greater_than(T const& This [[clang::lifetimebound]],                                 \
                            T const& That [[clang::lifetimebound]]) noexcept -> bool                \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than<T>;                                 \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_greater_than_or_equal(T const& This [[clang::lifetimebound]],                        \
                                     T const& That [[clang::lifetimebound]]) noexcept -> bool       \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than_or_equal<T>;                        \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                         \
                                    T const& That [[clang::lifetimebound]]) noexcept -> std::int8_t \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_has_operator_three_way_comparison<T>;                         \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> std::size_t                          \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_hashable<T>;                                               \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                      \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_debuggable<T>;                                             \
                                                                                                    \
  export template<typename T>                                                                       \
  [[nodiscard]]                                                                                     \
  auto                                                                                              \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                    \
    requires ::std::same_as<T, SELF>                                                                \
         and ::cxx_auto::satisfy::cxx_is_displayable<T>;

// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define CXX_AUTO_PRELUDE_SOURCE(SELF)                                                                   \
  template<typename T, typename... Args>                                                                \
  auto                                                                                                  \
  cxx_placement_new(T* This [[clang::lifetimebound]], Args&&... args) noexcept -> void                  \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_constructible<T, Args...>                                      \
  {                                                                                                     \
    new (This) T(::std::forward<Args>(args)...);                                                        \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  auto                                                                                                  \
  cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void                                    \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_default_constructible<T>                                       \
  {                                                                                                     \
    cxx_placement_new(This);                                                                            \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  auto                                                                                                  \
  cxx_copy_new(T* This [[clang::lifetimebound]],                                                        \
               T const& that [[clang::lifetimebound]]) noexcept -> void                                 \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_copy_constructible<T>                                          \
  {                                                                                                     \
    new (This) T(that);                                                                                 \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  auto                                                                                                  \
  cxx_move_new(T* This [[clang::lifetimebound]], T* that [[clang::lifetimebound]]) noexcept -> void     \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_move_constructible<T>                                          \
  {                                                                                                     \
    new (This) T(::std::forward<T>(that));                                                              \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  auto                                                                                                  \
  cxx_destruct(T* This [[clang::lifetimebound]]) noexcept -> void                                       \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_destructible<T>                                                \
  {                                                                                                     \
    ::std::destroy_at(This);                                                                            \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_equal(T const& This [[clang::lifetimebound]],                                            \
                     T const& That [[clang::lifetimebound]]) noexcept -> bool                           \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_has_operator_equal<T>                                             \
  {                                                                                                     \
    return (This == That);                                                                              \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_not_equal(T const& This [[clang::lifetimebound]],                                        \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                       \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_has_operator_not_equal<T>                                         \
  {                                                                                                     \
    return (This != That);                                                                              \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_less_than(T const& This [[clang::lifetimebound]],                                        \
                         T const& That [[clang::lifetimebound]]) noexcept -> bool                       \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than<T>                                         \
  {                                                                                                     \
    return (This < That);                                                                               \
  }                                                                                                     \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_less_than_or_equal(T const& This [[clang::lifetimebound]],                               \
                                  T const& That [[clang::lifetimebound]]) noexcept -> bool              \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_has_operator_less_than_or_equal<T>                                \
  {                                                                                                     \
    return (This <= That);                                                                              \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_greater_than(T const& This [[clang::lifetimebound]],                                     \
                            T const& That [[clang::lifetimebound]]) noexcept -> bool                    \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than<T>                                      \
  {                                                                                                     \
    return (This > That);                                                                               \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_greater_than_or_equal(T const& This [[clang::lifetimebound]],                            \
                                     T const& That [[clang::lifetimebound]]) noexcept -> bool           \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_has_operator_greater_than_or_equal<T>                             \
  {                                                                                                     \
    return (This >= That);                                                                              \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                             \
                                    T const& That [[clang::lifetimebound]]) noexcept -> std::int8_t     \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::detection::has_operator_three_way_comparison<T>                                \
  {                                                                                                     \
    auto result = (This <=> That);                                                                      \
    if (result < 0) {                                                                                   \
      return -1;                                                                                        \
    } else if (result > 0) { /* NOLINT(llvm-else-after-return, readability-else-after-return) */        \
      return 1;                                                                                         \
    } else if (result == 0) {                                                                           \
      return 0;                                                                                         \
    } else {                                                                                            \
      return ::std::numeric_limits<std::int8_t>::max();                                                 \
    }                                                                                                   \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_operator_three_way_comparison(T const& This [[clang::lifetimebound]],                             \
                                    T const& That [[clang::lifetimebound]]) noexcept -> std::int8_t     \
    requires ::std::same_as<T, SELF>                                                                    \
         and (not ::cxx_auto::detection::has_operator_three_way_comparison<T>)                          \
         and ::cxx_auto::detection::has_operator_less_than<T>                                           \
         and ::cxx_auto::detection::has_operator_equal<T>                                               \
  {                                                                                                     \
    auto le = (This < That);                                                                            \
    auto eq = (This == That);                                                                           \
    if (le and not eq) {                                                                                \
      return -1;                                                                                        \
    } else if (not le and not eq) { /* NOLINT(llvm-else-after-return, readability-else-after-return) */ \
      return 1;                                                                                         \
    } else if (not le and eq) {                                                                         \
      return 0;                                                                                         \
    } else {                                                                                            \
      return ::std::numeric_limits<std::int8_t>::max();                                                 \
    }                                                                                                   \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> std::size_t                              \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_hashable<T>                                                    \
  {                                                                                                     \
    return ::std::hash<T> {}(This);                                                                     \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                          \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_debuggable<T>                                                  \
  {                                                                                                     \
    ::std::ostringstream os;                                                                            \
    os << This;                                                                                         \
    return ::rust::String::lossy(os.str());                                                             \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                        \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::detection::has_operator_std_string_view<T>                                     \
  {                                                                                                     \
    return ::rust::String::lossy(::std::string { This.operator ::std::string_view() });                 \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                        \
    requires ::std::same_as<T, SELF>                                                                    \
         and (not ::cxx_auto::detection::has_operator_std_string_view<T>)                               \
         and ::cxx_auto::detection::has_operator_std_string<T>                                          \
  {                                                                                                     \
    return ::rust::String::lossy(This.operator ::std::string());                                        \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                        \
    requires ::std::same_as<T, SELF>                                                                    \
         and (not ::cxx_auto::detection::has_operator_std_string_view<T>)                               \
         and (not ::cxx_auto::detection::has_operator_std_string<T>)                                    \
         and ::cxx_auto::detection::has_to_string<T>                                                    \
  {                                                                                                     \
    return ::rust::String::lossy(::std::to_string(This));                                               \
  }                                                                                                     \
                                                                                                        \
  template<typename T>                                                                                  \
  [[nodiscard]]                                                                                         \
  auto                                                                                                  \
  cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> ::rust::String                        \
    requires ::std::same_as<T, SELF>                                                                    \
         and ::cxx_auto::satisfy::cxx_is_displayable<T>;
