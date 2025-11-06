use clap::Parser;
use colored::Colorize;
use std::io::IsTerminal;

mod api;
mod repl;
mod stdio;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Bearer token or API key for authentication")]
    pub token: String,

    #[arg(
        long,
        default_value = "api.dev.query.ai",
        help = "Hostname for FSQL API"
    )]
    pub host: String,

    #[arg(
        long,
        default_value = "search/translation/fsql",
        help = "Path to endpoint"
    )]
    pub path: String,

    #[arg(long, default_value_t = 443, help = "Port number for the API")]
    pub port: u16,

    #[arg(short, long, help = "Enable verbose output for debugging")]
    pub verbose: bool,

    #[arg(
        short,
        long,
        help = "Read FSQL command from a file",
        conflicts_with = "command"
    )]
    pub file: Option<String>,

    #[arg(
        short,
        long,
        help = "Execute FSQL command directly",
        conflicts_with = "file"
    )]
    pub command: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Check for explicit input methods, then piped input, then REPL
    if let Some(command) = args.command.clone() {
        if !std::io::stdin().is_terminal() {
            eprintln!(
                "{}",
                "❌ Cannot pipe to stdin and pass a command at the same time".red(),
            );
            std::process::exit(1);
        }
        let api_url = format!("https://{}/{}", args.host, args.path);
        stdio::process_command(&command, &api_url, &args.token, args.verbose);
    } else if let Some(file_path) = args.file.clone() {
        if !std::io::stdin().is_terminal() {
            eprintln!(
                "{}",
                "❌ Cannot pipe to stdin and pass a file at the same time".red(),
            );
            std::process::exit(1);
        }
        stdio::handle_file(args, &file_path);
    } else if !std::io::stdin().is_terminal() {
        stdio::handle_stdin(args);
    } else {
        repl::handle_repl(args);
    }
}
