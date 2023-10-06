import './style.css'
import typescriptLogo from './typescript.svg'
import viteLogo from '/vite.svg'
import { setupCounter } from './counter.ts'
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

console.log((instance.exports as any).add(1, 2)); // "3"
console.log((instance.exports as any).get_global_value())

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
