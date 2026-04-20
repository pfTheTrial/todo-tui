use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod pt_br;
pub mod en;
pub mod es;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Language {
    PtBr,
    En,
    Es,
}

impl Default for Language {
    fn default() -> Self {
        Language::PtBr
    }
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![Language::PtBr, Language::En, Language::Es]
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Language::PtBr => "Português",
            Language::En => "English",
            Language::Es => "Español",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::PtBr => "PT-BR",
            Language::En => "EN",
            Language::Es => "ES",
        }
    }
}

pub fn detect_system_language() -> Language {
    if let Some(locale) = sys_locale::get_locale() {
        let locale = locale.to_lowercase();
        if locale.starts_with("pt") {
            Language::PtBr
        } else if locale.starts_with("es") {
            Language::Es
        } else {
            Language::En
        }
    } else {
        Language::En // Fallback seguro
    }
}

pub struct I18n {
    strings: HashMap<&'static str, &'static str>,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        let strings = match lang {
            Language::PtBr => pt_br::get_strings(),
            Language::En => en::get_strings(),
            Language::Es => es::get_strings(),
        };
        Self { strings }
    }

    pub fn t<'a>(&self, key: &'a str) -> &'a str {
        self.strings.get(key).copied().unwrap_or(key)
    }
}
