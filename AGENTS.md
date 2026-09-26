# AGENTS.md

This repository is public. Read this file and `docs/agents/baseline.md`, then every matching row before acting. The deeper page owns its subject; the root binds shared examples to this project.

| About to | Read | Never |
| -------- | ---- | ----- |
| write, change, or review Rust | `docs/agents/rust.md` | weaken the lint wall to make a change pass |
| write a test or claim evidence for a specification | `docs/agents/testing-contracts.md` | count a passing test as proof of an unobserved property |
| write or review C++ | `docs/agents/cxx.md` | allow a C++ exception to unwind into Rust |
| build, format, gate, or commit | `docs/agents/source-workflow.md` | assume tasks from the later implementation layer already exist here |
| change hosted CI | `docs/agents/ci-local.md` | replace the shared CI shape with bare-runner-only jobs |

## Vendored-page bindings

`docs/agents/rust.md`, `docs/agents/testing-contracts.md`, `docs/agents/source-workflow.md`, and `docs/agents/ci-local.md` are byte-identical shared guidance. Project-specific mappings belong here, not in those pages.

| Shared site | Project binding |
| ----------- | --------------- |
| Rust Shape: crate names, categories, and data path | The root package is `cxx-auto`; the existing `xtask` is repository tooling. The path is C++ capability probe → generated Rust artifact → C++ bridge. Do not invent a category prefix for two packages. |
| Rust Correctness: checker and machine | Trait detection in `cxx/include/cxx-auto.hxx` and artifact emission in `src/cxx_auto_artifact_info.rs` are the correctness engines. |
| Rust Representation: `Maybe<T, R>` | Build the type in this workspace only if an absence/reason distinction needs it. Existing exported signatures and all callers change together. |
| Rust Enforcement and Verification | `rust-toolchain.toml` pins this layer’s compiler; `cargo xtask` exposes its existing checks. The `cxx-auto-modernize-implementation` layer adds the workspace lint wall and `mise run check`. Shared plugin-policy commands are source examples, not installed here. |
| Source workflow: release and toolchain | The root crate publishes from this workspace. This layer has `rust-toolchain.toml` and `cargo xtask`; `cxx-auto-modernize-implementation` adds `mise.toml`, `.config/mise/tasks/`, and the project-local Codegraph pin. Each worktree builds its own index there. |
| Local CI: jobs and platform lanes | `.github/workflows/ci.yaml` owns this project's build, test, and documentation lanes. Cache names and compiler versions are project values; the shared CI pattern still governs changes. |

The repository is `silvanshade-org/cxx-auto`; API documentation is on docs.rs. Keep package and README URLs on those destinations, not on the retired GitHub Pages site.
