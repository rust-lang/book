//use std::env;
use std::fs::File;
use std::io::prelude::*;
//use std::process;
use std::error::Error;
use std::vec::Vec;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

	  for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    //println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


 fn search <'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
   let mut results = Vec::new();
   
   for line in contents.lines() {
       if line.contains(query) {
            results.push(line);
        }
    }
	
	results
 }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}