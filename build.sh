cd utils/
cargo build --release --bin map --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../src/utils/map --out-name index --target web ./target/wasm32-unknown-unknown/release/map.wasm
cargo build --release --bin terrain --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../src/utils/terrain --out-name index --target web ./target/wasm32-unknown-unknown/release/terrain.wasm
cargo build --release --bin planet --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../src/utils/planet --out-name index --target web ./target/wasm32-unknown-unknown/release/planet.wasm
cd ../
bun run build
