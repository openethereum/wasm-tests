#!/usr/bin/env bash

set -e

TEST_NAME=$1
ADDITIONAL_FEATURES=$2

cargo run --manifest-path ./gen/Cargo.toml -- $TEST_NAME $ADDITIONAL_FEATURES
RUSTFLAGS="-C link-arg=-z  -C link-arg=stack-size=65536" CARGO_TARGET_DIR=./target cargo build --manifest-path=./target/tests/$TEST_NAME/Cargo.toml --release --target=wasm32-unknown-unknown
wasm-build ./target $TEST_NAME --save-raw ./target/$TEST_NAME.raw.wasm --target wasm32-unknown-unknown
cp ./target/$TEST_NAME.raw.wasm ./compiled/$TEST_NAME.wasm
