#!/usr/bin/env bash

for build in skeleton day*; do
    pushd $build
    cargo build
    cargo test
    popd
done;