#include "cxx-auto-example/cxx/include/foo.hh"

#include "cxx-auto/cxx/include/cxx-auto.hh"

namespace example {
// clang-format off
constexpr ::cxx_auto::TypeSpec const spec = {
  .cc_name = u8"Foo",
  .cc_namespace = u8"example",
  .rs_lifetimes {
    { .name = u8"a"
    , .bounds { u8"a", u8"b" } },
    { .name = u8"c"
    , .bounds { u8"x" } },
    { .name = u8"d"
    , .bounds { u8"z", u8"h" } }
  }
};
// clang-format on
CXX_AUTO_PRELUDE_SOURCE(example::Foo, spec)
} // namespace example
