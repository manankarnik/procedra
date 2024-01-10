cd utils/
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --bin map --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../src/utils/map --out-name index --target web ./target/wasm32-unknown-unknown/release/map.wasm
cargo build --release --bin terrain --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../src/utils/terrain --out-name index --target web ./target/wasm32-unknown-unknown/release/terrain.wasm
cargo build --release --bin planet --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../src/utils/planet --out-name index --target web ./target/wasm32-unknown-unknown/release/planet.wasm
cd ../
bun x prisma generate
bun run build
