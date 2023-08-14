
lol:
    cargo build --release --target wasm32-unknown-unknown --bin hello-triangle
    wasm-bindgen --no-typescript --out-dir target/generated-gpu --web "target/wasm32-unknown-unknown/release/hello-triangle.wasm"
    mv -f target/generated-gpu/* ../briefcase/examples-gpu/wasm/