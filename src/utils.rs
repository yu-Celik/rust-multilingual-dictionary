use std::io;
use crate::{exit_program, vocabulary_manager::VocabularyManager};

pub fn is_valid_word(word: &str) -> bool {
    !word.trim().is_empty() && word.chars().all(|c| c.is_alphabetic() || c.is_whitespace())
}

pub fn first_letter_uppercase(word: &str) -> String {
    let word_to_lowercase = word.to_lowercase();
    let mut chars = word_to_lowercase.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn supported_languages() -> &'static [&'static str] {
    &["Français", "Arabe"]
}

pub fn is_supported_language(language: &str) -> bool {
    supported_languages().contains(&language)
}

pub fn read_input(prompt: &str, vocabulary: &VocabularyManager, filename: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Échec de la lecture");

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("6") {
            exit_program(vocabulary, filename);
        }

        if is_valid_word(trimmed) {
            return first_letter_uppercase(trimmed);
        } else {
            println!("L'entrée ne peut pas être vide et ne doit contenir que des lettres et des espaces.");
            println!("Merci de réessayer ou entrez '6' pour annuler.");
        }
    }
}