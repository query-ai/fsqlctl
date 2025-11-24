use crate::{Args, api};
use clearscreen;
use colored::Colorize;
use rand::prelude::IndexedRandom;
use rand::rng;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use serde_json;
use std::path::PathBuf;

/// Handle validate command
fn handle_validate(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ValidateResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{}", "Command:".cyan());
                        println!("{}", data.command);
                        println!();
                    }
                    // The invalid query part probably will never display given the current API because
                    // it doesn't actually return is_valid: false - it gives a different error with an
                    // error code. We should probably fix the API.
                    if data.is_valid {
                        println!("✅ Query is valid")
                    } else {
                        eprintln!("❌ Query is invalid");
                    }
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle explain graphql command
fn handle_explain_graphql(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainGraphqlResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{}", "Command:".cyan());
                        println!("{}", data.command);
                        println!();
                    }
                    println!("{}", data.query);
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle explain version command
fn handle_explain_version(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainVersionResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{}", "Command:".cyan());
                        println!("{}", data.command);
                        println!();
                    }
                    println!("fsql: {}", data.fsql);
                    println!(" qdm: {}", data.qdm);
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle explain connectors command
fn handle_explain_connectors(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainConnectorsResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{}", "Command:".cyan());
                        println!("{}", data.command);
                        println!();
                    }
                    println!("{}", "Connectors:");
                    match serde_json::to_string_pretty(&data.connectors) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                    let total = data.connectors.len();
                    if total == 1 {
                        println!("{} connector found", data.connectors.len());
                    } else {
                        println!("{} connectors found", data.connectors.len());
                    }
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle explain schema command
fn handle_explain_schema(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainSchemaResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{}", "Command:".cyan());
                        println!("{}", data.command);
                        println!();
                    }
                    println!("{}", "Schema:");
                    match serde_json::to_string_pretty(&data.schema) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle explain attributes command
