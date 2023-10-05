// for now leave this out as with no allocator needed this still generates code
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// wee_alloc_WeeAlloc_as_core_alloc_global_GlobalAlloc_alloc_hb4dfa88b790f3b3d

#[wasm_bindgen(start)]
fn run() {
    //std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //console_log::init_with_level(Level::Debug).unwrap();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().unwrap();
    //let document = window.document().expect("should have a document on window");
    //let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    //let val = document.create_element("p")?;
    //val.set_text_content(Some("Hello from Rust!"));

    //body.append_child(&val)?;

    
}
