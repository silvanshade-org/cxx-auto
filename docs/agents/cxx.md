# C++ boundary and probes

Build the installed header and library as C++26. Exercise header probes with GCC 16 and Clang 22; a successful Rust build with only one compiler does not cover both C++ front ends.

Run `mise run treefmt:check` on C++ changes: pinned Clang-Format 22 covers headers, fixtures, and translation units; Clang-Tidy 22 analyzes both translation units after `cargo build` generates the Rust bridge header. Its safety and performance checks complement, not replace, GCC and Clang compilation.

C++ exceptions are disabled wherever realistic (`-fno-exceptions`). Every function we define is `noexcept`. At a call site that invokes a throwing library, catch and abort or convert to an explicit result before crossing the FFI boundary; exceptions must never unwind through Rust or a C ABI boundary. RTTI is independent of exception handling.

A trait probe observes expression validity or the exact type and value category it promises, without evaluating the expression. Document the admissible types and the result in a `# Specification` block beside the probe. Pair each nontrivial probe with positive and negative compile-time witnesses on both front ends; assertions about implementation text or the presence of a definition are not witnesses. Where overload resolution, access, cv-qualification, or ref-qualification changes the answer, include a distinguishing type. The same specification and adequacy vocabulary is defined in `testing-contracts.md`.

A generated Rust artifact must preserve its declared layout, lifetime and capability boundaries. Exercise generated code at a consumer boundary, not just as a token snapshot. The emitted code and its tests must agree on what construction, destruction and trait support mean.
