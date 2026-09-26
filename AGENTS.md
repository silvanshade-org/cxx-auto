# AGENTS.md

This repository is public. Read this file and `docs/agents/baseline.md`, then every matching row before acting. The deeper page owns its subject; the root binds shared examples to this project.

| About to | Read | Never |
| -------- | ---- | ----- |
| write, change, or review Rust | `docs/agents/rust.md` | weaken the lint wall to make a change pass |
| write a test or claim evidence for a specification | `docs/agents/testing-contracts.md` | count a passing test as proof of an unobserved property |
| write or review C++ | `docs/agents/cxx.md` | allow a C++ exception to unwind into Rust |
| build, format, gate, or commit | `docs/agents/source-workflow.md` | invoke a pinned binary outside mise |
| change hosted CI | `docs/agents/ci-local.md` | replace the shared CI shape with bare-runner-only jobs |

## Vendored-page bindings

`docs/agents/rust.md`, `docs/agents/testing-contracts.md`, `docs/agents/source-workflow.md`, and `docs/agents/ci-local.md` are byte-identical shared guidance. Project-specific mappings belong here, not in those pages.

| Shared site | Project binding |
| ----------- | --------------- |
| Rust Shape: crate names, categories, and data path | The root package is `cxx-auto`; its data path is C++ capability probe → generated Rust artifact → C++ bridge. A category prefix adds no distinction to this package. |
| Rust Correctness: checker and machine | Trait detection in `cxx/include/cxx-auto.hxx` and artifact emission in `src/cxx_auto_artifact_info.rs` are the correctness engines. |
| Rust Representation: `Maybe<T, R>` | Build the type in this workspace only if an absence/reason distinction needs it. Existing exported signatures and all callers change together. |
| Rust Enforcement and Verification | `Cargo.toml` owns the workspace lint wall; `mise run check` runs local gates. Shared plugin-policy commands describe other producer and consumer roles. |
| Source workflow: release and toolchain | The root library publishes from this workspace. `rust-toolchain.toml` pins the compiler; `mise.toml` and `.config/mise/tasks/` pin the tools and gates. Codegraph is pinned in `mise.toml`; `mise exec -- codegraph init` builds each worktree’s source index. |
| Local CI: jobs and platform lanes | `.github/workflows/ci.yaml` owns this project's build, test, and documentation lanes. Cache names and compiler versions are project values; the shared CI pattern still governs changes. |

The repository is `silvanshade-org/cxx-auto`; API documentation is on docs.rs. Keep package and README URLs on those destinations, not on the retired GitHub Pages site.
