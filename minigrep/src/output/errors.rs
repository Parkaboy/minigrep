use colored::*;
use crate::output::i18n::I18n;

pub struct ErrorHandler {
    i18n: I18n,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler {
            i18n: I18n::new(),
        }
    }

    pub fn new_with_language(i18n: I18n) -> Self {
        ErrorHandler { i18n }
    }

    pub fn print_argument_error(&self, err: &str) {
        let title = self.i18n.get_message("argument_error");
        eprintln!("{}: {}", title.red().bold(), err);
    }

    pub fn print_application_error(&self, err: &str) {
        let title = self.i18n.get_message("application_error");
        eprintln!("{}: {}", title.red().bold(), err);
    }

    pub fn print_file_read_error(&self, file_path: &str, err: &str) {
        let title = self.i18n.get_message("file_read_error");
        let message = self.i18n.get_message("file_read_message");
        eprintln!("{}: {} '{}': {}", 
            title.red().bold(), 
            message,
            file_path.yellow(), 
            err
        );
    }

    pub fn print_search_error(&self, query: &str, err: &str) {
        let title = self.i18n.get_message("search_error");
        let message = self.i18n.get_message("search_error_message");
        eprintln!("{}: {} '{}': {}", 
            title.red().bold(), 
            message,
            query.yellow(), 
            err
        );
    }
}
