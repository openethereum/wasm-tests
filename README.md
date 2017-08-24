# wasm-tests

Growing collection of varios WASM compilable contract examples, both source and binary for integration testing in Parity.

## Prerequisites for manual building

### Rust

- rustc with `wasm32-unknown-emscripten` target - instruction to setup can be found [here](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde)
- make sure `emcc` tool is in the `PATH` since build script uses it internally
- wasm build util, run `cargo install --git https://github.com/paritytech/wasm-utils wasm-build` to install
- bash to run `./build.sh`

### C/C++
- to be updated

### Online runner

All contracts can be also tested in online contract runner.

See this link (works only in Google Chrome and Mozilla Firefox!):

https://fckt.github.io/wasm-contract-runner/
