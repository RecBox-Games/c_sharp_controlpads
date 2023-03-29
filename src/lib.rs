#![feature(vec_into_raw_parts)]
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

use controlpads;


type CError = u64;

// TODO: more errors
pub const SUCCESS: CError = 0;
pub const ERROR_CONTROLPADS: CError = 1;
pub const ERROR_CSTR_TO_STR: CError = 2;

#[derive(Debug)]
#[repr(C)]
pub struct c_flat_string_vec {
	chars_ptr: *mut c_char,
	chars_len: u64,
	chars_cap: u64,
	lens_ptr: *mut u64,
	lens_len: u64,
	lens_cap: u64,
}

pub fn string_vec_to_c_flat_string_vec(vec: Vec<String>) -> c_flat_string_vec {
    // lens vec
    let mut lens_vec: Vec<u64> = Vec::new();    
    let mut num_chars = 0;
    for s in &vec {
        let s_len = s.len();
        lens_vec.push(s_len as u64);
        num_chars += s_len;
    }
    // chars vec
    let mut chars_vec: Vec<c_char> = Vec::with_capacity(num_chars + 1);
    let mut index: usize = 0;
    unsafe {
        let chars_vec_ptr = chars_vec.as_mut_ptr();
        for s in vec {
            let c_string = CString::new(s).unwrap();
            let c_string_len = c_string.to_bytes().len();
            let c_string_ptr = c_string.as_ptr();
            std::ptr::copy(c_string_ptr, chars_vec_ptr.offset(index as isize), c_string_len);
            chars_vec.set_len(chars_vec.len()+c_string_len);
            index += c_string_len;
        }
    }
    //
    let (chars_ptr, chars_len, chars_cap) = chars_vec.into_raw_parts();
    let (lens_ptr, lens_len, lens_cap) = lens_vec.into_raw_parts();
    c_flat_string_vec {
        chars_ptr,
        chars_len: chars_len as u64,
        chars_cap: chars_cap as u64,
        lens_ptr,
        lens_len: lens_len as u64,
        lens_cap: lens_cap  as u64,
    }
}

impl c_flat_string_vec {
    pub fn as_vecs(self) -> (Vec<c_char>, Vec<u64>) {
        unsafe {
            let chars_vec = Vec::from_raw_parts(self.chars_ptr,
                                                self.chars_len as usize,
                                                self.chars_cap as usize);
            let lens_vec = Vec::from_raw_parts(self.lens_ptr,
                                               self.lens_len as usize,
                                               self.lens_cap as usize);
            (chars_vec, lens_vec)
        }
     }
}


#[no_mangle]
pub extern "C" fn free_c_flat_string_vec(flat_vec: c_flat_string_vec) {
        // we take back ownership of the vec memory so that when we leave
        // this scope that memory is freed
        let (_,_) = flat_vec.as_vecs();
        /*let _chars_vec = Vec::from_raw_parts(flat_vec.chars_ptr,
                                            flat_vec.chars_len as usize,
                                            flat_vec.chars_cap as usize);
        let _lens_vec = Vec::from_raw_parts(flat_vec.lens_ptr,
                                            flat_vec.lens_len as usize,
                                            flat_vec.lens_cap as usize);
         */
}

#[no_mangle]
pub extern "C" fn clients_changed(did_change: &mut bool) -> CError {
    let result = controlpads::clients_changed();
    match result {
        Ok(x) => {
            *did_change = x;
            SUCCESS
        }
        Err(e) => { // TODO: use error (print it to stderr perhaps)
            println!("clients_changed() Error: {}", e);
            ERROR_CONTROLPADS
        }
    }

}

#[no_mangle]
pub extern "C" fn get_client_handles(client_handles: *mut c_flat_string_vec) -> CError {
    let result = controlpads::get_client_handles();
    match result {
        Ok(x) => {
            unsafe {
                *client_handles = string_vec_to_c_flat_string_vec(x);
            }
            SUCCESS
        }
        Err(e) => { // TODO: use error
            println!("get_client_handles() Error: {}", e);
            ERROR_CONTROLPADS
        }
    }
}

#[no_mangle]
pub extern "C" fn send_message(client: *const c_char, msg: *const c_char) -> CError {
    // TODO: We're copying data to make the String and eventually we should
    //       *not* do that
    // TODO: print along with errors
    unsafe {
        let client_str = match CStr::from_ptr(client).to_str() {
            Ok(ok) => ok,
            Err(e) => {
                println!("send_message() client_str Error: {}", e);
                return ERROR_CSTR_TO_STR;
            }
        };
        let msg_str = match CStr::from_ptr(msg).to_str() {
            Ok(ok) => ok,
            Err(e) => {
                println!("send_message() msg_str Error: {}", e);
                return ERROR_CSTR_TO_STR;
            }
        };

        match controlpads::send_message(&String::from(client_str), msg_str) {
            Ok(()) => {
                SUCCESS
            }
            Err(e) => {
                println!("send_message() Error: {}", e);
                ERROR_CONTROLPADS
            }
            
        }
    }
}

#[no_mangle]
pub extern "C" fn get_messages(client: *const c_char, messages: *mut c_flat_string_vec) -> CError {
    unsafe {
        let client_str = match CStr::from_ptr(client).to_str() {
            Ok(ok) => ok,
            Err(e) => {
                println!("get_messages() client_str Error: {}", e);
                return ERROR_CSTR_TO_STR;
            }
        };
        let result = controlpads::get_messages(&String::from(client_str));
        match result {
            Ok(x) => {
                *messages = string_vec_to_c_flat_string_vec(x);
                SUCCESS
            }
            Err(e) => {
                println!("get_messages() Error: {}", e);
                ERROR_CONTROLPADS
            }
        }
    }
}
