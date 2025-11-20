use reqwest::blocking::ClientBuilder;
// use reqwest::blocking::Client;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    q: String,
}

/// JSON Response details for the FSQL EXPLAIN CONNECTORS;
#[derive(Serialize, Deserialize)]
pub struct ExplainConnectorsResponse {
    pub command: String,
    pub connectors: Vec<serde_json::Value>,
}

/// JSON Response details for the FSQL EXPLAIN CONNECTORS;
#[derive(Serialize, Deserialize)]
pub struct ExplainVersionResponse {
    pub command: String,
    pub fsql: String,
    pub qdm: String,
}

/// JSON Response details for the FSQL EXPLAIN ATTRIBUTES;
#[derive(Serialize, Deserialize)]
pub struct ExplainAttributesResponse {
    pub command: String,
    pub attributes: Vec<String>,
}

/// JSON Response details for the FSQL EXPLAIN command
#[derive(Serialize, Deserialize)]
pub struct ExplainResponse {
    pub command: String,
    pub input: String,
    pub expanded_query: serde_json::Value,
}

/// JSON Response details for the FSQL VALIDATE command
#[derive(Serialize, Deserialize)]
pub struct ValidateResponse {
    pub command: String,
    pub is_valid: bool,
}

/// JSON Response details for the FSQL QUERY command
#[derive(Serialize, Deserialize)]
pub struct QueryResponse {
    pub command: String,
    pub search_id: String,
    pub trace_id: String,
    pub results: Vec<serde_json::Value>,
}

/// Remove the Bearer prefix (if present) from a given token string
fn strip_bearer_prefix(token: &str) -> &str {
    if token.starts_with("Bearer ") {
        &token[7..] // Remove "Bearer " (7 characters)
    } else {
        token
    }
}

/// Attempt to whether the given string is a JWT or API token
fn is_jwt_token(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    // Does this look like it's probably base64 (crude check)?
    parts.iter().all(|part| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    })
}

/// Add necessary headers to the request
///
/// The FSQL API has some header requirements - it needs to provide the
/// x-queryai-fuql version, Content-Type headers, and an auth header. We
/// also set the user-agent string so that we can detect and track which
/// requests come from this tool
fn add_headers(
    request_builder: reqwest::blocking::RequestBuilder,
    token: &str,
    verbose: bool,
) -> reqwest::blocking::RequestBuilder {
    let clean_token = strip_bearer_prefix(token);
    let mut request_headers = header::HeaderMap::new();

    request_headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("fsqlctl"),
    );
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
            eprintln!("🔍 Detected JWT token - using Authorization header");
        }
        let bearer_token = format!("Bearer {}", clean_token);
        request_headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&bearer_token).expect("Header value is invalid"),
        );
    } else {
        if verbose {
            eprintln!("🔍 Detected API key - using x-token-authorization header");
        }
        request_headers.insert(
            header::HeaderName::from_static("x-token-authorization"),
            header::HeaderValue::from_str(clean_token).expect("Header value is invalid"),
        );
    }

    request_builder.headers(request_headers)
}

/// Dispatch an FSQL command and print the response
pub fn dispatch_command(
    query: &str,
    api: &str,
    token: &str,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let data = PostData {
        q: query.to_string(),
    };

    if verbose {
        eprintln!("🚀 Dispatching query to: {}", api);
        let pretty_payload = serde_json::to_string_pretty(&data)
            .unwrap_or_else(|_| "Failed to serialize".to_string());
        eprintln!("Payload: ");
        eprintln!("{pretty_payload}");
    }

    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_secs(10)) // Time to establish connection
        .timeout(Duration::from_secs(650)) // Total request timeout
        .build()?;

    // Build request with appropriate auth header based on token type
    let request = add_headers(client.post(api), token, verbose).json(&data);

    if verbose {
        eprintln!("🌐 Making POST request...");
    }

    // Send the request
    let response = match request.send() {
        Ok(response) => {
            if verbose {
                eprintln!("✅ Request sent successfully!");
                eprintln!("📊 Status Code: {}", response.status());
                eprintln!(
                    "🏷️  Status Text: {}",
                    response.status().canonical_reason().unwrap_or("Unknown")
                );

                // Log response headers
                eprintln!("📋 Response Headers:");
                for (key, value) in response.headers() {
                    eprintln!("   {}: {}", key, value.to_str().unwrap_or("<non-utf8>"));
                }

                // Special handling for 401/403 responses
                if response.status() == 401 {
                    eprintln!("🚨 HTTP 401 Unauthorized - Check your token!");
                } else if response.status() == 403 {
                    eprintln!("🚨 HTTP 403 Forbidden - Token valid but insufficient permissions!");
                }
            }

            response
        }
        Err(e) => {
            if verbose {
                eprintln!("❌ Request failed to send!");
                eprintln!("🔍 Error type: {}", std::any::type_name_of_val(&e));

                // Provide more specific error information
                if e.is_timeout() {
                    eprintln!("⏰ Error details: Request timed out");
                } else if e.is_connect() {
                    eprintln!(
                        "🔌 Error details: Connection failed - check if the server is running and the URL is correct"
                    );
                } else if e.is_request() {
                    eprintln!(
                        "📤 Error details: Request construction failed - check your URL and parameters"
                    );
                } else {
                    eprintln!("🔍 Error details: {}", e);
                }
            }

            return Err(Box::new(e));
        }
    };

    // Check if the response status indicates success
    let status = response.status();
    if !status.is_success() {
        if verbose {
            eprintln!("⚠️ Status code: {}", status);

            // Provide specific debugging for auth issues
            if status == 401 {
                eprintln!("🔍 Authentication Debugging:");
                eprintln!("  • Verify your token is correct and not expired");
                eprintln!("  • Check if the API endpoint expects the correct auth method");
            } else if status == 403 {
                eprintln!("🔍 Authorization Debugging:");
                eprintln!("  • Token is valid but may lack required permissions");
                eprintln!("  • Check if your token has access to this specific endpoint");
            }
        }

        // Try to read error response body
        match response.text() {
            Ok(error_body) => {
                if verbose {
                    eprintln!("📄 Error response body:");
                    eprintln!("{}", error_body);

                    // Look for common auth error patterns
                    if error_body.to_lowercase().contains("unauthorized") {
                        eprintln!("💡 Server says 'unauthorized' - likely a token issue");
                    } else if error_body.to_lowercase().contains("invalid")
                        && error_body.to_lowercase().contains("token")
                    {
                        eprintln!("💡 Server says invalid token - check token format/expiration");
                    }
                }
                return Err(format!("Server returned error {}: {}", status, error_body).into());
            }
            Err(body_err) => {
                if verbose {
                    eprintln!("❌ Could not read error response body: {}", body_err);
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
        eprintln!("📖 Reading response body...");
    }

    let response_text = response.text().unwrap();
    Ok(response_text)
}
