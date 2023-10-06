#![no_std]

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}
// https://github.com/rust-lang/rust/issues/103516
// https://rustwasm.github.io/wasm-bindgen/reference/reference-types.html

// https://github.com/rust-lang/rust/issues/60825

#[no_mangle]
pub static mut GLOBALL: usize = 1;

//#[link(wasm_import_module = "Math")]
//#[export_name = "add"]
//#[link_name = "..."]

/*
const _: () = {
    #[link_section = "surmsection"]
    static SECTION_CONTENT: [u8; 11] = *b"hello world";
};
*/

extern "C" {
    //pub static mut GLOBALL: usize;

    pub fn console_log(input: usize);

    pub fn peekaboo(f: extern "C" fn(u32) -> u32);
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

pub extern "C" fn identity<T>(value: T) -> T {
    value
}

#[no_mangle]
pub extern "C" fn test() {
    unsafe { peekaboo(identity) }
}

#[no_mangle]
pub extern "C" fn get_global_value_fun() -> extern "C" fn() -> usize {
    get_global_value
}