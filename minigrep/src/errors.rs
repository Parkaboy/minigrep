use colored::*;

pub fn print_argument_error(err: &str) {
    eprintln!("{}: {}", "Error al parsear argumentos".red().bold(), err);
}

pub fn print_application_error(err: &str) {
    eprintln!("{}: {}", "Error de aplicación".red().bold(), err);
}

pub fn print_file_read_error(file_path: &str, err: &str) {
    eprintln!("{}: No se pudo leer el archivo '{}': {}", 
        "Error de archivo".red().bold(), 
        file_path.yellow(), 
        err
    );
}

pub fn print_search_error(query: &str, err: &str) {
    eprintln!("{}: Error al buscar '{}': {}", 
        "Error de búsqueda".red().bold(), 
        query.yellow(), 
        err
    );
}
