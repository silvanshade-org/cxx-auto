#!/usr/bin/env bash
# GCC 16's module probes need the same workspace and Cargo source path as CXX.
set -euo pipefail
: "${GITHUB_WORKSPACE:?}"
mkdir -p "$GITHUB_WORKSPACE/target/ccache-gcc16"
docker run --rm \
  -e CCACHE_DEPEND=1 \
  -v "$GITHUB_WORKSPACE:$GITHUB_WORKSPACE" \
  -v "$HOME/.cargo/registry/src:$HOME/.cargo/registry/src:ro" \
  -v "$GITHUB_WORKSPACE/target/ccache-gcc16:/root/.cache/ccache" \
  -w "$GITHUB_WORKSPACE/target" \
  gcc:16@sha256:ef558a40d1f13115293feee01526dbdb9aaad7c9c5a00da05f471ce042e855c1 \
  sh -ec '
    apt-get update -qq && apt-get install -y -qq --no-install-recommends ccache
    flags="-std=c++2c -fmodules -fno-exceptions -fno-rtti -Werror -Wall -Wextra -pedantic -I ../cxx/include -I cxxbridge"
    ccache g++ $flags -MMD -MF cxx_auto-gcc16.d -x c++ -c ../cxx/module/cxx_auto.cppm -o cxx_auto-gcc16.o
    ccache g++ $flags -MMD -MF cxx_auto-probes-gcc16.d ../cxx/tests/probes.cxx cxx_auto-gcc16.o -o cxx-auto-probes-gcc16
  '
