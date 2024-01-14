use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(c) => c,
        Err(err) => {
            println!("problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = match fs::read_to_string(&config.filename) {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };
    println!("with text {}", content);
    Ok(())
}
