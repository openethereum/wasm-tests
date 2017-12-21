#!/bin/sh

cargo build --release --bins --target wasm32-unknown-emscripten --features "pwasm-std/panic_with_msg"
wasm-build ./target call
wasm-build ./target call_code
wasm-build ./target call_static
wasm-build ./target creator
wasm-build ./target dispersion
wasm-build ./target empty
wasm-build ./target externs
wasm-build ./target events
wasm-build ./target identity
wasm-build ./target logger
wasm-build ./target realloc
wasm-build ./target rterr
wasm-build ./target keccak
wasm-build ./target suicidal
wasm-build ./target storage_read
wasm-build ./target math
wasm-build ./target mem

cp ./target/*.wasm ./compiled
