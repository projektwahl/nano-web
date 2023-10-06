# nano-web

https://developer.mozilla.org/en-US/docs/WebAssembly/Using_the_JavaScript_API
https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format
https://surma.dev/things/rust-to-webassembly/

https://github.com/rust-lang/rust/issues/60825#issuecomment-566273568 Custom globals not supported

rustc --target wasm32-unknown-unknown --print target-features

```bash
cargo build -Z build-std-features=panic_immediate_abort --target wasm32-unknown-unknown --release && wasm2wat --enable-threads target/wasm32-unknown-unknown/release/nano_web.wasm -o target/wasm32-unknown-unknown/release/nano_web.wat
```