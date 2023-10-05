```bash
wasm-pack build --reference-types --profiling && ls -l pkg/ && ls -lh pkg/ && wasm-decompile --output=pkg/nano_web_b
g.dcmp pkg/nano_web_bg.wasm

twiggy top -n 20 pkg/nano_web_bg.wasm
```

https://rustwasm.github.io/docs/book/reference/code-size.html