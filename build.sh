#!/usr/bin/env bash

set -e

for build in day*; do
    pushd $build
    cargo build
    cargo test
    cargo clippy -- -Dwarnings
    popd
done;