#pragma once

// Include cxx_auto first so later standard headers exercise the GCC import boundary.
#include "cxx-auto/cxx/include/cxx-auto.hxx"
#include "number.hxx"
#include "objects.hxx"

// `Handle` owns its allocation through a plain pointer, so moving its bytes
// and abandoning the old storage is sound.
template<>
inline constexpr bool cxx_auto::rust_relocatable<demo::Handle> = true;

// One prelude per proxy namespace: each binds that namespace's `Self`.
namespace demo::proxy {
CXX_AUTO_PRELUDE(Number, demo::Number)
} // namespace demo::proxy

namespace demo::tracked_proxy {
CXX_AUTO_PRELUDE(Tracked, demo::Tracked)
} // namespace demo::tracked_proxy

namespace demo::handle_proxy {
CXX_AUTO_PRELUDE(Handle, demo::Handle)
} // namespace demo::handle_proxy

namespace demo::partial_proxy {
CXX_AUTO_PRELUDE(PartialNumber, demo::PartialNumber)
}

namespace demo::weak_proxy {
CXX_AUTO_PRELUDE(WeakNumber, demo::WeakNumber)
}

namespace demo::legacy_proxy {
CXX_AUTO_PRELUDE(LegacyNumber, demo::LegacyNumber)
}

namespace demo::throwing_proxy {
CXX_AUTO_PRELUDE(Throwing, demo::Throwing)
}
