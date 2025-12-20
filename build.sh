#!/bin/bash
set -e

echo "Building Rust Core..."
cd rust
wasm-pack build --target web
cd ..

echo "Build Complete."
