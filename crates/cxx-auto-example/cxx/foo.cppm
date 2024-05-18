module;

#include "cxx-auto/macros.hpp"

export module cxx_auto_example.foo;

import cxx_auto.std;
import cxx_auto.rust;
import cxx_auto;

namespace example {
export class Foo
{};

// clang-format off
export inline constexpr auto const spec = ::cxx_auto::TypeSpec<3, 2>({
  .cc_name = u8"Foo",
  .cc_namespace = u8"example",
  .rs_lifetimes = {
    { .name = u8"a"
    , .bounds { u8"a", u8"b" } },
    { .name = u8"c"
    , .bounds { u8"x" } },
    { .name = u8"d"
    , .bounds { u8"z", u8"h" } }
  }
});
// clang-format on

CXX_AUTO_PRELUDE_HEADER(Foo)
} // namespace example
