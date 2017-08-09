#!/bin/sh

cargo build --release --bins --verbose --target wasm32-unknown-emscripten
wasm-build ./target empty
wasm-build ./target suicidal
wasm-build ./target realloc
wasm-build ./target logger
wasm-build ./target identity
wasm-build ./target dispersion
wasm-build ./target creator
wasm-build ./target call_code
wasm-build ./target call_static
wasm-build ./target rterr

cp ./target/*.wasm ./compiled