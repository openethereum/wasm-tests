#!/usr/bin/env bash

set -e

TEST_NAME=$1

mkdir -p ./target/wasm32-unknown-unknown/release/
wat2wasm src/$TEST_NAME.wat -o ./target/wasm32-unknown-unknown/release/$TEST_NAME.wasm

wasm-build ./target $TEST_NAME --target wasm32-unknown-unknown
cp ./target/$TEST_NAME.wasm ./compiled
