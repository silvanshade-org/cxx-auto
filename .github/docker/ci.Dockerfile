# GCC 16 comes from this digest-pinned upstream image. LLVM 22 and the
# repository's Rust/mise pins are installed once, rather than on every job.
FROM gcc:16@sha256:ef558a40d1f13115293feee01526dbdb9aaad7c9c5a00da05f471ce042e855c1

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates cmake curl git gnupg jq nodejs npm pkg-config && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://apt.llvm.org/llvm-snapshot.gpg.key | gpg --dearmor -o /usr/share/keyrings/llvm-archive-keyring.gpg \
    && echo 'deb [signed-by=/usr/share/keyrings/llvm-archive-keyring.gpg] https://apt.llvm.org/trixie/ llvm-toolchain-trixie-22 main' > /etc/apt/sources.list.d/llvm-22.list \
    && apt-get update && apt-get install -y --no-install-recommends clang-22 && rm -rf /var/lib/apt/lists/*

ENV MISE_DATA_DIR=/opt/mise CARGO_HOME=/opt/cargo RUSTUP_HOME=/opt/rustup
ENV PATH="/opt/cargo/bin:/opt/mise/shims:/usr/lib/llvm-22/bin:${PATH}"
# These tools are for local editing, hooks, indexing, act, and workflow lint;
# every container-backed gate's tools remain installed. Keep this list
# identical in ci.yaml.
ENV MISE_DISABLE_TOOLS="github:colbymchenry/codegraph,github:max-sixty/worktrunk,github:j178/prek,github:nektos/act,github:woodruffw/zizmor,npm:@commitlint/cli"
WORKDIR /workspace
COPY mise.toml mise.lock rust-toolchain.toml ./

ARG TARGETARCH
RUN set -eu; \
    case "$TARGETARCH" in \
      amd64) arch=x64; rust_target=x86_64-unknown-linux-gnu; mise_sha=a6dea05e896f1e6090f821588f45397262270e0e0b456252f8d1da28a416f3f2; rust_sha=dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71 ;; \
      arm64) arch=arm64; rust_target=aarch64-unknown-linux-gnu; mise_sha=124ea8f7c8cb9a6a3c99c763cbf37ca48c9beaa816735f011d9fd99e6cd463e9; rust_sha=15f6e4ce9f583b929c996c91562bad6d4454f3281de858b02cdfdef615fac433 ;; \
      *) echo "unsupported target architecture: $TARGETARCH" >&2; exit 1 ;; \
    esac; \
    curl -fsSL "https://github.com/jdx/mise/releases/download/v2026.8.15/mise-v2026.8.15-linux-${arch}" -o /usr/local/bin/mise; \
    printf '%s  %s\n' "$mise_sha" /usr/local/bin/mise | sha256sum -c -; \
    chmod +x /usr/local/bin/mise; \
    curl -fsSL "https://static.rust-lang.org/rustup/archive/1.29.1/${rust_target}/rustup-init" -o /tmp/rustup-init; \
    printf '%s  %s\n' "$rust_sha" /tmp/rustup-init | sha256sum -c -; \
    chmod +x /tmp/rustup-init; \
    /tmp/rustup-init -y --no-modify-path --profile minimal --default-toolchain none; \
    rustup toolchain install; \
    mise install --locked; \
    g++ --version; \
    clang++-22 --version; \
    mise exec -- cargo --version