fn handle_explain_attributes(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainAttributesResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{}", "Command:".cyan());
                        println!("{}", data.command);
                        println!();
                    }
                    println!("{}", "Attributes:");
                    for attr in data.attributes.iter() {
                        println!("{attr}");
                    }
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle explain command
fn handle_explain(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::ExplainResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{}", "Original Input:".cyan());
                        println!("{}", data.input);
                        println!();
                        println!("{}", "Command:".cyan());
                        println!("{}", data.command);
                        println!();
                    }
                    println!("{}", "Expanded Query:");
                    // If the parsed value is a string, just print it so that the newline characters are
                    // honoured. If not, use the pretty printer from serde_json
                    match &data.expanded_query {
                        serde_json::Value::String(s) => println!("{}", s),
                        _ => match serde_json::to_string_pretty(&data.expanded_query) {
                            Ok(pretty_json) => println!("{}", pretty_json),
                            Err(_) => println!("{}", response_text), // Fallback to raw text
                        },
                    }
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle query command
fn handle_query(trimmed_input: &str, api_url: &str, token: &str, args: &Args) {
    let result = api::dispatch_command(trimmed_input, api_url, token, args.verbose);
    match result {
        Ok(response_text) => {
            // Parse and pretty print JSON response
            match serde_json::from_str::<api::QueryResponse>(&response_text) {
                Ok(data) => {
                    if args.verbose {
                        println!("{} {}", "Command:".cyan(), data.command);
                        println!();
                        println!("{} {}", "Trace ID:".cyan(), data.trace_id);
                        println!();
                    }
                    println!("{} {}", "Search ID:".cyan(), data.search_id);
                    println!();
                    println!("{}", "Results:");
                    match serde_json::to_string_pretty(&data.results) {
                        Ok(pretty_json) => println!("{}", pretty_json),
                        Err(_) => println!("{}", response_text), // Fallback to raw text
                    }
                    let total = data.results.len();
                    if total == 1 {
                        println!("{} result found", data.results.len());
                    } else {
                        println!("{} results found", data.results.len());
                    }
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("❌ Failed to parse response as JSON: {}", e);
                    }
                    println!("{}", response_text); // Output raw response if not valid JSON
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error dispatching command: {e}");
        }
    }
}

/// Handle help command
fn handle_help() {
    print_help();
    println!();
    print_tips();
}

/// Handle clear command
fn handle_clear(api_url: &str) {
    clearscreen::clear().expect("Failed to clear screen");
    print_welcome(api_url);
}

/// Handle exit command
fn handle_exit(rl_editor: &mut DefaultEditor, history_path: &PathBuf) -> ! {
    println!();
    save_history_and_exit(rl_editor, history_path);
}

/// Handle invalid command
fn handle_invalid_command() {
    println!("(╯°□°)╯︵ ┻━┻ {}", "Invalid Command".red());
    println!("💡 Type 'help' for available commands");
}

/// Launch an iteractive REPL to dispatch FSQL commands
pub fn handle_repl(args: Args, token: &str) {
    let api_url = format!("https://{}:{}/{}", args.host, args.port, args.path);
    print_welcome(&api_url);
    print_help();

    // Initialize rustyline editor
    let mut rl_editor = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Failed to initialize readline editor: {}", e);
            std::process::exit(1);
        }
    };

    // Set up history file path
    let history_path = get_history_path();

    // Load existing history if the file exists
    let _ = rl_editor.load_history(&history_path);

    loop {
        // Read multiline input
        let mut input = String::new();
        let mut line_count = 0;
        let mut consecutive_empty_lines = 0;

        loop {
            let prompt = if line_count == 0 {
                "fsql> ".to_string()
            } else {
                format!("{:3}> ", line_count + 1)
            };

            match rl_editor.readline(&prompt) {
                Ok(line) => {
                    // Check for special commands on any line
                    let trimmed_line = line.trim();
                    let lower_line = trimmed_line.to_lowercase();

                    // If \reset is typed, exit the multiline query, reset the buffer, and clear the screen
                    if line_count > 0 {
                        if lower_line == "\\reset" {
                            input.clear();
                            line_count = 0;
                            consecutive_empty_lines = 0;
                            clearscreen::clear().expect("Failed to clear screen");
                            continue;
                        }
                    }

                    input.push_str(&line);
                    input.push('\n');
                    line_count += 1;

                    // Track consecutive empty lines for double-newline termination
                    if trimmed_line.is_empty() {
                        consecutive_empty_lines += 1;
                    } else {
                        consecutive_empty_lines = 0;
                    }

                    // There are three means of exiting the loop here:
                    // 1. Someone issues a single line command (i.e. help or exit)
                    // 2. They include an empty newline
                    // 3. The line ends with a semicolon
                    if consecutive_empty_lines >= 1 {
                        break;
                    } else if line_count == 1
                        && !trimmed_line.is_empty()
                        && !trimmed_line.contains(' ')
                    {
                        break;
                    } else if trimmed_line.ends_with(';') {
                        break;
                    }

                    // If this is the first line and it's empty, break to handle it as a command
                    if line_count == 1 && trimmed_line.is_empty() {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // Handle Ctrl+C  - added the println here so it is obvious what happened
                    println!("^C");
                    input.clear();
                    line_count = 0;
                    consecutive_empty_lines = 0;
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    // Handle Ctrl+D - exit the program
                    println!();
                    save_history_and_exit(&mut rl_editor, &history_path);
                }
                Err(err) => {
                    eprintln!("{} {}", "Error reading input:".yellow(), err);
                    continue;
                }
            }
        }

        let trimmed_input = input.trim();

        // Skip empty input
        if trimmed_input.is_empty() {
            continue;
        }

        // Add non-empty commands to history (ignore error if add fails)
        let _ = rl_editor.add_history_entry(trimmed_input);

        let lower_input = trimmed_input.to_lowercase();

        // Process the complete input (use cleaned input for API calls)
        if lower_input.starts_with("validate ") {
            handle_validate(trimmed_input, &api_url, token, &args);
        } else if lower_input.starts_with("explain schema ") {
            handle_explain_schema(trimmed_input, &api_url, token, &args);
        } else if lower_input.starts_with("explain graphql ") {
            handle_explain_graphql(trimmed_input, &api_url, token, &args);
        } else if lower_input.starts_with("explain version") {
            handle_explain_version(trimmed_input, &api_url, token, &args);
        } else if lower_input.starts_with("explain connectors") {
            handle_explain_connectors(trimmed_input, &api_url, token, &args);
        } else if lower_input.starts_with("explain attributes ") {
            handle_explain_attributes(trimmed_input, &api_url, token, &args);
        } else if lower_input.starts_with("explain ") {
            handle_explain(trimmed_input, &api_url, token, &args);
        } else if lower_input.starts_with("query ") {
            handle_query(trimmed_input, &api_url, token, &args);
        } else if lower_input == "help" || lower_input == "h" {
            handle_help();
        } else if lower_input == "clear" {
            handle_clear(&api_url);
        } else if lower_input == "exit" {
            handle_exit(&mut rl_editor, &history_path);
        } else {
            handle_invalid_command();
        }
    }
}

