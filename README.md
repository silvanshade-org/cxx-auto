# cxx-auto

Generate Rust [CXX](https://cxx.rs/) bindings from C++ layout and trait capabilities that the C++ compiler records at compile time. The C++ library is the named module `cxx_auto`, built as C++26; its macros live in `cxx-auto.hxx`, which imports it. The `cxx` crate's `c++20` feature enables its newest available bridge mode, while the compiler uses `-std=c++2c`. Clang and GCC are supported, and generated code builds on stable Rust.

## Generate a binding

A crate generates its bindings in its own build script, in one pass. Nothing is linked or run to discover a type: `cxx-auto` reads each type's record from a compiled object file as data.

1. In a proxy header, invoke `CXX_AUTO_PRELUDE(Name, Type)` inside a namespace of its own, as the fixture's [proxy header](tests/fixtures/dynamic/proxy.hxx) does. This declares the helper functions the generated bridge calls.
2. In a C++ source file, invoke `CXX_AUTO_EXPORT` once per type, as the fixture's [export source](tests/fixtures/dynamic/export.cxx) does. It takes a record identifier, which must be unique within the crate, the proxy namespace, and `cxx_auto::TypeSpec` fields:
   - `rust_path`: the generated module, `::`-separated;
   - `rust_name`;
   - `rust_lifetimes`: optional, for example `'a, 'b: 'a`;
   - `cxx_name`, `cxx_namespace`, and `cxx_proxy_include`.

   A malformed spec stops the C++ compile.
3. In the build script, configure one `cc::Build` with the compiler, standard and flags that every C++ unit shares, then compile the export source together with any modules of your own: `let modules = cxx_auto::compile_modules(&base, ["src/export.cxx"], &out_dir)?;`. [cpp-deps](https://github.com/silvanshade-org/cpp-deps) scans every unit and compiles it after the modules it imports, including the `cxx_auto` module, so sources may be listed in any order. Clang builds also need `clang-scan-deps` on `PATH`. Then generate the bindings: `let generated = cxx_auto::generate(modules.objects(), &out_dir)?;`. It writes `out_dir/auto.rs` and one module per type beneath `out_dir/auto`, and returns the type modules. Pass those to `cxx_build::bridges` along with any bridges of your own, apply `Modules::configure` to that build so the bridges can import the modules, and link `Modules::objects` into it. The [fixture build script](tests/fixtures/dynamic/build.rs) shows the whole sequence.
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
| `Unpin` | the type is trivially move-constructible and trivially destructible, or specializes `cxx_auto::rust_relocatable` to `true` |

Declare each specialization beside the type, in a header the export source includes before invoking `CXX_AUTO_EXPORT`. A specialization declared after that point is ill-formed and may be silently ignored. Specialize `rust_relocatable` only for a type that holds no pointer into itself and that nothing else tracks by address: Rust then moves it by copying its bytes.

### Constructing C++ objects

A C++ object that isn't `Unpin` must never move bytewise, so it's built in the place that owns it. Each generated type has an inherent method for each special member C++ provides:

| Method | C++ operation |
| ------ | ------------- |
| `default_new() -> impl PinInit<Self, E>` | default constructor |
| `copy_from(&Self) -> impl PinInit<Self, E>` | copy constructor |
| `move_from(Pin<&mut Self>) -> impl PinInit<Self, E>` | move constructor; the source stays, moved-from, with its owner |
| `copy_assign(self: Pin<&mut Self>, &Self)` | copy assignment |
| `move_assign(self: Pin<&mut Self>, Pin<&mut Self>)` | move assignment |

The two assignments are generated only for a type declared `final` or one that specializes `cxx_auto::rust_assignable` to `true`. Safe Rust can reach a `Pin<&mut Base>` for the base subobject of a more-derived object, and assigning through it would rewrite the base's state beneath the derived class's invariants. Specialize `rust_assignable` only for a type no class derives from, or whose derived classes keep no invariant over its state.

An operation C++ declares `noexcept` is infallible: its initializer has error `Infallible`, and its assignment returns `()`. Any other operation returns the exception its shim caught as `cxx_auto::init::CxxException`, from the initializer or as the assignment's `Result`. Catching needs exceptions enabled in the translation unit that compiles the generated bridge; without them, a throw terminates the process at the shim.

Run an initializer in an owner from `cxx_auto::init`: `Box::pin_init`, `Rc::pin_init` or `Arc::pin_init` (and their `try_` forms, through the `InPlaceInit` trait), or a stack slot with `cxx_auto::stack_pin_init!(let x = init)` / `cxx_auto::stack_try_pin_init!(let x = init)`. The owner destroys the object when it ends, as for any Rust value. A failed initializer leaves nothing constructed.

The [dynamic binding fixture](tests/fixtures/dynamic/) is one crate with three C++ types: a trivially movable `Number`, a final `Tracked` object that aborts if it is ever found away from the address it was built at, and a `Handle` declared relocatable. Its [integration test](tests/dynamic_binding.rs) compiles the crate as a shared library, loads it through `libloading`, calls the generated equality and ordering across equal, reversed, and integer-boundary inputs, and then constructs, copies, moves, assigns and destroys `Tracked` and `Handle` objects in heap and stack places, including constructors and assignments that throw.

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

Pull requests run the light quality, format, workflow lint, and fail-open path-filter contracts; pushes, merge-queue entries, and manual runs add Rust/Clang/GCC/macOS tests with nextest JUnit reports. Native builds use ccache and emit depfiles; C++ formatting follows the reference runtime profile. Run `mise run check:ci-scripts` for path-filter boundary cases and `mise run lint:cpp` for the full gandr-style clang-tidy audit and `mise exec -- act push -j workflow-lint` for the local workflow smoke. GCC 16 and macOS keep their separate runners. If the image pull or warm run fails, the owner removes `CXX_AUTO_CI_IMAGE` to restore the bare-runner fallback; do not change package visibility or add a personal access token to repair GHCR access. When pin files change, build and preview the new tag before changing the variable.

The format lane also runs `mise run check:ci-pins`: it rejects mismatched mise releases between the hosted workflow and the verified image installer.

## Release notes

`CHANGELOG.md` records package releases under `v<version>` tags. In a release PR, fetch full history and tags, then run `mise run changelog -- --tag v<version>`. Pinned git-cliff renders committed Conventional Commits, links authored commits, and excludes merges and pre-release tags. Commit the result, land the release PR as a merge commit, then tag the release commit and push the tag. Ordinary PRs leave release history unchanged; treefmt only formats tracked Markdown.
