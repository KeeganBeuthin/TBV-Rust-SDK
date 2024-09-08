use serde_json::Value;

pub fn handle_http_request(request: &Value) -> Result<String, String> {
    // Log the incoming request for debugging
    log(&format!("SDK received request: {}", request));

    // Extract necessary information from the request
    let method = request["method"].as_str().ok_or("Missing 'method' in request")?;
    let path = request["path"].as_str().ok_or("Missing 'path' in request")?;
    let headers = request["headers"].as_object().ok_or("Missing 'headers' in request")?;
    let body = request.get("body").and_then(Value::as_str).unwrap_or("");

    // Process the request based on the path and method
    let response = match (method, path) {
        ("GET", "/api/data") => handle_get_data(),
        ("POST", "/api/data") => handle_post_data(body),
        ("PUT", "/api/data") => handle_put_data(body),
        ("DELETE", "/api/data") => handle_delete_data(),
        _ => Err(format!("Unsupported method or path: {} {}", method, path)),
    }?;

    // Construct the response JSON
    let response_json = serde_json::json!({
        "statusCode": response.status_code,
        "headers": {
            "Content-Type": "application/json",
            // Add any additional headers here
        },
        "body": response.body
    });

    // Log the outgoing response for debugging
    log(&format!("SDK sending response: {}", response_json));

    Ok(response_json.to_string())
}

// Helper function for logging
fn log(message: &str) {
    // Implement logging logic here (e.g., printing to console or writing to a file)
    println!("[SDK] {}", message);
}

// Helper struct for responses
struct Response {
    status_code: u16,
    body: String,
}

// Handler functions for different endpoints
fn handle_get_data() -> Result<Response, String> {
    Ok(Response {
        status_code: 200,
        body: r#"{"message": "Hello from Rust WebAssembly API!"}"#.to_string(),
    })
}

fn handle_post_data(body: &str) -> Result<Response, String> {
    // Here you can process the body if needed
    Ok(Response {
        status_code: 201,
        body: r#"{"message": "Data created successfully"}"#.to_string(),
    })
}

fn handle_put_data(body: &str) -> Result<Response, String> {
    // Here you can process the body if needed
    Ok(Response {
        status_code: 200,
        body: r#"{"message": "Data updated successfully"}"#.to_string(),
    })
}

fn handle_delete_data() -> Result<Response, String> {
    Ok(Response {
        status_code: 200,
        body: r#"{"message": "Data deleted successfully"}"#.to_string(),
    })
}