use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[wasm_bindgen]
pub fn dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}

pub fn string_to_ptr(s: &str) -> *mut u8 {
    let len = s.len();
    let ptr = unsafe { alloc(len + 1) };
    unsafe {
        std::ptr::copy_nonoverlapping(s.as_ptr(), ptr, len);
        *ptr.add(len) = 0;
    }
    ptr
}

pub fn ptr_to_string(ptr: *const u8) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *ptr.add(i) != 0).count();
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf8_lossy(slice).into_owned()
    }
}