use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: here
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
    // ANCHOR_END: here

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    // ANCHOR: here
}
// ANCHOR_END: here
