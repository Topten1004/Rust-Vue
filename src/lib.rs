extern crate libc;

use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub fn substract(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

#[no_mangle]
pub fn rest(a: i32, b: i32) -> i32 {
    a % b
}

#[no_mangle]
pub fn power(a: i32, b: i32) -> i32 {
    a ^ b
}

#[no_mangle]
pub fn get_key() -> *mut c_char {
    let key = String::from("1937987YE");
    let c_str = CString::new(key).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub fn free_key(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
