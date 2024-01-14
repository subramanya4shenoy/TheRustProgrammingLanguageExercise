use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|_err: &str| {
        process::exit(1);
    });

    if let Err (_e) = minigrep::run(config) {
        process::exit(1);
    };
}