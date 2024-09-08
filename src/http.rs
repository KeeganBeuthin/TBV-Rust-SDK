use serde_json::{json, Value};

pub fn handle_http_request(request_json: &str) -> String {
    let request: Value = serde_json::from_str(request_json).unwrap_or(json!({}));
    let path = request["path"].as_str().unwrap_or("");
    let method = request["method"].as_str().unwrap_or("");

    match path {
        "/api/data" => handle_data_request(method),
        _ => json!({
            "statusCode": 404,
            "headers": {"Content-Type": "text/plain"},
            "body": "Not Found"
        }).to_string(),
    }
}

fn handle_data_request(method: &str) -> String {
    let response = match method {
        "GET" => json!({
            "statusCode": 200,
            "headers": {"Content-Type": "application/json"},
            "body": json!({"message": "Hello from WebAssembly API!"})
        }),
        "POST" => json!({
            "statusCode": 201,
            "headers": {"Content-Type": "application/json"},
            "body": json!({"message": "Data created successfully"})
        }),
        "PUT" => json!({
            "statusCode": 200,
            "headers": {"Content-Type": "application/json"},
            "body": json!({"message": "Data updated successfully"})
        }),
        "DELETE" => json!({
            "statusCode": 200,
            "headers": {"Content-Type": "application/json"},
            "body": json!({"message": "Data deleted successfully"})
        }),
        _ => json!({
            "statusCode": 405,
            "headers": {"Content-Type": "text/plain"},
            "body": "Method Not Allowed"
        }),
    };

    response.to_string()
}