cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "flappy-ball" \
    ./target/wasm32-unknown-unknown/release/flappy-ball.wasm
