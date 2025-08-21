use colored::*;
use crate::i18n::I18n;

pub struct MessageHandler {
    i18n: I18n,
}

impl MessageHandler {
    pub fn new_with_language(i18n: I18n) -> Self {
        MessageHandler { i18n }
    }

    pub fn print_no_results(&self) {
        let message = self.i18n.get_message("no_results");
        println!("{}", message.yellow());
    }

    pub fn print_results_count(&self, count: usize) {
        let suffix = self.i18n.get_message("results_count");
        println!("{}{}", count.to_string().green(), suffix);
    }

    pub fn print_line_with_number(&self, line_num: usize, highlighted_line: String) {
        let colored_line_num = self.color_line_number(line_num);
        println!("{}: {}", colored_line_num, highlighted_line);
    }

    fn color_line_number(&self, line_num: usize) -> String {
        // Usar diferentes colores basados en el número de línea
        match line_num % 6 {
            0 => line_num.to_string().blue().bold(),
            1 => line_num.to_string().green().bold(),
            2 => line_num.to_string().yellow().bold(),
            3 => line_num.to_string().magenta().bold(),
            4 => line_num.to_string().cyan().bold(),
            5 => line_num.to_string().red().bold(),
            _ => line_num.to_string().white().bold(), // Nunca debería llegar aquí
        }.to_string()
    }
}
