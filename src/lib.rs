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
    match panic::catch_unwind(|| {  // head of `match`
        let token_stream = get_tokens(input);
        let mut parser = Parser::new(token_stream.clone());
        match mode {
            Mode::Calculate => eval(parser.parse()).to_string(),
            Mode::Lexeme => show_tokens(&token_stream),
            Mode::Tree => format!("{:#?}", parser.parse()),
        }
    })
    {  // body of `match`
        Ok(s) => s,
        Err(_e) => {
            format!("@error: {:#?}", _e)  // todo: more user-friendly
        }
    }
}

#[no_mangle]
pub extern "C" fn calculate(input_ptr: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input_ptr) }.to_str().unwrap();
    let result = exec(input, Mode::Calculate);
    let result_cstr = CString::new(result).unwrap();
    result_cstr.into_raw()
}

#[no_mangle]
pub extern "C" fn lexeme(input_ptr: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input_ptr) }.to_str().unwrap();
    let result = exec(input, Mode::Lexeme);
    let result_cstr = CString::new(result).unwrap();
    result_cstr.into_raw()
}

#[no_mangle]
pub extern "C" fn tree(input_ptr: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input_ptr) }.to_str().unwrap();
    let result = exec(input, Mode::Tree);
    let result_cstr = CString::new(result).unwrap();
    result_cstr.into_raw()
}