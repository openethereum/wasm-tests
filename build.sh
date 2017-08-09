#!/bin/sh

cargo build --release --bins --verbose --target wasm32-unknown-emscripten
wasm-build ./target empty
wasm-build ./target suicidal
wasm-build ./target realloc
wasm-build ./target logger

cp ./target/*.wasm ./compiled