import wasmUrl from '../../target/wasm32-unknown-unknown/release/nano_web.wasm?url'

let module = await WebAssembly.compileStreaming(fetch(wasmUrl))
let instance = await WebAssembly.instantiate(module, {
  env: {
    console_log(arg: number) {
      console.log(arg)
    },
    consoleLogString(offset: number, length: number) {
      const bytes = new Uint8Array(instance.exports.memory as unknown as ArrayBufferLike, offset, length);
      const string = new TextDecoder("utf8").decode(bytes);
      console.log(string);
    },
  },
})

console.log((instance.exports as any).add(1, 2));
console.log((instance.exports as any).get_global_value())