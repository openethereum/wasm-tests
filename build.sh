#!/bin/sh

mkdir -p target
input_file=$1
output_file=$2
wasm_file=$3

rustc $input_file -o $output_file -O --target wasm32-unknown-emscripten -C linker=./linker_emcc.sh
cargo run --manifest-path=./wasm-tools/ext/Cargo.toml --release -- $wasm_file $wasm_file
cargo run --manifest-path=./wasm-tools/opt/Cargo.toml --release -- $wasm_file $wasm_file
