use crate::{Args, api};
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

    // Check if input starts with "explain" or "query" (case insensitive)
    if !lower_input.starts_with("explain ") && !lower_input.starts_with("query ") {
        eprintln!("(╯°□°)╯︵ ┻━┻ Invalid Command");
        std::process::exit(1);
    }

    // Dispatch the query using the same code as the REPL
    let result = api::dispatch_query(input, &api_url, &args.token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and print JSON response
            match serde_json::from_str::<serde_json::Value>(&response_text) {
                Ok(data) => match serde_json::to_string(&data) {
                    Ok(valid_json) => {
                        println!("{valid_json}");
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to serialize response to JSON: {}", e);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("❌ Failed to parse response as JSON: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {}", e);
            std::process::exit(1);
        }
    }
}
