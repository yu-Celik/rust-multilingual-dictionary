use crate::saveable::Saveable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VocabularyManager {
    pub words: HashMap<String, HashMap<String, String>>,
}

impl VocabularyManager {
    pub fn new() -> Self {
        VocabularyManager {
            words: HashMap::new(),
        }
    }
    pub fn add_word(&mut self, english: String, translations: HashMap<String, String>) {
        self.words.insert(english, translations);
    }
    pub fn get_translation(&self, english: &str, language: &str) -> Option<&String> {
        self.words.get(english)?.get(language)
    }
    pub fn update_translation(
        &mut self,
        english: &str,
        language: &str,
        new_translation: String,
    ) -> bool {
        if let Some(translations) = self.words.get_mut(english) {
            translations.insert(language.to_string(), new_translation);
            true
        } else {
            false
        }
    }
    pub fn count_words_per_language(&self) -> HashMap<String, usize> {
        let mut count = HashMap::new();
        for translations in self.words.values() {
            for language in translations.keys() {
                count
                    .entry(language.to_string())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
        count
    }
    pub fn find_word(&self, english: &str) -> bool {
        self.words.contains_key(english)
    }
}

impl Display for VocabularyManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (english, translations) in &self.words {
            write!(f, "{}: ", english)?;
            let mut iter = translations.iter();
            if let Some((language, translation)) = iter.next() {
                write!(f, "{}: {}", language, translation)?;
            }
            for (language, translation) in iter {
                write!(f, ", {}: {}", language, translation)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Saveable for VocabularyManager {
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let vocabulary: VocabularyManager = serde_json::from_reader(reader)?;
        Ok(vocabulary)
    }
}
