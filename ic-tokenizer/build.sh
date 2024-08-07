#!/bin/bash
set -ex

export RUSTFLAGS=$RUSTFLAGS' -C target-feature=+simd128'
cargo build --release --target=wasm32-wasi
wasi2ic ./target/wasm32-wasi/release/ic_tokenizer_backend.wasm ./target/wasm32-wasi/release/ic_tokenizer_backend.wasm
wasm-opt -Os -o ./target/wasm32-wasi/release/ic_tokenizer_backend.wasm \
        ./target/wasm32-wasi/release/ic_tokenizer_backend.wasm

# candid-extractor "target/wasm32-wasi/release/ic_tokenizer_backend.wasm" > "src/ic-tokenizer-backend/ic-tokenizer-backend.did"
