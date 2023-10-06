// for now leave this out as with no allocator needed this still generates code
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use js_sys::Object;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::Window;
use wasm_bindgen::JsCast;

// wee_alloc_WeeAlloc_as_core_alloc_global_GlobalAlloc_alloc_hb4dfa88b790f3b3d

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

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.

    // the comment in the source code even says this is large
    //let window = Global::get_window().unwrap().dyn_into::<Window>().ok();
    //let window = web_sys::window().unwrap();
    //let document = window.document().expect("should have a document on window");
    //let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    //let val = document.create_element("p")?;
    //val.set_text_content(Some("Hello from Rust!"));

    //body.append_child(&val)?;

    
}
