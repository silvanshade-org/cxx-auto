# cxx-auto

Generate Rust [CXX](https://cxx.rs/) bindings from C++ layout and trait-capability probes. The C++ header is built as C++26 with exceptions disabled; C++ functions used across the bridge must be `noexcept`. The `cxx` crate's `c++20` feature enables its newest available bridge mode, while the compiler uses `-std=c++2c`.

## Generate a binding

1. Describe each C++ type in a JSON file beneath a configuration `auto` directory. Set `cxx_include` and `cxx_namespace` to the type's header and namespace. If its generated proxy functions live elsewhere, also set `cxx_proxy_include` and `cxx_proxy_namespace`. The [Number configuration](tests/fixtures/dynamic/number.json) uses a [proxy header](tests/fixtures/dynamic/probe.hxx) with `CXX_AUTO_PRELUDE` to expose the capability probes and callable helper functions.
2. Call `cxx_auto::process_artifacts(out_dir, cfg_dir)` from the probe crate's build script. This writes intermediate Rust modules to `out_dir/src/auto.rs` and its descendants. Build their generated CXX bridge alongside the proxy header, then run an executable that calls the generated `auto::process_artifacts(binding_dir)`. The executable runs the C++ probes and writes the final modules beneath `binding_dir/src/auto.rs`.
3. Build the final crate with `cxx` and `cxx-auto` as dependencies, and compile its generated bridge with `cxx-build`. The final bridge includes the configured proxy header; keep that header and the underlying type header on the C++ include path. Use the generated Rust type in any additional CXX bridge declarations.

The [dynamic binding fixture](tests/fixtures/dynamic/) contains both build stages and a C++ `Number` implementation. Its [integration test](tests/dynamic_binding.rs) compiles the final crate as a shared library, loads it through `libloading`, and calls the generated equality and ordering implementations across equal, reversed, and integer-boundary inputs.

## Verify

Install the pinned tools with `mise install`, then run `mise run check` for the workspace build, tests, lint wall, documentation, formatting, and repository gates. The C++ probe target is also exercised with GCC 16 and Clang 22 by configuring `CMakeLists.txt` with each compiler and running `ctest` on each build directory.

On macOS with Homebrew-installed `rustup`, run `rustup toolchain install` before `mise install`. The project then selects `cargo` and `rustc` from the repository-pinned toolchain rather than the Homebrew directory.

API documentation is published at [docs.rs/cxx-auto](https://docs.rs/cxx-auto/latest/cxx_auto/). Source code and issues live at [silvanshade-org/cxx-auto](https://github.com/silvanshade-org/cxx-auto).
