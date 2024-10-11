mod vocabulary_manager;
mod saveable;
mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::io;
use vocabulary_manager::VocabularyManager;
use saveable::Saveable;
use utils::{read_input, supported_languages, is_supported_language};

fn exit_program(vocabulary: &VocabularyManager, filename: &str) {
    println!("Sauvegarde du dictionnaire...");
    if let Err(e) = vocabulary.save_to_file(filename) {
        println!("Erreur lors de la sauvegarde : {}", e);
    } else {
        println!("Dictionnaire sauvegardé avec succès.");
    }
    println!("Merci d'avoir utilisé le dictionnaire !");
    std::process::exit(0);
}

fn main() {
    let filename = "vocabulary.json";
    let mut vocabulary = VocabularyManager::load_from_file(filename).unwrap_or_else(|_| {
        println!("Aucun fichier de vocabulaire trouvé. Création d'un nouveau dictionnaire.");
        VocabularyManager::new()
    });

    println!("--------------------------------");
    println!("Bienvenue dans le dictionnaire !");
    println!("--------------------------------");
    println!("Remarque : vous pouvez quitter le programme en entrant '6' à n'importe quel moment.");
    println!("--------------------------------");
    println!("Commandes :");
    println!("--------------------------------");
    loop {
        println!("1 : Ajouter un mot dans le dictionnaire");
        println!("----------------");
        println!("2 : Obtenir une traduction");
        println!("----------------");
        println!("3 : Mettre à jour une traduction");
        println!("----------------");
        println!("4 : Compter les mots par langue");
        println!("----------------");
        println!("5 : Afficher tout le vocabulaire");
        println!("----------------");
        println!("6 : Quitter");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        };

        match choice {
            1 => {
                // Ajouter un mot
                let mut translations = HashMap::new();
                let english = read_input("Entrez le mot en anglais :", &vocabulary, filename);
                if vocabulary.find_word(&english) {
                    println!("Le mot existe déjà dans le dictionnaire.");
                    continue;
                }

                loop {
                    let language = read_input(&format!(
                        "Selectionnez la langue entre < {} > : ",
                        supported_languages().join(", ")
                    ), &vocabulary, filename);

                    if is_supported_language(&language) {
                        let word = read_input("Entrez la traduction :", &vocabulary, filename);
                        translations.insert(language, word);
                        vocabulary.add_word(english, translations);
                        println!("Mot ajouté avec succès");
                        break;
                    } else {
                        println!("Langue non supportée. Veuillez réessayer.");
                    }
                }
            }
            2 => {
                // Obtenir une traduction
                let english = read_input("Entrez le mot en anglais :", &vocabulary, filename);
                if !vocabulary.find_word(&english) {
                    println!("Le mot n'existe pas dans le dictionnaire.");
                    continue;
                }
                loop {
                    let language = read_input(&format!(
                        "Selectionnez la langue entre < {} > : ",
                        supported_languages().join(", ").to_lowercase()
                    ), &vocabulary, filename);

                    if is_supported_language(&language) {
                        match vocabulary.get_translation(&english, &language) {
                            Some(translation) => println!("Traduction : {}", translation),
                            None => println!("Traduction non trouvée"),
                        }
                        break;
                    } else {
                        println!("Langue non supportée. Veuillez réessayer.");
                    }
                }
            }
            3 => {
                // Mettre à jour une traduction
                let english = read_input("Entrez le mot en anglais :", &vocabulary, filename);
                if !vocabulary.find_word(&english) {
                    println!("Le mot n'existe pas dans le dictionnaire.");
                    continue;
                }

                loop {
                    let language = read_input(&format!(
                        "Selectionnez la langue entre < {} > : ",
                        supported_languages().join(", ")
                    ), &vocabulary, filename);

                    if is_supported_language(&language) {
                        let new_translation = read_input("Entrez la nouvelle traduction :", &vocabulary, filename);
                        if vocabulary.update_translation(&english, &language, new_translation) {
                            println!("Traduction mise à jour avec succès");
                        } else {
                            println!("Échec de la mise à jour de la traduction");
                        }
                        break;
                    } else {
                        println!("Langue non supportée. Veuillez réessayer.");
                    }
                }
            }
            4 => {
                // Compter les mots par langue
                let word_count = vocabulary.count_words_per_language();
                println!("Nombre de mots par langue : {:?}", word_count);
            }
            5 => {
                // Afficher tout le vocabulaire
                println!("Vocabulaire complet :");
                println!("{}", vocabulary);
            }
            6 => exit_program(&vocabulary, filename),
            _ => println!("Choix invalide"),
        }
    }
}
