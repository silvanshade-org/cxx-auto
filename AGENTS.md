# AGENTS.md

This repository is public. Read this file, then `docs/agents/baseline.md`, then every matching row before acting. Parent and deeper guidance apply together; nearest guidance wins on overlap.

| About to | Read | Never |
| -------- | ---- | ----- |
| write or emit anything | `docs/agents/baseline.md` | use a register or reference the baseline rules out |
| write, change, or review Rust | `docs/agents/rust.md` | weaken the lint wall to make a change pass |
| write a test or claim evidence for a specification | `docs/agents/testing-contracts.md` | count a passing test as proof of an unobserved property |
| write or review C++ | `docs/agents/cxx.md` | allow a C++ exception to unwind into Rust |
| build, format, gate, commit, or land a PR | `docs/agents/source-workflow.md` | invoke a pinned binary outside mise |
| change hosted CI | `docs/agents/ci-local.md` | replace the shared CI shape with bare-runner-only jobs |

## vendored-page-bindings

`docs/agents/rust.md`, `docs/agents/testing-contracts.md`, `docs/agents/source-workflow.md`, and `docs/agents/ci-local.md` are byte-identical copies of the shared guidance. `docs/agents/baseline.md` and `docs/agents/baseline.sha256` are a coupled copy checked by `mise run check:baseline-hash`. Project-specific bindings belong here; none weakens a shared rule.

| Shared site | Binding here | Reversal |
| ----------- | ------------ | -------- |
| Rust Shape: crate names, categories, and data path | Product package is `cxx-auto`. The product data path is C++ capability probe → generated Rust artifact → C++ bridge; a category prefix adds no distinction. | Another crate category needs a distinct axis. |
| Rust Correctness: checker and machine | Trait detection in `cxx/include/cxx-auto.hxx` and artifact emission in `src/cxx_auto_artifact_info.rs` are the correctness engines. | A separate engine owns either invariant. |
| Rust Representation: `Maybe<T, R>` | Build the type in this workspace at its first reasoned absence; change existing exported signatures and callers together. | A shared crate supplies the type. |
| Rust Enforcement and Verification | `Cargo.toml` owns the lint wall; `mise run check` runs local gates. Dylint-library and external-policy commands in the shared page describe producer and consumer roles, not commands in this workspace. | This workspace adopts that policy. |
| Source workflow: project changelog | `cliff.toml` owns release rendering. `mise run changelog` invokes pinned git-cliff on committed history only in a release PR; a release publishes a package milestone under a `v<version>` tag. Treefmt only formats tracked Markdown. | Release policy or tag namespace changes. |
| Source workflow: publication and index | Root library publishes from this workspace. `mise.toml` pins Codegraph; `mise exec -- codegraph init` indexes each worktree after its source exists. | Publication or indexing workflow changes. |
| Local CI: jobs, image, and platform lanes | `.github/workflows/ci.yaml` owns Rust, C++26, format, and workflow checks; `.github/workflows/ci-image.yaml` builds the shared container image. The owner-set `CXX_AUTO_CI_IMAGE` or manual `ci_image` input selects a published image; empty both selects bootstrap bare runners. GCC 16 and macOS retain their own lanes. | A measured image rollback or platform change. |

The repository is `silvanshade-org/cxx-auto`; API documentation is on docs.rs. Package and README URLs point to those destinations.
