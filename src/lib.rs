use crate::interpreter::{interpret::eval, lexeme::get_tokens, parsing::Parser};
use std::ptr;
use std::{ffi::CStr, os::raw::c_char};

mod interpreter;
mod number;

#[no_mangle]
pub extern "C" fn calculate(cmd_ptr: *const c_char) -> *mut u8 {
    let cmd = unsafe { CStr::from_ptr(cmd_ptr) }.to_str().unwrap();
    let tokens = get_tokens(cmd);
    let mut parser = Parser::new(tokens);

    let result = eval(parser.parse()).to_string();
    println!("Result in Rust: {}", result);

    let allocated = unsafe { libc::malloc(result.len() + 1) } as *mut u8;

    if allocated.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        allocated.copy_from_nonoverlapping(result.as_bytes().as_ptr(), result.len());
    }
    allocated
}
