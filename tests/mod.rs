// Copyright 2022-2024 RecBox, Inc.
//
// This file is part of the c_sharp_controlpads repository.
//
// c_sharp_controlpads is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by the 
// Free Software Foundation, either version 3 of the License, or (at your option)
// any later version.
// 
// c_sharp_controlpads is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
// more details.
// 
// You should have received a copy of the GNU General Public License along with
// c_sharp_controlpads. If not, see <https://www.gnu.org/licenses/>.

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
