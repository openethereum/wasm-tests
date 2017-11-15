# wasm-tests

Growing collection of varios WASM compilable contract examples, both source and binary for integration testing in Parity.

[![Build Status](https://travis-ci.org/paritytech/wasm-tests.svg?branch=ci-setup)](https://travis-ci.org/paritytech/wasm-tests)
## Prerequisites for manual building

### Rust

- rust with `wasm32-unknown-emscripten` target - instruction to setup can be found [here](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde)
- make sure `emcc` tool is in the `PATH` since build script uses it internally
- wasm build util, run `cargo install --git https://github.com/paritytech/wasm-utils wasm-build` to install
- bash to run `./build.sh`

### C/C++
- to be updated

## Your own setup to build contracts

Along with all prerequisites for manual building listed above, make sure you put cargo `config` file in `.cargo` folder in the root of your project - the same way it is done in this repository. This config file tells cargo to use custom linker which is just a shell script `linker.sh` (and you also need this file in the root of your project). What this sort of "linker" does is just passing/substituting some arguments rust passes while invoking to build contracts. This step ensures minimal runtime usage of resulting WASM binary (`NO_FILESYSTEM`, `NO_EXIT_RUNTIME`, `USE_PTHREADS` flags) and also allows contract binaries to contain dynamic symbols which will be later linked to Parity (or Browser) runtime (`ERROR_ON_UNDEFINED_SYMBOLS` is set to 0).

## Online runner

All contracts can be also tested in online contract runner.

See this link (works only in Google Chrome and Mozilla Firefox!):

https://fckt.github.io/wasm-contract-runner/

# License

`parity-wasm` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.
