# Struct

En Rust, une struct (structure) est un type de données personnalisé qui permet de regrouper plusieurs valeurs de types différents dans une seule unité logique. Les structs aident à organiser les données selon leur signification.

## Il existe trois formes de structs:

**Structs classiques** (nommées): avec champs nommés
**Structs tuples**: avec champs indexés par position
**Structs unitaires**: sans champs (utiles pour les traits)

```rust
// Struct classique
struct Personne {
    nom: String,
    age: u32,
}

// Struct tuple
struct Point(i32, i32);

// Struct unitaire
struct Unité;
```

Les structs peuvent implémenter des méthodes et des fonctions associées via les blocs impl. Elles sont au cœur de la programmation orientée objet en Rust, permettant d'encapsuler données et comportements.

- https://blog.guillaume-gomez.fr/Rust/1/10
- https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
