use crate::interpreter::{interpret::eval, lexeme::get_tokens, parsing::Parser};

use std::{ffi::CStr, os::raw::c_char};
use std::ffi::CString;

mod interpreter;
mod number;

#[no_mangle]
pub extern "C" fn calculate(cmd_ptr: *const c_char) -> *mut c_char {
    let cmd = unsafe { CStr::from_ptr(cmd_ptr) }.to_str().unwrap();
    let tokens = get_tokens(cmd);
    let mut parser = Parser::new(tokens);

    let result = eval(parser.parse()).to_string();
    println!("Result in Rust: {}", result);

    let result_cstr = CString::new(result).unwrap();
    result_cstr.into_raw()
}

#[no_mangle]
pub unsafe extern "C"  fn free_result(ptr: *mut c_char) {
    drop(CString::from_raw(ptr))
}
