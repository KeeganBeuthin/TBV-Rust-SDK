use wasm_bindgen::prelude::*;

pub fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}

pub fn read_html_code(html_code: &str) -> Result<String, String> {
    if html_code.is_empty() {
        return Err("Error: empty HTML code provided".to_string());
    }
    // Here you can add any processing logic if needed
    Ok(html_code.to_string())
}
