#!/bin/sh

set -e

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build --profiling --out-dir ../scenarios/ms-web-plat/pkg --target bundler
