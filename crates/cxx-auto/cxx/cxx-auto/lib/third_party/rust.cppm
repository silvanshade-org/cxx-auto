module;

#include "rust/cxx.h"

export module cxx_auto.rust;

// NOLINTBEGIN(cert-dcl58-cpp, misc-unused-using-decls)
export namespace rust {
// NOTE: GCC (unlike clang) does not re-export this if the name is not redeclared `String = ...`.
// This may be due to the class being declared `final`.
using String = ::rust::String;
} // namespace rust
// NOLINTEND(cert-dcl58-cpp, misc-unused-using-decls)
