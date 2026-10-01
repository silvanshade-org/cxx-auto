#include "proxy.hxx"

CXX_AUTO_EXPORT(
  number,
  demo::proxy,
  .rust_path = "number",
  .rust_name = "Number",
  .cxx_name = "Number",
  .cxx_namespace = "demo",
  .cxx_proxy_include = "proxy.hxx"
)

CXX_AUTO_EXPORT(
  tracked,
  demo::tracked_proxy,
  .rust_path = "tracked",
  .rust_name = "Tracked",
  .cxx_name = "Tracked",
  .cxx_namespace = "demo",
  .cxx_proxy_include = "proxy.hxx"
)

CXX_AUTO_EXPORT(
  handle,
  demo::handle_proxy,
  .rust_path = "handle",
  .rust_name = "Handle",
  .cxx_name = "Handle",
  .cxx_namespace = "demo",
  .cxx_proxy_include = "proxy.hxx"
)

CXX_AUTO_EXPORT(
  partial_number,
  demo::partial_proxy,
  .rust_path = "partial_number",
  .rust_name = "PartialNumber",
  .cxx_name = "PartialNumber",
  .cxx_namespace = "demo",
  .cxx_proxy_include = "proxy.hxx"
)
CXX_AUTO_EXPORT(
  weak_number,
  demo::weak_proxy,
  .rust_path = "weak_number",
  .rust_name = "WeakNumber",
  .cxx_name = "WeakNumber",
  .cxx_namespace = "demo",
  .cxx_proxy_include = "proxy.hxx"
)
CXX_AUTO_EXPORT(
  legacy_number,
  demo::legacy_proxy,
  .rust_path = "legacy_number",
  .rust_name = "LegacyNumber",
  .cxx_name = "LegacyNumber",
  .cxx_namespace = "demo",
  .cxx_proxy_include = "proxy.hxx"
)
CXX_AUTO_EXPORT(
  throwing,
  demo::throwing_proxy,
  .rust_path = "throwing",
  .rust_name = "Throwing",
  .cxx_name = "Throwing",
  .cxx_namespace = "demo",
  .cxx_proxy_include = "proxy.hxx"
)
