use crate::ffi::log_message;

pub fn log(message: &str) {
    let bytes = message.as_bytes();
    unsafe {
        log_message(bytes.as_ptr(), bytes.len() as i32);
    }
}

pub fn read_html_code(html_code: &str) -> Result<String, String> {
    if html_code.is_empty() {
        Err("Error: empty HTML code provided".to_string())
    } else {
        Ok(html_code.to_string())
    }
}