use std::error::Error;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Please pass a src directory as the first argument");
        std::process::exit(1);
    });

    match check_directory(&Path::new(&arg)) {
        Ok(()) => println!("passed!"),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }

}

fn check_directory(dir: &Path) -> Result<(), Box<Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.contains("#![feature") {
            return Err(From::from(format!("Feature flag found in {:?}", path)));
        }
    }

    Ok(())
}
