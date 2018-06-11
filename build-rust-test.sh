#!/bin/bash

TEST_NAME=$1

cargo run --manifest-path ./gen/Cargo.toml -- $TEST_NAME &&
CARGO_TARGET_DIR=./target cargo build --manifest-path=./target/tests/$TEST_NAME/Cargo.toml --release --target=wasm32-unknown-unknown &&
wasm-build ./target $TEST_NAME --target wasm32-unknown-unknown &&
cp ./target/$TEST_NAME.wasm ./compiled
