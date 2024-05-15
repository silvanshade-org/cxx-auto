#include "cxx-auto/cxx/include/cxx-auto.hh"

extern "C" char const* const HELLO;

namespace example {
class Foo
{};
CXX_AUTO_PRELUDE_HEADER(example::Foo)
} // namespace example
