#!/usr/bin/sh
set -e

cd rust-guest
cargo build --release --target wasm32-unknown-unknown
cd ..

cd rust-runtime
cargo run
cd ..
