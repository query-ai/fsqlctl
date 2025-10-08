use clap::Parser;
// use rand::seq::SliceRandom;
use clearscreen;
use rand::prelude::IndexedRandom;
use rand::rng;
use std::io;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    host: String, // Hostname for FSQL API

    /// Port
    #[arg(long, default_value_t = 80)]
    port: u8,
}

fn main() {
    let args = Args::parse();
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
    clearscreen::clear().expect("");
    println!("Federated Search Query Language (FSQL) Interpreter");
    println!("API: {}:{}", args.host, args.port);

    loop {
        print!("fsql> ");
        io::stdout().flush().unwrap(); // Flush

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("??? WTF was that?!");

        let trimmed_input = input.trim(); // Kill leading and trailing whitespace

        match trimmed_input {
            "exit" => {
                let mut rand_gen = rng();
                if let Some(goodbye_msg) = exit_messages.choose(&mut rand_gen) {
                    println!("{}", goodbye_msg);
                } else {
                    println!("Exiting REPL."); // The vec won't be empty but rust makes me cover the case since it's mutable
                }
                break;
            }
            "query" => {
                println!("In theory, dispatch a query!");
            }
            "explain" => {
                println!("It's a pain but you gotta explain!");
            }
            _ => {
                println!("(╯°□°)╯︵ ┻━┻: '{}'", trimmed_input);
            }
        }
    }
}
