# Exercices de programmation Rust

Ce dépôt contient une série d'exercices pour apprendre et pratiquer différents concepts en Rust.

## Structure du projet

```
src/
├── exercices/           # Code des exercices (à compléter par les élèves)
├── solutions/           # Implémentations de référence
└── mains.rs             # Point d'entrée pour les exercices
```

Il y a des fonctions de test dans chaque fichier. Ces tests ont pour but de vous aiguiller, utilisez les !!

## Comment compiler et exécuter

### Pour les étudiants

Pour travailler sur les exercices, utilisez la commande suivante :

```bash
rustc filename
./filename
```

Pour tester

```bash
rustc filename --test
./filename
```

# Solutions

```rust
cargo run --bin solutions --features solutions
```
