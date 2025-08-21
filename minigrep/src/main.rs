use minigrep::Config;
use minigrep::errors;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        errors::print_argument_error(err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        errors::print_application_error(&e.to_string());
        process::exit(1);
    }
}
