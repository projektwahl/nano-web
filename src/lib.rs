// https://github.com/rust-lang/rust/issues/103516
// https://rustwasm.github.io/wasm-bindgen/reference/reference-types.html

extern "C" {
    pub fn console_log(input: usize);
}

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    unsafe {
        console_log(left);
        console_log(right);
    }
    left + right
}
