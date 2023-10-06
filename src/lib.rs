// https://github.com/rust-lang/rust/issues/103516
// https://rustwasm.github.io/wasm-bindgen/reference/reference-types.html

// https://github.com/rust-lang/rust/issues/60825

#[no_mangle]
pub static mut GLOBALL: usize = 1;

extern "C" {
    //pub static mut GLOBALL: usize;

    pub fn console_log(input: usize);
}

#[no_mangle]
pub extern "C" fn get_global_value() -> usize {
   unsafe { GLOBALL }
}

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    unsafe {
        console_log(left);
        console_log(right);
        console_log(GLOBALL);
        GLOBALL += 1;
    }
    left + right
}
