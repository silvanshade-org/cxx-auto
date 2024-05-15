#include "cxx-auto/cxx/include/cxx-auto.hh"

namespace cxx_auto {
TypeSpecAdapter::TypeSpecAdapter(TypeSpec const& init)
  : cc_name(init.cc_name)
  , cc_namespace(init.cc_namespace)
  , rs_name(init.rs_name)
  , rs_namespace(init.rs_namespace)
  , rs_lifetimes(init.rs_lifetimes)
  , rs_lifetimes_ffi(std::vector<TypeSpecLifetimeFFI> { rs_lifetimes.size() })
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

// NOTE: The returned `TypeSpecFFI` is only valid as long as the instance `.ffi()` was called on
// remains initialized. There is an implicit lifetime bound.
TypeSpecAdapter::operator TypeSpecFFI() const
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
} // namespace cxx_auto
