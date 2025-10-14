use clap::Parser;
use clearscreen;
use rand::prelude::IndexedRandom;
use rand::rng;
use std::io;
use std::io::Write;

mod api;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    token: String, // Bearer token or API key for authentication

    #[arg(long, default_value = "api.dev.query.ai")]
    host: String, // Hostname for FSQL API

    #[arg(long, default_value = "search/translation/fsql")]
    path: String, // Path to endpoint

    /// Port
    #[arg(long, default_value_t = 443)]
    port: u16,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let api_url = format!("https://{}/{}", args.host, args.path);

    println!("Federated Search Query Language (FSQL) Interpreter");
    println!("API: {api_url}");

    loop {
        // Read multiline input
        let mut input = String::new();
        let mut line_count = 0;
        let mut consecutive_empty_lines = 0;

        loop {
            if line_count == 0 {
                print!("fsql> ");
            } else {
                print!("{:3}> ", line_count + 1);
            }
            io::stdout().flush().unwrap();

            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => {
                    // Detect EOF (Ctrl+D on Unix, Ctrl+Z on Windows) and add a newline
                    // before printing the goodbye message
                    println!("\n");
                    print_goodbye();
                    return;
                }
                Ok(_) => {
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
                    line_count += 1;

                    // Track consecutive empty lines for double-newline termination
                    if trimmed_line.is_empty() {
                        consecutive_empty_lines += 1;
                    } else {
                        consecutive_empty_lines = 0;
                    }

                    // There are two means of exiting the loop here, either someone
                    // issues a single line command (i.e. help or exit) or they
                    // include two newline characters
                    if consecutive_empty_lines >= 2 {
                        break;
                    } else if line_count == 1
                        && !trimmed_line.is_empty()
                        && !trimmed_line.contains(' ')
                    {
                        break;
                    }

                    // If this is the first line and it's empty, break to handle it as a command
                    if line_count == 1 && trimmed_line.is_empty() {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error reading input: {}", e);
                    continue;
                }
            }
        }

        let trimmed_input = input.trim();
        let lower_input = trimmed_input.to_lowercase();

        // Skip empty input
        if trimmed_input.is_empty() {
            continue;
        }

        // Clean up terminators for processing (but keep the original query structure)
        let cleaned_input = if trimmed_input.ends_with(';') {
            trimmed_input.trim_end_matches(';').trim()
        } else {
            trimmed_input
        };

        // Process the complete input (use cleaned input for API calls)
        if lower_input.starts_with("query") {
            let result = api::dispatch_query(cleaned_input, &api_url, &args.token, args.verbose);
            match result {
                Ok(_response) => {
                    // Response is already printed in the function
                }
                Err(e) => {
                    println!("❌ Error dispatching command: {e}");
                }
            }
        } else if lower_input.starts_with("explain") {
            let result = api::dispatch_query(cleaned_input, &api_url, &args.token, args.verbose);
            match result {
                Ok(_response) => {
                    // Response is printed in the dispatch_query function
                }
                Err(e) => {
                    println!("❌ Error dispatching command:");
                    println!("{e}")
                }
            }
        } else if lower_input == "help" || lower_input == "h" {
            println!("📚 FSQL REPL Help:");
            println!("  QUERY <fsql>     - Execute an FSQL query");
            println!("  EXPLAIN <sql>    - Get query execution details");
            println!("  help, h          - Show this help message");
            println!("  clear            - Clear the screen");
            println!("  exit             - Exit the REPL");
            println!();
            println!("💡 Tips:");
            println!("  • Hit enter twice to dispatch your command to the FSQL API");
            println!("  • Multiline queries can be pasted");
            println!("  • Use \\reset to clear a query without submitting it");
            println!("  • Press Ctrl+D (Unix) or Ctrl+Z (Windows) to exit");
        } else if lower_input == "clear" {
            clearscreen::clear().expect("Failed to clear screen");
            println!("Federated Search Query Language (FSQL) Interpreter");
            println!("API: {}", api_url);
        } else {
            println!("(╯°□°)╯︵ ┻━┻ Invalid Command");
            println!("💡 Type 'help' for available commands");
        }
    }
}

fn print_goodbye() {
    let exit_messages = vec![
        "FUQL off!",
        "I was just FUQLing with you...",
        "Get the FUQL out of here!",
        "Take your stupid FUQLing rope!",
        "Where the FUQL do you think you're going?",
        "What the FUQL?!?",
        "Well FUQL you, too!",
        "Well aren't you FUQLing special?",
        "What did you do?! FUQLin'... what the FUQLin' FUQL! Who the FUQL, FUQLed this FUQLin'? FUQL. How did you two FUQLin', FUQLs?......... FUQL!!!",
    ];
    let mut rand_gen = rng();
    if let Some(goodbye_msg) = exit_messages.choose(&mut rand_gen) {
        println!("{}", goodbye_msg);
    } else {
        println!("Exiting REPL.");
    }
}
