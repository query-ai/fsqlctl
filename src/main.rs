use clap::Parser;

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

    #[arg(short = 'i', long = "stdin", help = "Accept input from stdin")]
    pub stdin: bool,
}

fn main() {
    let args = Args::parse();

    if args.stdin {
        stdio::handle_stdin(args);
    } else {
        repl::handle_repl(args);
    }
}
