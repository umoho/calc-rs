use std::{ffi::{CString, CStr}, os::raw::c_char, panic};

use crate::interpreter::{
    lexeme::{get_tokens, show_tokens},
    interpret::eval,
    parsing::Parser
};

mod number;
mod interpreter;

enum Mode { Calculate, Lexeme, Tree }

fn exec(input: &str, mode: Mode) -> String {
    let result = panic::catch_unwind(|| {
        let token_stream = get_tokens(input);
        let mut parser = Parser::new(token_stream.clone());
        match mode {
            Mode::Calculate => eval(parser.parse()).to_string(),
            Mode::Lexeme => show_tokens(&token_stream),
            Mode::Tree => format!("{:#?}", parser.parse()),
        }
    });
    match result {
        Ok(s) => s,
        Err(_) => {
            format!("@error")  // todo: more user-friendly
        }
    }
}

// ---- C FFI ----

#[no_mangle]
#[cfg(feature="ffi")]
pub extern "C" fn calc_calculate(input_ptr: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input_ptr) }.to_str().unwrap();
    let result = exec(input, Mode::Calculate);
    let result_cstr = CString::new(result).unwrap();
    result_cstr.into_raw()
}

#[no_mangle]
#[cfg(feature="ffi")]
pub extern "C" fn calc_lexeme(input_ptr: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input_ptr) }.to_str().unwrap();
    let result = exec(input, Mode::Lexeme);
    let result_cstr = CString::new(result).unwrap();
    result_cstr.into_raw()
}

#[no_mangle]
#[cfg(feature="ffi")]
pub extern "C" fn calc_tree(input_ptr: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input_ptr) }.to_str().unwrap();
    let result = exec(input, Mode::Tree);
    let result_cstr = CString::new(result).unwrap();
    result_cstr.into_raw()
}

#[no_mangle]
#[cfg(feature="ffi")]
pub unsafe extern "C" fn calc_free(ptr: *mut c_char) {
    drop(CString::from_raw(ptr));
}

// ---- Rust lib ----

pub fn calculate(input: String) -> String {
    exec(&input, Mode::Calculate)
}

pub fn lexeme(input: String) -> String {
    exec(&input, Mode::Lexeme)
}

pub fn tree(input: String) -> String {
    exec(&input, Mode::Tree)
}