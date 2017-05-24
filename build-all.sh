#!/bin/sh

./build.sh ./src/empty.rs ./target/empty.js ./target/empty.wasm
./build.sh ./src/static.rs ./target/static.js ./target/static.wasm
./build.sh ./src/logger.rs ./target/logger.js ./target/logger.wasm
cp ./target/*.wasm ./compiled/