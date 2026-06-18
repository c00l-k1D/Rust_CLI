use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::processor::process_data;

/// Process data via FFI (C interface)
/// 
/// # Safety
/// This function is unsafe because it deals with raw pointers from C code.
/// The caller must ensure that:
/// - `input` is a valid null-terminated C string
/// - The returned pointer is freed using `free_string` when done
#[no_mangle]
pub unsafe extern "C" fn process_ffi(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        let err = CString::new("Null pointer").unwrap();
        return err.into_raw();
    }

    match CStr::from_ptr(input).to_str() {
        Ok(input_str) => {
            match process_data(input_str) {
                Ok(result) => {
                    match CString::new(result) {
                        Ok(c_result) => c_result.into_raw(),
                        Err(_) => {
                            let err = CString::new("Failed to convert result to C string").unwrap();
                            err.into_raw()
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("Error: {}", e);
                    match CString::new(error_msg) {
                        Ok(c_err) => c_err.into_raw(),
                        Err(_) => {
                            let err = CString::new("Unknown error").unwrap();
                            err.into_raw()
                        }
                    }
                }
            }
        }
        Err(_) => {
            let err = CString::new("Invalid UTF-8 string").unwrap();
            err.into_raw()
        }
    }
}

/// Free memory allocated by FFI functions
/// 
/// # Safety
/// This function is unsafe because it deals with raw pointers.
/// The caller must ensure that the pointer was allocated by a Rust FFI function.
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}
