use reqwest::blocking::Client;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    q: String,
}

fn strip_bearer_prefix(token: &str) -> &str {
    if token.starts_with("Bearer ") {
        &token[7..] // Remove "Bearer " (7 characters)
    } else {
        token
    }
}

fn is_jwt_token(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    // Does this look like it's probably base64?
    parts.iter().all(|part| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    })
}

fn add_headers(
    request_builder: reqwest::blocking::RequestBuilder,
    token: &str,
    verbose: bool,
) -> reqwest::blocking::RequestBuilder {
    let clean_token = strip_bearer_prefix(token);
    let mut request_headers = header::HeaderMap::new();

    request_headers.insert(header::USER_AGENT, header::HeaderValue::from_static("curl"));
    request_headers.insert(
        header::HeaderName::from_static("x-queryai-fuql"),
        header::HeaderValue::from_static("v2"),
    );
    request_headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    if is_jwt_token(clean_token) {
        if verbose {
            println!("🔍 Detected JWT token - using Authorization header");
        }
        request_headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(clean_token).expect("Header value is invalid"),
        );
    } else {
        if verbose {
            println!("🔍 Detected API key - using x-token-authorization header");
        }
        request_headers.insert(
            header::HeaderName::from_static("x-token-authorization"),
            header::HeaderValue::from_str(clean_token).expect("Header value is invalid"),
        );
    }

    request_builder.headers(request_headers)
}

pub fn dispatch_query(
    query: &str,
    api: &str,
    token: &str,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let data = PostData {
        q: query.to_string(),
    };

    if verbose {
        println!("🚀 Dispatching query to: {}", api);
        println!(
            "📦 Payload: {}",
            serde_json::to_string_pretty(&data)
                .unwrap_or_else(|_| "Failed to serialize".to_string())
        );
    }

    let client = Client::new();

    // Build request with appropriate auth header based on token type
    let request = add_headers(client.post(api), token, verbose).json(&data);

    if verbose {
        println!("🌐 Making POST request...");
    }

    // Send the request
    let response = match request.send() {
        Ok(response) => {
            if verbose {
                println!("✅ Request sent successfully!");
                println!("📊 Status Code: {}", response.status());
                println!(
                    "🏷️  Status Text: {}",
                    response.status().canonical_reason().unwrap_or("Unknown")
                );

                // Log response headers
                println!("📋 Response Headers:");
                for (key, value) in response.headers() {
                    println!("   {}: {}", key, value.to_str().unwrap_or("<non-utf8>"));
                }

                // Special handling for 401/403 responses
                if response.status() == 401 {
                    println!("🚨 HTTP 401 Unauthorized - Check your token!");
                } else if response.status() == 403 {
                    println!("🚨 HTTP 403 Forbidden - Token valid but insufficient permissions!");
                }
            }

            response
        }
        Err(e) => {
            if verbose {
                println!("❌ Request failed to send!");
                println!("🔍 Error type: {}", std::any::type_name_of_val(&e));

                // Provide more specific error information
                if e.is_timeout() {
                    println!("⏰ Error details: Request timed out");
                } else if e.is_connect() {
                    println!(
                        "🔌 Error details: Connection failed - check if the server is running and the URL is correct"
                    );
                } else if e.is_request() {
                    println!(
                        "📤 Error details: Request construction failed - check your URL and parameters"
                    );
                } else {
                    println!("🔍 Error details: {}", e);
                }
            }

            return Err(Box::new(e));
        }
    };

    // Check if the response status indicates success
    let status = response.status();
    if !status.is_success() {
        if verbose {
            println!("⚠️  Non-success status code: {}", status);

            // Provide specific debugging for auth issues
            if status == 401 {
                println!("🔍 Authentication Debugging:");
                println!("  • Verify your token is correct and not expired");
                println!("  • Check if the API endpoint expects the correct auth method");
            } else if status == 403 {
                println!("🔍 Authorization Debugging:");
                println!("  • Token is valid but may lack required permissions");
                println!("  • Check if your token has access to this specific endpoint");
            }
        }

        // Try to read error response body
        match response.text() {
            Ok(error_body) => {
                if verbose {
                    println!("📄 Error response body:");
                    println!("{}", error_body);

                    // Look for common auth error patterns
                    if error_body.to_lowercase().contains("unauthorized") {
                        println!("💡 Server says 'unauthorized' - likely a token issue");
                    } else if error_body.to_lowercase().contains("invalid")
                        && error_body.to_lowercase().contains("token")
                    {
                        println!("💡 Server says invalid token - check token format/expiration");
                    }
                }
                return Err(format!("Server returned error {}: {}", status, error_body).into());
            }
            Err(body_err) => {
                if verbose {
                    println!("❌ Could not read error response body: {}", body_err);
                }
                return Err(format!(
                    "Server returned error {} (could not read response body)",
                    status
                )
                .into());
            }
        }
    }

    // Try to read the response body
    if verbose {
        println!("📖 Reading response body...");
    }

    let response_text = response.text().unwrap();

    let _data = match serde_json::from_str::<serde_json::Value>(&response_text) {
        Ok(_data) => {
            println!("{}", serde_json::to_string_pretty(&_data).unwrap());
            _data
        }
        Err(e) => {
            if verbose {
                println!("❌ Failed to read response body: {}", e);
            }
            return Err(Box::new(e));
        }
    };
    Ok(response_text)
}
