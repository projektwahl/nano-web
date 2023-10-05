#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    //std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //console_log::init_with_level(Level::Debug).unwrap();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    Ok(())
}
