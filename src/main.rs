//! # fsqlctl
//!
//! `fsqlctl` is a command line utility used to interact with the FSQL API.
//!
//! This utility can dispatch several types of commands in FSQL format. These
//! are:
//!
//! - `EXPLAIN` - get a fully expanded version of the query
//! - `VALIDATE` - determine if a query has valid syntax
//! - `QUERY` - execute a query and get the results
//!
//! ## REPL
//!
//! The easiest way to interact with `fsqlctl` is to run simply run it. It takes a
//! single positional argument which can be either a JWT token or an API key. Running
//! without other arguments will drop you into an interactive shell which you can
//! use to dispatch commands to the FSQL API.
//!
//! ```shell
//! cargo run <SEKRET-HERE>
//!    Compiling fsqlctl v0.12.0 (/home/brant/projects/qai/fsqlctl)
//!     Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.75s
//!      Running `target/debug/fsqlctl foobar`
//! ================================================================================
//! Federated Search Query Language (FSQL) Interpreter
//! 🔗 API: https://api.dev.query.ai/search/translation/fsql
//! ================================================================================
//! 📚 FSQL REPL Help:
//!    EXPLAIN CONNECTORS         - Get details about all configured connectors
//!    EXPLAIN VERSION            - List FSQL and QDM versions
//!    EXPLAIN ATTRIBUTES <fsql>  - Get a list of explanded attributes
//!    EXPLAIN <fsql>             - Get query execution details
//!    EXPLAIN GRAPHQL <fsql>      - Show the graphql translation of the given FSQL
//!    help, h                    - Show this help message
//!    clear                      - Clear the screen
//!    exit                       - Exit the REPL
//! fsql>
//! ```
//!
//! ## Pipes
//!
//! When passed information via stdin, fsqlctl will work as a stage in a pipe. It
//! will write the results of the command to stdout. For EXPLAIN and VALIDATE
//! commands it prints textual output. When piped a QUERY command it will write
//! the output as a json array. This opens up the ability to use it with tools
//! such as `jq`.
//!
//! Example:
//!
//! ```shell
//! echo "QUERY module_activity.** WITH module_activity.activity_id = LOAD AND module_activity.actor.process.file.name = 'regsvr32.exe' AFTER 1h" | fsqlctl eyJ...lA | jq
//! ```
//!
//! ## File & Command Switch
//!
//! You may also route a command to `fsqlctl` by loading it from a file (``-f``) or
//! placing it directly as a command line argument (``-c``). In this mode it will
//! output in the same way as it would if it were piped a command.
//!
//! File:
//! ```shell
//! fsqlctl eyJ...lA -f query.txt
//! ```
//!
//! Switch:
//! ```shell
//! fsqlctl eyJ...lA -c "QUERY module_activity.** WITH module_activity.activity_id = LOAD"
//! ```

////////////////////////////////////////////////////////////////////////////////
use clap::Parser;
use colored::Colorize;
use std::io::IsTerminal;

mod api;
mod repl;
mod stdio;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        help = "Bearer token or API key for authentication",
        env = "FSQL_TOKEN"
    )]
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
