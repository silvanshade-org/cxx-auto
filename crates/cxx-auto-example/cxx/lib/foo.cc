#include "cxx-auto-example/cxx/include/foo.hh"

#include "cxx-auto/cxx/include/cxx-auto.hh"

namespace example {
constexpr ::cxx_auto::TypeSpec const spec = {
  .cc_name = "Foo",
  .cc_namespace = "example",
  .rs_lifetimes = {
    { .name = "a",
      .bounds {
        "a",
        "b" } } }
};
CXX_AUTO_PRELUDE_SOURCE(example::Foo, spec)
} // namespace example
