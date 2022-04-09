use std::{ffi::CString, os::raw::c_char};

// STRING INTEROP ----------
// Thanks to https://dev.to/living_syn/calling-rust-from-c-6hk for string interop source!
static mut STRING_POINTER: *mut c_char = 0 as *mut c_char;

#[repr(C)]
pub struct InteropableString {
    pub value: *mut c_char,
}

fn store_string_on_heap(string_to_store: &'static str) -> *mut c_char {
    //create a new raw pointer
    let pntr = CString::new(string_to_store).unwrap().into_raw();
    //store it in our static variable (REQUIRES UNSAFE)
    unsafe {
        STRING_POINTER = pntr;
    }
    //return the c_char
    return pntr;
}

#[no_mangle]
pub extern "C" fn free_string() {
    unsafe {
        let _ = CString::from_raw(STRING_POINTER);
        STRING_POINTER = 0 as *mut c_char;
    }
}

// ACTUAL SAVE STATE METHODS ----------
// #[no_mangle] is needed so the name is exported as human readable instead of garbage
#[no_mangle]
pub extern "C" fn test_method() -> InteropableString {
    let return_value: &'static str = "Hello from the unmanaged world!";

    InteropableString {
        value: store_string_on_heap(return_value),
    }
}
