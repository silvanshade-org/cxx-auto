export module cxx_auto:ffi.specification;
import :derive;

import cxx_auto.std;

namespace cxx_auto {
namespace type_spec {
namespace lifetime {

export struct alignas(32) FFI final
{
  char8_t const* name;
  char8_t const* const* bounds_data;
  std::size_t bounds_len;
};
static_assert(derive::rust_should_impl_cxx_extern_type_trivial<FFI>);
} // namespace lifetime

export template<std::size_t b_len>
struct [[gnu::aligned(16)]]
Lifetime final // NOLINT(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
{
  // NOLINTBEGIN(misc-non-private-member-variables-in-classes)
  char8_t const* name;
  std::array<char8_t const*, b_len> bounds {};
  // NOLINTEND(misc-non-private-member-variables-in-classes)

  [[nodiscard]]
  inline constexpr auto ffi() const -> lifetime::FFI
  {
    return {
      .name = this->name,
      .bounds_data = this->bounds.data(), // NOLINT(cppcoreguidelines-pro-bounds-array-to-pointer-decay, hicpp-no-array-decay)
      .bounds_len = b_len,
    };
  }
};

// NOTE: We should really be using `std::initializer_list` rather than arrays here since they would
// allow us to erase the size parameters. In clang, using `std::initializer_list` does work, but in
// GCC it does not.
//
// The problem GCC complains about is: "pointer to subobject of temporary is not a constant
// expression", specifically for the bounds aggregate.
//
// This proposal (https://wg21.link/P2752R3) may be relevant, but although it is supposed to be
// implemented in GCC 14, it does not appear to make a difference.
export template<std::size_t l_len, std::size_t b_len>
struct [[gnu::aligned(64)]]
Data final // NOLINT(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
{
  char8_t const* cc_name;
  char8_t const* cc_namespace;
  char8_t const* rs_name {};
  char8_t const* rs_namespace {};
  std::array<Lifetime<b_len>, l_len> rs_lifetimes {};
};

export struct alignas(64) FFI final
{
  // NOLINTBEGIN(misc-non-private-member-variables-in-classes)
  char8_t const* cc_name;
  char8_t const* cc_namespace;
  char8_t const* rs_name;
  char8_t const* rs_namespace;
  type_spec::lifetime::FFI const* rs_lifetimes_data;
  std::size_t rs_lifetimes_len;
  // NOLINTEND(misc-non-private-member-variables-in-classes)
};
static_assert(derive::rust_should_impl_cxx_extern_type_trivial<type_spec::FFI>);
}; // namespace type_spec

export template<std::size_t l_len = 0, std::size_t b_len = 0>
struct [[gnu::aligned(128)]]
TypeSpec final
{
private:
  type_spec::Data<l_len, b_len> data;
  std::array<type_spec::lifetime::FFI, l_len> rs_lifetimes_data {};

public:
  explicit inline constexpr TypeSpec(type_spec::Data<l_len, b_len>&& data)
    : data(data)
  {
    for (std::size_t i = 0; i < l_len; ++i) {
      this->rs_lifetimes_data[i] = this->data.rs_lifetimes[i].ffi(); // NOLINT(cppcoreguidelines-pro-bounds-constant-array-index)
    }
    this->validate();
  }

  [[nodiscard]]
  inline constexpr auto ffi() const -> type_spec::FFI
  {
    auto const& ffi = type_spec::FFI {
      .cc_name = this->data.cc_name,
      .cc_namespace = this->data.cc_namespace,
      .rs_name = this->data.rs_name,
      .rs_namespace = this->data.rs_namespace,
      .rs_lifetimes_data = this->rs_lifetimes_data.data(), // NOLINT(cppcoreguidelines-pro-bounds-array-to-pointer-decay, hicpp-no-array-decay)
      .rs_lifetimes_len = l_len,
    };
    // ffi.validate();
    return ffi;
  }

  // NOLINTNEXTLINE(readability-function-cognitive-complexity)
  inline constexpr void validate() const
  {
    // NOLINTBEGIN(cppcoreguidelines-pro-bounds-pointer-arithmetic, cppcoreguidelines-pro-bounds-constant-array-index)
    if (nullptr == this->data.cc_name) {
      throw std::runtime_error("invalid empty `cc_name`");
    }
    if (nullptr == this->data.cc_namespace) {
      throw std::runtime_error("invalid empty `cc_namespace`");
    }
    for (std::size_t i = 0; i < l_len; ++i) {
      if (nullptr == this->rs_lifetimes_data[i].name) {
        throw std::runtime_error("invalid NULL lifetime `name`");
      }
      if ('\0' == this->rs_lifetimes_data[i].name[0]) {
        throw std::runtime_error("invalid empty lifetime `name`");
      }
      std::size_t j = 0;
      for (; j < this->rs_lifetimes_data[i].bounds_len; ++j) {
        // if we encounter a `nullptr` for a bound...
        if (nullptr == this->rs_lifetimes_data[i].bounds_data[j]) {
          // ... then ensure all the remaining are also `nullptr`
          while (j < this->rs_lifetimes_data[i].bounds_len) {
            if (nullptr != this->rs_lifetimes_data[i].bounds_data[j]) {
              throw std::runtime_error("invalid bounds array (contains non-NULL after initial NULL)");
            }
            ++j;
          }
          break;
        }
        // rule-out empty strings for bound
        if ('\0' == this->rs_lifetimes_data[i].bounds_data[j][0]) {
          throw std::runtime_error("invalid NULL empty lifetime bound");
        }
      }
      // ensure all remaining bounds (including the first) are `nullptr`
      while (j < this->rs_lifetimes_data[i].bounds_len) {
        if (nullptr != this->rs_lifetimes_data[i].bounds_data[j]) {
          throw std::runtime_error("invalid bounds array (contains non-NULL after initial NULL)");
        }
        ++j;
      }
    }
    // NOLINTEND(cppcoreguidelines-pro-bounds-pointer-arithmetic, cppcoreguidelines-pro-bounds-constant-array-index)
  }
};
} // namespace cxx_auto
