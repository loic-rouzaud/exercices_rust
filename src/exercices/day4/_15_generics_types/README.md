# Generic Types

En Rust, les types génériques (ou génériques) permettent d'écrire du code flexible qui fonctionne avec différents types de données. Ils favorisent la réutilisation du code tout en conservant la sécurité et l'efficacité du typage statique.
Les génériques peuvent être utilisés dans les fonctions, les structures, les énumérations et les implémentations de traits. Ils sont définis avec des paramètres de type entre crochets angulaires <T>.

```rust
// Fonction générique
fn fonction<T>(valeur: T) -> T {
    valeur
}

// Structure générique
struct Paire<T, U> {
    premier: T,
    second: U,
}
```

Les génériques peuvent être contraints par des traits pour garantir que les types utilisés disposent de certaines fonctionnalités:

```rust
fn affiche<T: Display>(valeur: T) {
    println!("{}", valeur);
}
```

Contrairement à d'autres langages, le code générique en Rust est monomorphisé à la compilation, générant du code optimisé pour chaque type concret utilisé, sans coût d'exécution.

- https://doc.rust-lang.org/book/ch10-01-syntax.html
- https://blog.guillaume-gomez.fr/Rust/2/4
