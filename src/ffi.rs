use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, len);
    }
}

pub fn string_to_ptr(s: &str) -> *const u8 {
    let c_str = CString::new(s).unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    ptr as *const u8
}

pub fn free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}

extern "C" {
    pub fn log_message(ptr: *const u8, len: i32);
}