#!/usr/bin/env bash

set -e

for build in skeleton day*; do
    pushd $build
    cargo build
    cargo test
    popd
done;