#!/bin/sh

./build.sh ./src/empty.rs ./target/empty.js ./target/empty.wasm
./build.sh ./src/identity.rs ./target/identity.js ./target/identity.wasm
./build.sh ./src/logger.rs ./target/logger.js ./target/logger.wasm
cp ./target/*.wasm ./compiled/