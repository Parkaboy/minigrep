use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Language {
    Spanish,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Spanish
    }
}

impl Language {
    pub fn from_env() -> Self {
        let lang = env::var("LANG")
            .or_else(|_| env::var("LANGUAGE"))
            .or_else(|_| env::var("LC_ALL"))
            .unwrap_or_else(|_| "es".to_string());
        
        if lang.starts_with("en") {
            Language::English
        } else {
            Language::Spanish
        }
    }
}

pub struct I18n {
    language: Language,
    messages: HashMap<String, HashMap<Language, String>>,
}

impl I18n {
    pub fn new() -> Self {
        let mut i18n = I18n {
            language: Language::from_env(),
            messages: HashMap::new(),
        };
        i18n.init_messages();
        i18n
    }

    pub fn new_with_language(language: Language) -> Self {
        let mut i18n = I18n {
            language,
            messages: HashMap::new(),
        };
        i18n.init_messages();
        i18n
    }

    fn init_messages(&mut self) {
        // Mensajes de resultados
        self.add_message("no_results", 
            Language::Spanish, "No se encontraron coincidencias.",
            Language::English, "No matches found."
        );
        
        self.add_message("results_count", 
            Language::Spanish, " coincidencias encontradas:",
            Language::English, " matches found:"
        );
        
        // Mensajes de error
        self.add_message("argument_error", 
            Language::Spanish, "Error al parsear argumentos",
            Language::English, "Error parsing arguments"
        );
        
        self.add_message("application_error", 
            Language::Spanish, "Error de aplicación",
            Language::English, "Application error"
        );
        
        self.add_message("file_read_error", 
            Language::Spanish, "Error de archivo",
            Language::English, "File error"
        );
        
        self.add_message("file_read_message", 
            Language::Spanish, "No se pudo leer el archivo",
            Language::English, "Could not read file"
        );
        
        self.add_message("search_error", 
            Language::Spanish, "Error de búsqueda",
            Language::English, "Search error"
        );
        
        self.add_message("search_error_message", 
            Language::Spanish, "Error al buscar",
            Language::English, "Error searching for"
        );
    }

    fn add_message(&mut self, key: &str, lang1: Language, msg1: &str, lang2: Language, msg2: &str) {
        let mut translations = HashMap::new();
        translations.insert(lang1, msg1.to_string());
        translations.insert(lang2, msg2.to_string());
        self.messages.insert(key.to_string(), translations);
    }

    pub fn get_message(&self, key: &str) -> &str {
        self.messages
            .get(key)
            .and_then(|translations| translations.get(&self.language))
            .map(|s| s.as_str())
            .unwrap_or("Message not found")
    }

    pub fn get_language(&self) -> &Language {
        &self.language
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}
