#!/usr/bin/sh
set -e

cd rust-guest
cargo build --release --target wasm32-unknown-unknown
cd ..

cd rust-runtime
wasm-pack build --release --target web
docker-compose down
docker-compose up -d
echo View the webpage at http://127.0.0.1:8100
cd ..
