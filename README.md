# wasm-tests

Growing collection of varios WASM compilable contract examples, both source and binary for integration testing in Parity.

## Prerequisites for manual building

### Rust

- rustc with `wasm32-unknown-emscripten` target - instruction to setup can be found [here](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde)
- make sure `emcc` tool is in the `PATH` since build script uses it internally
- bash to run `./build-all.sh`

### C/C++
- to be updated
