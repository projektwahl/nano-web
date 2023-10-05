```bash
wasm-pack build --reference-types --profiling && ls -lh pkg/ && wasm-decompile --output=pkg/nano_web_bg.dcmp pkg/nano_web_bg.wasm
```