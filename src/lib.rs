#![no_std]

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}
// https://github.com/rust-lang/rust/issues/103516
// https://rustwasm.github.io/wasm-bindgen/reference/reference-types.html

// https://github.com/rust-lang/rust/issues/60825

#[no_mangle]
pub static mut GLOBALL: isize = 0;

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
    //pub static GLOBAL2: usize;

    pub fn console_log(input: isize);

    pub fn console_log_string(ptr: *const u8, length: usize);

    pub fn peekaboo(f: extern "C" fn(isize) -> isize);

    pub fn query_selector(ptr: *const u8, length: usize);
}

#[no_mangle]
pub extern "C" fn get_global_value() -> isize {
   unsafe { GLOBALL }
}

#[no_mangle]
pub extern "C" fn add(left: isize, right: isize) -> isize {
    unsafe {
        console_log(left);
        console_log(right);
        console_log(GLOBALL);
        GLOBALL += 1;
    }
    left + right
}

pub extern "C" fn negate<T: core::ops::Neg<Output = T>>(value: T) -> T {
    -value
}

#[no_mangle]
pub extern "C" fn test() {
    unsafe { console_log_string("hello".as_ptr(), 5) };
    unsafe { query_selector("#counter".as_ptr(), 8) };
    unsafe { peekaboo(negate) }
}

#[no_mangle]
pub extern "C" fn get_global_value_fun() -> extern "C" fn() -> isize {
    get_global_value
}

#[no_mangle]
pub extern "C" fn peekaboo2(f: extern "C" fn(isize) -> isize) -> isize {
    f(1)
}