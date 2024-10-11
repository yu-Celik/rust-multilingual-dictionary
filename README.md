# Rust Multilingual Dictionary

## Description
Rust Multilingual Dictionary est une application de gestion de vocabulaire multilingue développée en Rust. Elle permet aux utilisateurs de stocker, récupérer et gérer des traductions de mots dans différentes langues.

## Fonctionnalités
- Ajout de nouveaux mots avec leurs traductions
- Récupération de traductions
- Mise à jour des traductions existantes
- Comptage des mots par langue
- Affichage de tout le vocabulaire
- Sauvegarde et chargement du dictionnaire depuis un fichier JSON

## Prérequis
- Rust (édition 2021 ou ultérieure)
- Cargo (généralement installé avec Rust)

## Installation
1. Clonez ce dépôt :
   ```
   git clone https://github.com/yu-Celik/rust-multilingual-dictionary.git
   ```
2. Naviguez dans le répertoire du projet :
   ```
   cd rust-multilingual-dictionary
   ```
3. Compilez le projet :
   ```
   cargo build --release
   ```

## Utilisation
Exécutez le programme avec :
   ```
   cargo run --release
   ```

Suivez les instructions à l'écran pour interagir avec le dictionnaire.

## Structure du Projet
- `src/main.rs` : Point d'entrée du programme
- `src/vocabulary_manager.rs` : Implémentation du gestionnaire de vocabulaire
- `src/saveable.rs` : Trait pour la sauvegarde et le chargement
- `src/utils.rs` : Fonctions utilitaires
- `src/tests.rs` : Tests unitaires

## Tests
Pour exécuter les tests :
   ```
   cargo test
   ```

## Contribution
Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à soumettre une pull request.

## Licence
Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## Auteur
CELIK

## Remerciements
- Merci à la communauté Rust pour ses excellents outils et sa documentation.


