#!/usr/bin/sh

echo "Clearing plugins cache" && \
rm -rf plugins && mkdir plugins && \
\
echo "Building Rust plugin" && \
cd plugin-rust && cargo build --target wasm32-unknown-unknown && cd .. && \
cp plugin-rust/target/wasm32-unknown-unknown/debug/plugin_rust.wasm plugins/plugin_rust.wasm && \
\
echo "Runntig Rust wasmer host" && \
cd runtime-rust-wasmer && cargo run && cd .. && \
\
echo "Runntig Rust wasmtime host" && \
cd runtime-rust-wasmtime && cargo run && cd .. && \
\
echo "All done"
