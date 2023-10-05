Nightly Rust needed for building std

```bash
wasm-pack build --reference-types --profiling . -- -Z build-std=core,std,panic_abort -Z build-std-features=panic_immediate_abort && ls -l pkg/ && ls -lh pkg/ && wasm-decompile --output=pkg/nano_web_bg.dcmp pkg/nano_web_bg.wasm

twiggy top -n 20 pkg/nano_web_bg.wasm


cargo rustc --release -- --emit llvm-ir
find target/release -type f -name '*.ll'
```

https://github.com/rustwasm/wasm-pack/issues/737

https://rustwasm.github.io/docs/book/reference/code-size.html

https://rustwasm.github.io/docs/book/reference/tools.html