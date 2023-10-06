import wasmUrl from '../../target/wasm32-unknown-unknown/release/nano_web.wasm?url'

let module = await WebAssembly.compileStreaming(fetch(wasmUrl))
let instance = await WebAssembly.instantiate(module, {
  env: {
    console_log(arg: number) {
      console.log(arg)
    },
    peekaboo(arg: number) {
      let table = instance.exports.__indirect_function_table as WebAssembly.Table
      let f = table.get(arg)
      let result = f(1)
      console.log("result", result)
    },
    consoleLogString(offset: number, length: number) {
      const bytes = new Uint8Array(instance.exports.memory as unknown as ArrayBufferLike, offset, length);
      const string = new TextDecoder("utf8").decode(bytes);
      console.log(string);
    },
  },
})

console.log((instance.exports as any).test())
let global = instance.exports.GLOBALL as WebAssembly.Global
console.log(global.value)