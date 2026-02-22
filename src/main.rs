use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {err}");
        process::exit(1);
    });

    println!("--- Setup ---");
    println!("Search query: {}", config.query);
    println!("Filepath: {}", config.file_path);
    println!("Case sensitive: {}", config.case_sensitive);
    println!("");

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path).expect("Should be able to read file");

    let mut results = search(&config.query, &contents, &config.case_sensitive)
        .filter(|line| !line.is_empty())
        .peekable();

    if results.peek().is_some() {
        println!("--- Search results found ---");
        for line in results {
            println!("{}", line);
        }
    } else {
        eprint!("--- No search results found ---")
    }

    Ok(())
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path."),
        };

        let case_sensitivite = match args.next() {
            Some(_arg) => true,
            None => false,
        };

        Ok(Config {
            query,
            file_path,
            case_sensitive: case_sensitivite,
        })
    }
}
