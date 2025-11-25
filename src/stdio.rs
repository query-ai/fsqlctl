use crate::{Args, api};
use colored::Colorize;
use serde_json;
use std::fs;
use std::io::{self, Read};

/// Explain configured connectors
///
/// Prints a summary of connectors
fn handle_explain_connectors(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainConnectorsResponse>(&response_text) {
                Ok(data) => {
                    if verbose {
                        eprintln!("{}", "Command:".cyan());
                        eprintln!("{}", data.command);
                        eprintln!();
                    }
                    eprintln!("{}", "Connectors:");
                    match serde_json::to_string_pretty(&data.connectors) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Explain schema
///
/// Prints a description of the graphql schema for a given path
fn handle_explain_schema(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainSchemaResponse>(&response_text) {
                Ok(data) => {
                    if verbose {
                        eprintln!("{}", "Command:".cyan());
                        eprintln!("{}", data.command);
                        eprintln!();
                    }
                    eprintln!("{}", "Schema:");
                    match serde_json::to_string_pretty(&data.schema) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Explain attributes
///
/// Prints an expansion of the given attributes
fn handle_explain_attributes(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainAttributesResponse>(&response_text) {
                Ok(data) => {
                    if verbose {
                        eprintln!("{}", "Command:".cyan());
                        eprintln!("{}", data.command);
                        eprintln!();
                    }
                    eprintln!("{}", "Attributes:");
                    match serde_json::to_string_pretty(&data.attributes) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Explain version
///
/// Prints version info from the API
fn handle_explain_version(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainVersionResponse>(&response_text) {
                Ok(data) => {
                    eprintln!("{}", "Version Information:");
                    match serde_json::to_string_pretty(&data) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Summarize
///
/// Prints summary information
fn handle_summarize(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            match serde_json::from_str::<api::SummarizeResponse>(&response_text) {
                Ok(data) => {
                    eprintln!("{}", "Summarize Details:");
                    match serde_json::to_string_pretty(&data) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Explain graphql
///
/// Prints the graphql version of a given FSQL query
fn handle_explain_graphql(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainGraphqlResponse>(&response_text) {
                Ok(data) => {
                    eprintln!("{}", "Graphql Query:");
                    println!("{}", data.query);
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Explain an FSQL query
///
/// Prints an expanded version of the query
fn handle_explain(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainResponse>(&response_text) {
                Ok(data) => {
                    if verbose {
                        eprintln!("{}", "Original Input:".cyan());
                        eprintln!("{}", data.input);
                        eprintln!();
                        eprintln!("{}", "Command:".cyan());
                        eprintln!("{}", data.command);
                        eprintln!();
                    }
                    eprintln!("{}", "Expanded Query:");
                    // If the parsed value is a string, just print it so that the newline characters are
                    // honoured. If not, use the pretty printer from serde_json
                    match &data.expanded_query {
                        serde_json::Value::String(s) => println!("{}", s),
                        _ => match serde_json::to_string_pretty(&data.expanded_query) {
                            Ok(pretty_json) => eprintln!("{}", pretty_json),
                            Err(_) => eprintln!("{}", response_text), // Fallback to raw text
                        },
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Validate an FSQL query
///
/// Dispatches a validation request to the FSQL API.
fn handle_validate(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            match serde_json::from_str::<api::ValidateResponse>(&response_text) {
                Ok(data) => {
                    if verbose {
                        eprintln!("{}", "Command:".cyan());
                        eprintln!("{}", data.command);
                        eprintln!();
                    }
                    // The invalid query part probably will never display given the current API because
                    // it doesn't actually return is_valid: false - it gives a different error with an
                    // error code. We should probably fix the API.
                    if data.is_valid {
                        eprintln!("✅ Query is valid");
                        std::process::exit(0);
                    } else {
                        eprintln!("❌ Query is invalid");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

/// Dispatch an FSQL query
///
/// Dispatches a query to the FSQL API. User-facing messages are printed to
/// stderr and the actual query results are written to stdout so that the
/// tool will work in a pipeline.
fn handle_query(input: &str, api_url: &str, token: &str, verbose: bool) {
    let result = api::dispatch_command(input, api_url, token, verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::QueryResponse>(&response_text) {
                Ok(data) => {
                    if verbose {
                        eprintln!("{}", "Command:".cyan());
                        eprintln!("{}", data.command);
                        eprintln!();
                        eprintln!("{}", "Trace ID:".cyan());
                        eprintln!("{}", data.trace_id);
                        eprintln!();
                    }
                    eprintln!("{}", "Search ID:".cyan());
                    eprintln!("{}", data.search_id);
                    eprintln!();
                    eprintln!("{}", "Results:");
                    match serde_json::to_string_pretty(&data.results) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    eprintln!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
            std::process::exit(1);
        }
    }
}

pub fn process_command(input: &str, api_url: &str, token: &str, verbose: bool) {
    if input.is_empty() {
        eprintln!("(╯°□°)╯︵ ┻━┻ Invalid Command");
        std::process::exit(1);
    }
    let lower_input = input.to_lowercase();

    if lower_input.starts_with("explain connectors") {
        handle_explain_connectors(input, api_url, token, verbose);
    } else if lower_input.starts_with("explain schema ") {
        handle_explain_schema(input, api_url, token, verbose);
    } else if lower_input.starts_with("explain graphql ") {
        handle_explain_graphql(input, api_url, token, verbose);
    } else if lower_input.starts_with("explain version") {
        handle_explain_version(input, api_url, token, verbose);
    } else if lower_input.starts_with("explain attributes ") {
        handle_explain_attributes(input, api_url, token, verbose);
    } else if lower_input.starts_with("explain ") {
        handle_explain(input, api_url, token, verbose);
    } else if lower_input.starts_with("summarize ") {
        handle_summarize(input, api_url, token, verbose);
    } else if lower_input.starts_with("validate ") {
        handle_validate(input, api_url, token, verbose);
    } else if lower_input.starts_with("query ") {
        handle_query(input, api_url, token, verbose);
    } else {
        eprintln!("(╯°□°)╯︵ ┻━┻ Invalid Command");
        std::process::exit(1);
    }
}

/// Handle reading an FSQL query piped in on stdin
pub fn handle_stdin(args: Args, token: &str) {
    let api_url = format!("https://{}/{}", args.host, args.path);

    // Read all of stdin
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error reading from stdin: {}", e);
            std::process::exit(1);
        }
    }

    let input = buffer.trim();
    process_command(input, &api_url, token, args.verbose);
}

/// Handle loading an FSQL query from a file.
pub fn handle_file(args: Args, token: &str, file_path: &str) {
    let api_url = format!("https://{}/{}", args.host, args.path);

    // Read all from file
    let buffer = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading from file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    let input = buffer.trim();
    process_command(input, &api_url, token, args.verbose);
}
