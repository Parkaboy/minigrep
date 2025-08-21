use minigrep::Config;
use minigrep::errors::ErrorHandler;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_handler = ErrorHandler::new();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        error_handler.print_argument_error(err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        error_handler.print_application_error(&e.to_string());
        process::exit(1);
    }
}
