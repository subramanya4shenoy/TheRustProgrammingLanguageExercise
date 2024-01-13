use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing input: {}", err);
        process::exit(1);
    });

    println!("Sraching for the text {}", config.query);
    println!("In the file {}", config.file_path);

    if let Err (e) = run(config) {
        println!("App error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String =
    fs::read_to_string(config.file_path) ?;
    println!("contents {}", contents);
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result <Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments are insuffiecient!!");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
    }
}
