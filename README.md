Nightly Rust needed for building std

wasm_bindgen_convert_closures_invoke3_mut_h6effd6ec78924e06

find out why it allocates and fix that

```bash
wasm-pack build --profiling . -- -Z build-std=core,std,panic_abort -Z build-std-features=panic_immediate_abort && ls -l pkg/ && ls -lh pkg/ && wasm2wat pkg/nano_web_bg.wasm -o pkg/nano_web_bg.wat 

# bundler, nodejs, web, no-modules

# https://github.com/rustwasm/wasm-bindgen/blob/main/src/convert/closures.rs

# the wat format is pretty good

twiggy top -n 20 pkg/nano_web_bg.wasm


cargo rustc --release -- --emit llvm-ir
find target/release -type f -name '*.ll'
```

https://github.com/rustwasm/wasm-pack/issues/737

https://rustwasm.github.io/docs/book/reference/code-size.html

https://rustwasm.github.io/docs/book/reference/tools.html