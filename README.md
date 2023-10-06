# nano-web

https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format

```bash
cargo build --target wasm32-unknown-unknown --release && wasm2wat target/wasm32-unknown-unknown/release/nano_web.wasm -o target/wasm32-unknown-unknown/release/nano_web.wat
```