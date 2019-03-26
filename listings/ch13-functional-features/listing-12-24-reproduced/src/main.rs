use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: ch13
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
    // ANCHOR_END: ch13

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    // ANCHOR: ch13
}
// ANCHOR_END: ch13
