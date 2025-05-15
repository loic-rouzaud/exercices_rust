# Box

En Rust, Box<T> est un type de pointeur intelligent (smart pointer) qui stocke des données sur le tas (heap) plutôt que sur la pile (stack). Il s'agit du moyen le plus simple d'allouer des données sur le tas.

## Principales caractéristiques:

- Possède ses données (ownership)
- Taille connue à la compilation
- Libère automatiquement la mémoire quand il sort de portée
- Représente une unique instance d'un type

```rust
let b = Box::new(5); // Alloue un entier sur le tas
```

## Les cas d'utilisation courants de Box incluent:

- Stocker des types dont la taille est inconnue à la compilation
- Créer des structures de données récursives (comme les arbres et listes chaînées)
- Créer des trait objects pour le polymorphisme dynamique
- Transférer la propriété d'une grande quantité de données sans copie

Box est le plus simple des smart pointers en Rust, avec une surcharge minimale par rapport aux références brutes.

- https://doc.rust-lang.org/book/ch15-01-box.html
- https://blog.guillaume-gomez.fr/Rust/2/14
