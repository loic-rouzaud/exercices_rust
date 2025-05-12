# Exercices de programmation Rust

Ce dépôt contient une série d'exercices pour apprendre et pratiquer différents concepts en Rust. Les exercices couvrent des sujets allant des boucles et des collections aux smart pointers et à la gestion d'erreurs.

## Structure du projet

```
src/
├── exercices/           # Code des exercices (à compléter par les élèves)
├── solutions/           # Implémentations de référence
├── main.rs              # Point d'entrée pour les exercices
└── resume_exercices.rs  # Fonctions d'exercices
```

## Comment compiler et exécuter

### Pour les étudiants

Pour travailler sur les exercices, utilisez la commande suivante :

```bash
cargo run --bin exercises
```

Cette commande compilera et exécutera les exercices sans les solutions.

### Pour les solutions

Pour exécuter les solutions de référence, utilisez :

```bash
cargo run --bin solutions --features="solutions"
```

Cette commande active la fonctionnalité "solutions" et utilise le fichier main.rs situé dans le répertoire des solutions.
