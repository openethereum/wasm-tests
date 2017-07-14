#!/bin/sh

./build.sh ./src/empty.rs ./target/empty.js ./target/empty.wasm
./build.sh ./src/identity.rs ./target/identity.js ./target/identity.wasm
./build.sh ./src/logger.rs ./target/logger.js ./target/logger.wasm
./build.sh ./src/dispersion.rs ./target/dispersion.js ./target/dispersion.wasm
./build.sh ./src/suicidal.rs ./target/suicidal.js ./target/suicidal.wasm
./build.sh ./src/creator.rs ./target/creator.js ./target/creator.wasm
./build.sh ./src/caller.rs ./target/caller.js ./target/caller.wasm
cp ./target/*.wasm ./compiled/