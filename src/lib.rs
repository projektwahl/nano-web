extern crate alloc;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Window;

// wee_alloc_WeeAlloc_as_core_alloc_global_GlobalAlloc_alloc_hb4dfa88b790f3b3d

// https://github.com/rustwasm/wasm-bindgen/blob/5d0f935d658e10cdb4ae825394a9b25b4e9e6369/crates/js-sys/src/lib.rs#L5947

#[wasm_bindgen]
extern "C" {
    type Global;

    #[wasm_bindgen(getter, static_method_of = Global, js_class = window, js_name = window)]
    fn get_window() -> Window;
}


#[wasm_bindgen(start)]
fn run() {
    //std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //console_log::init_with_level(Level::Debug).unwrap();

    let window = Global::get_window();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let val = document.create_element("p").unwrap();
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val).unwrap();
}