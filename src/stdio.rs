use crate::{Args, api};
use colored::Colorize;
use serde_json;
use std::io::{self, Read};

pub fn handle_stdin(args: Args) {
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

    // Check if input is empty
    if input.is_empty() {
        eprintln!("(╯°□°)╯︵ ┻━┻ Invalid Command");
        std::process::exit(1);
    }

    let lower_input = input.to_lowercase();

    if lower_input.starts_with("explain ") {
        let result = api::dispatch_query(input, &api_url, &args.token, args.verbose);
        match result {
            Ok(response_text) => {
                // Parse and pretty print JSON response
                match serde_json::from_str::<api::ExplainResponse>(&response_text) {
                    Ok(data) => {
                        if args.verbose {
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
                        if args.verbose {
                            eprintln!("❌ Failed to parse response as JSON: {}", e);
                        }
                        eprintln!("{}", response_text); // Output raw response if not valid JSON
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error dispatching command: {e}");
            }
        }
    } else if lower_input.starts_with("validate ") {
        let result = api::dispatch_query(input, &api_url, &args.token, args.verbose);
        match result {
            Ok(response_text) => {
                // Parse and pretty print JSON response
                match serde_json::from_str::<api::ValidateResponse>(&response_text) {
                    Ok(data) => {
                        if args.verbose {
                            eprintln!("{}", "Command:".cyan());
                            eprintln!("{}", data.command);
                            eprintln!();
                        }
                        // The invalid query part probably will never display given the current API because
                        // it doesn't actually return is_valid: false - it gives a different error with an
                        // error code. We should probably fix the API.
                        if data.is_valid {
                            eprintln!("✅ Query is valid")
                        } else {
                            eprintln!("❌ Query is invalid");
                        }
                    }
                    Err(e) => {
                        if args.verbose {
                            eprintln!("❌ Failed to parse response as JSON: {}", e);
                        }
                        eprintln!("{}", response_text); // Output raw response if not valid JSON
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error dispatching command: {e}");
            }
        }
    } else if lower_input.starts_with("query ") {
        let result = api::dispatch_query(input, &api_url, &args.token, args.verbose);
        match result {
            Ok(response_text) => {
                // Parse and pretty print JSON response
                match serde_json::from_str::<api::QueryResponse>(&response_text) {
                    Ok(data) => {
                        if args.verbose {
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
                        if args.verbose {
                            eprintln!("❌ Failed to parse response as JSON: {}", e);
                        }
                        eprintln!("{}", response_text); // Output raw response if not valid JSON
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error dispatching command: {e}");
            }
        }
    } else {
        eprintln!("(╯°□°)╯︵ ┻━┻ Invalid Command");
        std::process::exit(1);
    }
}
