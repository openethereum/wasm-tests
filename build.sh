rustc $file -o out/contract.js -O --target wasm32-unknown-emscripten -C linker=./linker_emcc.sh
cargo run --manifest-path=./../ext/Cargo.toml --release -- ./out/contract.wasm ./out/contract.wasm
cargo run --manifest-path=./../opt/Cargo.toml --release -- ./out/contract.wasm ./out/contract.wasm
