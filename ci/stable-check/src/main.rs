use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
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

enum Error {
    Io(io::Error),
    LintFailure(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::LintFailure(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
           &Error::Io(ref e) => write!(f, "I/O error: {}", e),
           &Error::LintFailure(ref e) => write!(f, "Lint failed: {}", e),
        }
    }
}

fn check_directory(dir: &Path) -> Result<(), Error> {
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
            return Err(Error::LintFailure(format!("Feature flag found in {:?}", path)));
        }
    }

    Ok(())
}