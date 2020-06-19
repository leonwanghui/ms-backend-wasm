#!/bin/sh

set -e

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build --profiling --out-dir ../pkg --target bundler
