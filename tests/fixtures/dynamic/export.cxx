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
