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

## CI image

The [image workflow](.github/workflows/ci-image.yaml) builds the pinned toolchain natively for amd64 and arm64 on `main`, publishes a GHCR manifest by digest, and tags it with the SHA-256 of `mise.toml`, `mise.lock`, and `rust-toolchain.toml` concatenated in that order. Its `workflow_dispatch` can rebuild the same pin. The repository variable `CXX_AUTO_CI_IMAGE` remains empty until the image is proven.

On `main`, dispatch the image builder if these pinned inputs have no successful image run:

```sh
mise exec -- gh workflow run ci-image.yaml --repo silvanshade-org/cxx-auto --ref main
```

Wait for the builder’s `manifest` job to pass and publish its GHCR tag. From that same `main` revision, run a full hosted gate against the image without changing the repository variable:

```sh
IMAGE="ghcr.io/silvanshade-org/cxx-auto-ci:$(cat mise.toml mise.lock rust-toolchain.toml | sha256sum | cut -d ' ' -f1)"
mise exec -- gh workflow run ci.yaml --repo silvanshade-org/cxx-auto --ref main -f ci_image="$IMAGE"
```

After the preview proves an actual Linux image pull and all applicable Rust, GCC 16, Clang 22, and macOS jobs pass, a repository administrator can activate it and verify the repository-variable path:

```sh
mise exec -- gh variable set CXX_AUTO_CI_IMAGE --repo silvanshade-org/cxx-auto --body "$IMAGE"
mise exec -- gh workflow run ci.yaml --repo silvanshade-org/cxx-auto --ref main
```

Merge-queue and manual CI runs force the full source gate; documentation-only pull requests may skip source lanes but still run formatting and workflow lint. GCC 16 and macOS keep their separate runners. If the image pull or warm run fails, the owner removes `CXX_AUTO_CI_IMAGE` to restore the bare-runner fallback; do not change package visibility or add a personal access token to repair GHCR access. When pin files change, build and preview the new tag before changing the variable.

## Release notes

`CHANGELOG.md` is generated from the full tagged Git history by the private [changelog crate](crates/changelog/) with `mise run changelog:render`; this tool does not build the product's C++ bridge. On a branch with an open GitHub pull request, the renderer reads its proposed squash subject and live `origin` base revision, then substitutes that subject and review number for temporary branch commits; on `main`, it uses the landed commits. It refuses a missing or stale review base and leaves an already-current changelog untouched. After changing the pull request title or rebasing, rerun the task and commit the updated file; `mise run check:format` checks it.
