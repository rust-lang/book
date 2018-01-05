extern crate minigrep13_1;

use std::env;
use std::process;

use minigrep13_1::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();
	
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep13_1::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

