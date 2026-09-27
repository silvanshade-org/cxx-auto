# cxx-auto

Generate Rust [CXX](https://cxx.rs/) bindings from C++ layout and trait capabilities that the C++ compiler records at compile time. The C++ library is the named module `cxx_auto`, built as C++26 with exceptions disabled; its macros live in `cxx-auto.hxx`, which imports it. C++ functions used across the bridge must be `noexcept`. The `cxx` crate's `c++20` feature enables its newest available bridge mode, while the compiler uses `-std=c++2c`. Clang and GCC are supported.

## Generate a binding

A crate generates its bindings in its own build script, in one pass. Nothing is linked or run to discover a type: `cxx-auto` reads each type's record from a compiled object file as data.

1. In a proxy header, invoke `CXX_AUTO_PRELUDE(Name, Type)` inside a namespace of its own, as the fixture's [proxy header](tests/fixtures/dynamic/proxy.hxx) does. This declares the helper functions the generated bridge calls.
2. In a C++ source file, invoke `CXX_AUTO_EXPORT` once per type, as the fixture's [export source](tests/fixtures/dynamic/export.cxx) does. It takes a record identifier, which must be unique within the crate, the proxy namespace, and `cxx_auto::TypeSpec` fields:
   - `rust_path`: the generated module, `::`-separated;
   - `rust_name`;
   - `rust_lifetimes`: optional, for example `'a, 'b: 'a`;
   - `cxx_name`, `cxx_namespace`, and `cxx_proxy_include`.

   A malformed spec stops the C++ compile.
3. In the build script, configure one `cc::Build` with the compiler, standard and flags that every C++ unit shares, then compile the export source together with any modules of your own: `cxx_auto::compile_modules(&base, ["src/export.cxx"], &out_dir)`. [cpp-deps](https://github.com/silvanshade-org/cpp-deps) scans every unit and compiles it after the modules it imports, including the `cxx_auto` module, so sources may be listed in any order. Clang builds also need `clang-scan-deps` on `PATH`. Then call `cxx_auto::generate(modules.objects(), &out_dir)`. It writes `out_dir/auto.rs` and one module per type beneath `out_dir/auto`, and returns the type modules. Pass those to `cxx_build::bridges` along with any bridges of your own, apply `Modules::configure` to that build so the bridges can import the modules, and link `Modules::objects` into it. The [fixture build script](tests/fixtures/dynamic/build.rs) shows the whole sequence.
4. In the crate, `include!(concat!(env!("OUT_DIR"), "/auto.rs"))` inside a module named `auto`, and use the generated types in any other CXX bridge declarations.

### What the record claims

Layout, construction, destruction, copying and moving follow the C++ type traits. The rest follow what the C++ type itself provides or claims:

| Rust trait | Implemented when |
| ---------- | ---------------- |
| `PartialEq` | `operator==` exists; `ne` calls C++ `operator!=` when the type declares one |
| `PartialOrd` | `operator<=>` exists; `partial_cmp` makes one C++ three-way comparison |
| `Eq`, `Ord` | `operator<=>` yields `std::strong_ordering`, or the type specializes `cxx_auto::rust_eq` / `cxx_auto::rust_ord` |
| `Hash` | `std::hash` is enabled for the type |
| `Send`, `Sync` | the type specializes `cxx_auto::rust_send` / `cxx_auto::rust_sync` to `true` |

Declare each specialization beside the type, in a header the export source includes before invoking `CXX_AUTO_EXPORT`. A specialization declared after that point is ill-formed and may be silently ignored.

The [dynamic binding fixture](tests/fixtures/dynamic/) is one crate with a C++ `Number` implementation. Its [integration test](tests/dynamic_binding.rs) compiles the crate as a shared library, loads it through `libloading`, and calls the generated equality and ordering implementations across equal, reversed, and integer-boundary inputs.

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
