#!/bin/sh

./build.sh ./src/empty.rs ./target/empty.js ./target/empty.wasm
./build.sh ./src/identity.rs ./target/identity.js ./target/identity.wasm
./build.sh ./src/logger.rs ./target/logger.js ./target/logger.wasm
./build.sh ./src/dispersion.rs ./target/dispersion.js ./target/dispersion.wasm
./build.sh ./src/suicidal.rs ./target/suicidal.js ./target/suicidal.wasm
./build.sh ./src/creator.rs ./target/creator.js ./target/creator.wasm
./build.sh ./src/call_code.rs ./target/call_code.js ./target/call_code.wasm
./build.sh ./src/call_static.rs ./target/call_static.js ./target/call_static.wasm
./build.sh ./src/rterr.rs ./target/rterr.js ./target/rterr.wasm
./build.sh ./src/realloc.rs ./target/realloc.js ./target/realloc.wasm no-ext
./build.sh ./src/storage_read.rs ./target/storage_read.js ./target/storage_read.wasm
cp ./target/*.wasm ./compiled/