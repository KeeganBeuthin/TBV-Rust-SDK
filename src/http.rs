use serde_json::Value;
use serde::Serialize;
use crate::ffi::string_to_ptr;
use crate::utils::log;

#[no_mangle]
pub extern "C" fn handle_http_request(request_ptr: *const u8, request_len: usize) -> *const u8 {
    let request_str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(request_ptr, request_len)) };
    log(&format!("Received request: {}", request_str));

    let request: Value = match serde_json::from_str(request_str) {
        Ok(v) => v,
        Err(e) => {
            let error_msg = format!("{{\"error\": \"Invalid JSON: {}\"}}", e);
            return string_to_ptr(&error_msg) as *const u8;
        }
    };

    let response = match process_request(&request) {
        Ok(resp) => resp,
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    };

    log(&format!("Sending response: {}", response));
    string_to_ptr(&response) as *const u8
}

fn process_request(request: &Value) -> Result<String, String> {
    let method = request["method"].as_str().ok_or("Missing 'method' in request")?;
    let path = request["path"].as_str().ok_or("Missing 'path' in request")?;

    let response = match (method, path) {
        ("GET", "/api/data") => handle_get_data(),
        ("POST", "/api/data") => handle_post_data(request),
        ("PUT", "/api/data") => handle_put_data(request),
        ("DELETE", "/api/data") => handle_delete_data(),
        _ => Err(format!("Unsupported method or path: {} {}", method, path)),
    }?;

    Ok(serde_json::to_string(&response).map_err(|e| e.to_string())?)
}

#[derive(Serialize)]
struct Response {
    status_code: u16,
    headers: Value,
    body: String,
}

fn handle_get_data() -> Result<Response, String> {
    Ok(Response {
        status_code: 200,
        headers: serde_json::json!({"Content-Type": "application/json"}),
        body: r#"{"message": "Hello from Rust WebAssembly API!"}"#.to_string(),
    })
}

fn handle_post_data(request: &Value) -> Result<Response, String> {
    log(&format!("Handling POST request with body: {:?}", request["body"]));
    Ok(Response {
        status_code: 201,
        headers: serde_json::json!({"Content-Type": "application/json"}),
        body: r#"{"message": "Data created successfully"}"#.to_string(),
    })
}

fn handle_put_data(request: &Value) -> Result<Response, String> {
    log(&format!("Handling PUT request with body: {:?}", request["body"]));
    Ok(Response {
        status_code: 200,
        headers: serde_json::json!({"Content-Type": "application/json"}),
        body: r#"{"message": "Data updated successfully"}"#.to_string(),
    })
}

fn handle_delete_data() -> Result<Response, String> {
    Ok(Response {
        status_code: 200,
        headers: serde_json::json!({"Content-Type": "application/json"}),
        body: r#"{"message": "Data deleted successfully"}"#.to_string(),
    })
}