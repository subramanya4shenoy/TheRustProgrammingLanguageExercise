use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("problem parsing file args {}", err);
        process::exit(1);
    });

    if let Err (e) = minigrep::run(config) {
        eprintln!("App Error: {e}");
        process::exit(1); 
    };
}