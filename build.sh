#! /bin/bash

if [ ! -d wasm ]
then
    mkdir wasm
    cp src/index.html wasm
fi

cargo build --release --target wasm32-unknown-unknown && \
    wasm-bindgen target/wasm32-unknown-unknown/release/pet-dungeon.wasm --out-dir wasm --no-modules --no-typescript
