Nightly Rust needed for building std

wasm_bindgen_convert_closures_invoke3_mut_h6effd6ec78924e06

find out why it allocates and fix that

```bash
cargo rustc --release --target wasm32-unknown-unknown -Z build-std=core,std,panic_abort -Z build-std-features=panic_immediate_abort -- --emit llvm-ir
find target -type f -name '*.ll'
# target/wasm32-unknown-unknown/release/deps/nano_web.ll

cargo clean
RUSTFLAGS="--emit=llvm-ir" wasm-pack build --release . -- -Z build-std=core,std,panic_abort -Z build-std-features=panic_immediate_abort && ls -l pkg/ && ls -lh pkg/ && wasm2wat pkg/nano_web_bg.wasm -o pkg/nano_web_bg.wat 
rm panic_unwind-42d0f035091cc3cc.ll
llvm-link target/wasm32-unknown-unknown/release/deps/*.ll
llvm-link --only-needed *.ll > out.bc
opt -O3 out.bc -o optimized.bc
llvm-dis optimized.bc

# bundler, nodejs, web, no-modules

cargo expand --target wasm32-unknown-unknown

# https://github.com/rustwasm/wasm-bindgen/blob/main/crates/web-sys/src/lib.rs

# https://github.com/rustwasm/wasm-bindgen/blob/main/src/convert/closures.rs

# the wat format is pretty good

twiggy top -n 20 pkg/nano_web_bg.wasm


cargo rustc --release -- --emit llvm-ir
find target/release -type f -name '*.ll'
```

https://github.com/rustwasm/wasm-pack/issues/737

https://rustwasm.github.io/docs/book/reference/code-size.html

https://rustwasm.github.io/docs/book/reference/tools.html