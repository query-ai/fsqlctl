use crate::{Args, api};
use clearscreen;
use rand::prelude::IndexedRandom;
use rand::rng;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::path::PathBuf;

pub fn handle_repl(args: Args) {
    let api_url = format!("https://{}/{}", args.host, args.path);

    println!("Federated Search Query Language (FSQL) Interpreter");
    println!("API: {api_url}");

    // Initialize rustyline editor
    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Failed to initialize readline editor: {}", e);
            std::process::exit(1);
        }
    };

    // Set up history file path
    let history_path = get_history_path();

    // Load existing history
    if let Err(_) = rl.load_history(&history_path) {
        // History file doesn't exist yet, which is fine for first run
    }

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

            match rl.readline(&prompt) {
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
                    // Handle Ctrl+C
                    println!("^C");
                    input.clear();
                    line_count = 0;
                    consecutive_empty_lines = 0;
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    // Handle Ctrl+D - exit the program
                    println!();
                    save_history_and_exit(&mut rl, &history_path);
                }
                Err(err) => {
                    eprintln!("Error reading input: {err}");
                    continue;
                }
            }
        }

        let trimmed_input = input.trim();

        // Skip empty input
        if trimmed_input.is_empty() {
            continue;
        }

        // Add non-empty commands to history
        if let Err(_) = rl.add_history_entry(trimmed_input) {
            // History add failed, but we continue
        }

        let lower_input = trimmed_input.to_lowercase();

        // Process the complete input (use cleaned input for API calls)
        if lower_input.starts_with("explain ") || lower_input.starts_with("query ") {
            let result = api::dispatch_query(trimmed_input, &api_url, &args.token, args.verbose);
            match result {
                Ok(_response) => {
                    // Response is already printed in the function
                }
                Err(e) => {
                    eprintln!("❌ Error dispatching command: {e}");
                }
            }
        } else if lower_input == "help" || lower_input == "h" {
            println!("📚 FSQL REPL Help:");
            println!("  EXPLAIN <fsql>   - Get query execution details");
            println!("  help, h          - Show this help message");
            println!("  clear            - Clear the screen");
            println!("  exit             - Exit the REPL");
            println!();
            println!("💡 Tips:");
            println!("  • Multiline queries can be pasted");
            println!("  • Use \\reset to clear a query without submitting it");
            println!("  • Hit enter twice to send your command to the FSQL API");
            println!("  • End a command with ';' to end multiline input and send your command");
            println!("  • Press Ctrl+D (Unix) or Ctrl+Z (Windows) to exit");
            println!("  • Use Up/Down arrows to navigate command history");
            println!("  • Use Ctrl+R for reverse history search");
        } else if lower_input == "clear" {
            clearscreen::clear().expect("Failed to clear screen");
            println!("Federated Search Query Language (FSQL) Interpreter");
            println!("API: {}", api_url);
        } else if lower_input == "exit" {
            println!();
            save_history_and_exit(&mut rl, &history_path);
        } else {
            println!("(╯°□°)╯︵ ┻━┻ Invalid Command");
            println!("💡 Type 'help' for available commands");
        }
    }
}

fn get_history_path() -> PathBuf {
    dirs::home_dir()
        .map(|mut path| {
            path.push(".fsql_history");
            path
        })
        .unwrap_or_else(|| PathBuf::from(".fsql_history"))
}

fn save_history_and_exit(rl: &mut DefaultEditor, history_path: &PathBuf) -> ! {
    // Save history before exit
    if let Err(e) = rl.save_history(history_path) {
        eprintln!("Warning: Failed to save history: {}", e);
    }

    print_goodbye();
    std::process::exit(0);
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
        println!("{goodbye_msg}");
    } else {
        println!("Exiting REPL.");
    }
}
