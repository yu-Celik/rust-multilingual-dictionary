use std::collections::HashMap;
use crate::saveable::Saveable;
use crate::vocabulary_manager::VocabularyManager;
use crate::utils::{first_letter_uppercase, is_valid_word, is_supported_language};

#[test]
fn test_add_word() {
    let mut vm = VocabularyManager::new();
    let mut translations = HashMap::new();
    translations.insert("français".to_string(), "bonjour".to_string());
    translations.insert("arabe".to_string(), "مرحبا".to_string());
    vm.add_word("hello".to_string(), translations);
    assert_eq!(vm.words.len(), 1);
    assert_eq!(
        vm.words.get("hello").unwrap().get("français").unwrap(),
        "bonjour"
    );
    assert_eq!(
        vm.words.get("hello").unwrap().get("arabe").unwrap(),
        "مرحبا"
    );
}

#[test]
fn test_get_translation() {
    let mut vm = VocabularyManager::new();
    let mut translations = HashMap::new();
    translations.insert("français".to_string(), "bonjour".to_string());
    translations.insert("arabe".to_string(), "مرحبا".to_string());
    vm.add_word("hello".to_string(), translations);
    assert_eq!(
        vm.get_translation("hello", "français"),
        Some(&"bonjour".to_string())
    );
    assert_eq!(
        vm.get_translation("hello", "arabe"),
        Some(&"مرحبا".to_string())
    );
    assert_eq!(vm.get_translation("goodbye", "français"), None);
    assert_eq!(vm.get_translation("hello", "espagnol"), None);
}

#[test]
fn test_update_translation() {
    let mut vm = VocabularyManager::new();
    let mut translations = HashMap::new();
    translations.insert("français".to_string(), "bonjour".to_string());
    translations.insert("arabe".to_string(), "مرحبا".to_string());
    vm.add_word("hello".to_string(), translations);
    assert!(vm.update_translation("hello", "français", "salut".to_string()));
    assert_eq!(
        vm.get_translation("hello", "français"),
        Some(&"salut".to_string())
    );
    assert!(!vm.update_translation("goodbye", "français", "au revoir".to_string()));
    assert!(vm.update_translation("hello", "arabe", "صباح الخير".to_string()));
    assert_eq!(
        vm.get_translation("hello", "arabe"),
        Some(&"صباح الخير".to_string())
    );
}

#[test]
fn test_count_words_per_language() {
    let mut vm = VocabularyManager::new();
    let mut translations1 = HashMap::new();
    translations1.insert("français".to_string(), "bonjour".to_string());
    translations1.insert("arabe".to_string(), "مرحبا".to_string());
    vm.add_word("hello".to_string(), translations1);

    let mut translations2 = HashMap::new();
    translations2.insert("français".to_string(), "au revoir".to_string());
    translations2.insert("arabe".to_string(), "إلى اللقاء".to_string());
    vm.add_word("goodbye".to_string(), translations2);

    let count = vm.count_words_per_language();
    assert_eq!(count.get("français"), Some(&2));
    assert_eq!(count.get("arabe"), Some(&2));
}

#[test]
fn test_display() {
    let mut vm = VocabularyManager::new();
    let mut translations = HashMap::new();
    translations.insert("français".to_string(), "bonjour".to_string());
    translations.insert("arabe".to_string(), "مرحبا".to_string());
    vm.add_word("hello".to_string(), translations);

    let display = format!("{}", vm);
    assert!(display.contains("hello:"));
    assert!(display.contains("français: bonjour"));
    assert!(display.contains("arabe: مرحبا"));
}

#[test]
fn test_save_and_load() {
    let mut vm = VocabularyManager::new();
    let mut translations = HashMap::new();
    translations.insert("français".to_string(), "bonjour".to_string());
    vm.add_word("hello".to_string(), translations);

    let filename = "test_vocabulary.json";
    vm.save_to_file(filename).unwrap();

    let loaded_vm = VocabularyManager::load_from_file(filename).unwrap();
    assert_eq!(loaded_vm.words, vm.words);

    // Nettoyage
    std::fs::remove_file(filename).unwrap();
}

#[test]
fn test_arabic_input() {
    let mut vm = VocabularyManager::new();
    let mut translations = HashMap::new();
    translations.insert("arabe".to_string(), "مرحبا".to_string());
    vm.add_word("hello".to_string(), translations);
    assert_eq!(
        vm.get_translation("hello", "arabe"),
        Some(&"مرحبا".to_string())
    );
}

#[test]
fn test_french_input() {
    let mut vm = VocabularyManager::new();
    let mut translations = HashMap::new();
    translations.insert("français".to_string(), "bonjour".to_string());
    vm.add_word("hello".to_string(), translations);
    assert_eq!(
        vm.get_translation("hello", "français"),
        Some(&"bonjour".to_string())
    );
}

#[test]
fn test_invalid_input() {
    let mut vm = VocabularyManager::new();
    assert!(!vm.find_word("hello!"));
    assert!(!vm.update_translation("hello!", "français", "bonjour".to_string()));
    assert_eq!(vm.get_translation("hello!", "français"), None);
}

#[test]
fn test_invalid_language() {
    let mut vm = VocabularyManager::new();
    assert!(!vm.find_word("hello"));
    assert!(!vm.update_translation("hello", "invalid", "bonjour".to_string()));
    assert_eq!(vm.get_translation("hello", "invalid"), None);
}

#[test]
fn test_first_letter_uppercase() {
    assert_eq!(first_letter_uppercase("hello"), "Hello");
    assert_eq!(first_letter_uppercase("HELLO"), "Hello");
    assert_eq!(first_letter_uppercase("hELLO"), "Hello");
}

#[test]
fn test_is_valid_word() {
    assert!(is_valid_word("hello"));
    assert!(!is_valid_word("hello!"));
    assert!(!is_valid_word("123"));
}

#[test]
fn test_is_supported_language() {
    assert!(is_supported_language("Français"));
    assert!(is_supported_language("Arabe"));
    assert!(!is_supported_language("Invalid"));
}
