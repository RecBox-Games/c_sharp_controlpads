#![feature(vec_into_raw_parts)]
use c_sharp_controlpads::*;

use std::ffi::{c_char, CString};

#[test]
fn test_string_vec_to_c_flat_string_vec() {
    let a = String::from("Hello World!");
    let b = String::from("and");
    let c = String::from("Goodbye!");
    let string_vec = vec![a,b,c];
    let c_flat = string_vec_to_c_flat_string_vec(string_vec);
    let (chars, lens) = c_flat.as_vecs();
    let s = CString::new("Hello World!andGoodbye!").unwrap();
    let s_ptr = s.as_ptr();
    let s_len = s.to_bytes().len();
    let expected_chars: Vec<c_char> = unsafe { Vec::from_raw_parts(s_ptr as *mut c_char, s_len, s_len) };
    let expected_lens: Vec<u64> = vec![12, 3, 8];
    assert_eq!(chars, expected_chars);
    assert_eq!(lens, expected_lens);
    // avoid double free
    let (_,_,_) = expected_chars.into_raw_parts();
}
