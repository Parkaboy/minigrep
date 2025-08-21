use std::env;
use std::error::Error;
use std::fs;
use colored::*;

mod messages;
pub mod errors;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path).map_err(|e| {
        errors::print_file_read_error(&config.file_path, &e.to_string());
        e
    })?;
    
    let results = if config.ignore_case {
        search_case_insensitive_with_line_numbers(&config.query, &contents)
    } else {
        search_with_line_numbers(&config.query, &contents)
    };
    
    if results.is_empty() {
        messages::print_no_results();
    } else {
        messages::print_results_count(results.len());
        for (line_num, line) in results {
            let highlighted_line = highlight_query(&config.query, line);
            messages::print_line_with_number(line_num, highlighted_line);
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    for (line_num, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((line_num + 1, line)); // +1 porque enumerate() empieza en 0
        }
    }
    results
}

pub fn search_case_insensitive_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for (line_num, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((line_num + 1, line));
        }
    }
    results
}

fn highlight_query<'a>(query: &str, line: &'a str) -> String {
    if line.contains(query) {
        line.replace(query, &query.red().bold().to_string())
    } else {
        // Para búsqueda case-insensitive, buscar la versión en minúsculas
        let query_lower = query.to_lowercase();
        let line_lower = line.to_lowercase();
        if line_lower.contains(&query_lower) {
            // Encontrar la posición exacta en el texto original
            let start = line_lower.find(&query_lower).unwrap();
            let end = start + query.len();
            let before = &line[..start];
            let matched = &line[start..end];
            let after = &line[end..];
            format!("{}{}{}", before, matched.red().bold(), after)
        } else {
            line.to_string()
        }
    }
}



#[cfg(test)]
mod tests;
