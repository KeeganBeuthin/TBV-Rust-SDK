use serde::{Deserialize, Serialize};
use crate::ffi::string_to_ptr;
use crate::utils::log;

#[derive(Deserialize)]
struct Request {
    method: String,
    path: String,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

#[derive(Serialize)]
struct Response {
    status_code: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

#[no_mangle]
pub extern "C" fn handle_http_request(request_ptr: *const u8, request_len: usize) -> *const u8 {
    let request_str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(request_ptr, request_len)) };
    log(&format!("Received request: {}", request_str));

    let request: Request = match serde_json::from_str(request_str) {
        Ok(v) => v,
        Err(e) => {
            let error_msg = format!("{{\"error\": \"Invalid JSON: {}\"}}", e);
            return string_to_ptr(&error_msg);
        }
    };

    let response = handle_request(request);
    let response_json = serde_json::to_string(&response).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e));

    log(&format!("Sending response: {}", response_json));
    string_to_ptr(&response_json)
}

fn handle_request(req: Request) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/api/data") => handle_data_request(req),
        ("POST", "/api/data") => handle_data_request(req),
        ("PUT", "/api/data") => handle_data_request(req),
        ("DELETE", "/api/data") => handle_data_request(req),
        _ => Response {
            status_code: 404,
            headers: [("Content-Type".to_string(), "text/plain".to_string())].into(),
            body: "Not Found".to_string(),
        },
    }
}

fn handle_data_request(req: Request) -> Response {
    match req.method.as_str() {
        "GET" => Response {
            status_code: 200,
            headers: [("Content-Type".to_string(), "application/json".to_string())].into(),
            body: r#"{"message": "Hello from Rust WebAssembly API!"}"#.to_string(),
        },
        "POST" => Response {
            status_code: 201,
            headers: [("Content-Type".to_string(), "application/json".to_string())].into(),
            body: r#"{"message": "Data created successfully"}"#.to_string(),
        },
        "PUT" => Response {
            status_code: 200,
            headers: [("Content-Type".to_string(), "application/json".to_string())].into(),
            body: r#"{"message": "Data updated successfully"}"#.to_string(),
        },
        "DELETE" => Response {
            status_code: 200,
            headers: [("Content-Type".to_string(), "application/json".to_string())].into(),
            body: r#"{"message": "Data deleted successfully"}"#.to_string(),
        },
        _ => Response {
            status_code: 405,
            headers: [("Content-Type".to_string(), "text/plain".to_string())].into(),
            body: "Method Not Allowed".to_string(),
        },
    }
}