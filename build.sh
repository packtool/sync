#!/bin/bash
pushd src/
cargo build --release
popd
cp src/target/release/packtool ./node/bin
