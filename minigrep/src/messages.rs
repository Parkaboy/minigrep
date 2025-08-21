use colored::*;

pub fn print_no_results() {
    println!("{}", "No se encontraron coincidencias.".yellow());
}

pub fn print_results_count(count: usize) {
    println!("{} coincidencias encontradas:", count.to_string().green());
}

pub fn print_line_with_number(line_num: usize, highlighted_line: String) {
    let colored_line_num = color_line_number(line_num);
    println!("{}: {}", colored_line_num, highlighted_line);
}

fn color_line_number(line_num: usize) -> String {
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
