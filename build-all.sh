#!/usr/bin/env bash

set -e

./build-rust-test.sh alloc
./build-rust-test.sh call
./build-rust-test.sh call_code
./build-rust-test.sh call_static
./build-rust-test.sh creator kip4
./build-rust-test.sh call_gasleft kip6
./build-rust-test.sh gasleft kip6
./build-rust-test.sh dispersion
./build-rust-test.sh empty
./build-rust-test.sh externs
./build-rust-test.sh events
./build-rust-test.sh identity
./build-rust-test.sh logger
./build-rust-test.sh realloc
./build-rust-test.sh rterr
./build-rust-test.sh keccak
./build-rust-test.sh suicidal
./build-rust-test.sh storage_read
./build-rust-test.sh math
./build-rust-test.sh setter
./build-wat.sh recursive