/// Return the path where the RELP history is stored
fn get_history_path() -> PathBuf {
    dirs::home_dir()
        .map(|mut path| {
            path.push(".fsql_history");
            path
        })
        .unwrap_or_else(|| PathBuf::from(".fsql_history"))
}

/// Saves REPL history, prints a goodbye message, and exits
fn save_history_and_exit(rl_editor: &mut DefaultEditor, history_path: &PathBuf) -> ! {
    // Save history before exit
    if let Err(e) = rl_editor.save_history(history_path) {
        eprintln!("Warning: Failed to save history: {}", e);
    }

    print_goodbye();
    std::process::exit(0);
}

/// Print the REPL command list
fn print_help() {
    println!("📚 {}", "FSQL REPL Help:".cyan());
    println!("   EXPLAIN CONNECTORS          - Get details about all configured connectors");
    println!("   EXPLAIN VERSION             - List FSQL and QDM versions");
    println!("   EXPLAIN ATTRIBUTES <fsql>   - Get a list of explanded attributes");
    println!("   EXPLAIN <fsql>              - Get query execution details");
    println!("   EXPLAIN GRAPHQL <fsql>      - Show the graphql translation of the given FSQL");
    println!("   help, h                     - Show this help message");
    println!("   clear                       - Clear the screen");
    println!("   exit                        - Exit the REPL");
}

/// Print helpful REPL tips
fn print_tips() {
    println!("💡 {}", "Tips:".cyan());
    println!("  • Multiline queries can be pasted");
    println!("  • Use \\reset to clear a query without submitting it");
    println!("  • Hit enter twice to send your command to the FSQL API");
    println!("  • End a command with ';' to end multiline input and send your command");
    println!("  • Press Ctrl+D (Unix) or Ctrl+Z (Windows) to exit");
    println!("  • Use Up/Down arrows to navigate command history");
    println!("  • Use Ctrl+R for reverse history search");
}

/// Print the welcome text
fn print_welcome(api_url: &str) {
    let div = "================================================================================"
        .bright_blue();
    println!("{}", div);
    println!(
        "{} {} {}",
        "Federated Search Query Language".cyan(),
        "(FSQL)".bright_cyan(),
        "Interpreter".cyan(),
    );
    println!("🔗 {} {}", "API:".cyan(), api_url.green());
    println!("{}", div);
}

/// Select a random goodbye message and print it
fn print_goodbye() {
    let exit_messages = vec![
        "Query ya later!",
        "Ya'll query again now, ya hear?",
        "Catch you on the Query side!",
        "The FSQL was strong with this session.",
    ];
    let mut rand_gen = rng();
    if let Some(goodbye_msg) = exit_messages.choose(&mut rand_gen) {
        println!("❤ {}", goodbye_msg.yellow());
    } else {
        println!("🙁 Exiting REPL.");
    }
}
