#!/bin/sh

TEST_NAME=$1

cargo run --manifest-path ./gen/Cargo.toml -- $TEST_NAME &&
RUSTFLAGS=-g CARGO_TARGET_DIR=./target cargo build --manifest-path=./target/tests/$TEST_NAME/Cargo.toml --release --target=wasm32-unknown-unknown &&
../wabt/wasm2wat -f ./target/wasm32-unknown-unknown/release/$TEST_NAME.wasm -o ./target/$TEST_NAME-unopt.wat
wasm-build ./target $TEST_NAME --target wasm32-unknown-unknown &&
cp ./target/$TEST_NAME.wasm ./compiled