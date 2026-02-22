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

    println!("Search query: {}", config.query);
    println!("Filepath: {}", config.file_path);
    println!("");

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path).expect("Should be able to read file");

    let mut found: bool = false;
    for line in search(&config.query, &contents, config.ignore_case) {
        if !line.is_empty() {
            println!("{}", line);
            found = true;
        }
    }

    if !found {
        eprint!(
            "Could not find query: {} in file: {}",
            &config.query, &config.file_path
        );
    }

    Ok(())
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
