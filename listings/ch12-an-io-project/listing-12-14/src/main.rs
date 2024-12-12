// ANCHOR: here
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // ANCHOR: here
    if let Err(e) = minigrep::run(config) {
        // --snip--
        // ANCHOR_END: here
        println!("Application error: {e}");
        process::exit(1);
        // ANCHOR: here
    }
}
// ANCHOR_END: here
