import wasmUrl from '../../target/wasm32-unknown-unknown/release/nano_web.wasm?url'

// https://rustwasm.github.io/docs/wasm-bindgen/reference/reference-types.html

let module = await WebAssembly.compileStreaming(fetch(wasmUrl))
let exports = (await WebAssembly.instantiate(module, {
  env: {
    console_log(arg: number) {
      console.log(arg)
    },
    peekaboo(arg: number) {
      let table = exports.__indirect_function_table
      let f = table.get(arg)
      let result = f(1)
      console.log("result", result)
    },
    console_log_string(offset: number, length: number) {
      const bytes = new Uint8Array(exports.memory.buffer, offset, length);
      const string = new TextDecoder("utf8").decode(bytes);
      console.log(string);
    },
    query_selector(offset: number, length: number): number {
      const bytes = new Uint8Array(exports.memory.buffer, offset, length);
      const string = new TextDecoder("utf8").decode(bytes);
      let element = document.querySelector(string);
      let index = exports.externrefs.grow(1, element)
      return index
    },
    set_text_content(elementOffset: number, offset: number, length: number) {
      const bytes = new Uint8Array(exports.memory.buffer, offset, length);
      const string = new TextDecoder("utf8").decode(bytes);
      let element = exports.externrefs.get(elementOffset) as Element;
      element.textContent = string
    }
  },
})).exports as {
  memory: WebAssembly.Memory,
  __indirect_function_table: WebAssembly.Table,
  externrefs: WebAssembly.Table,
  GLOBALL: WebAssembly.Global,
  test: () => void,
  add: (a: number, b: number) => number,
};

exports.test();
let global = exports.GLOBALL
console.log("memory offset", global.value)
let global_mem = new Uint32Array(exports.memory.buffer, global.value);
console.log("value: ", global_mem.at(0));
exports.add(1, 1)
console.log("value: ", global_mem.at(0));
