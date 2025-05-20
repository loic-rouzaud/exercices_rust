# Trait

En Rust, un trait définit un ensemble de fonctionnalités qu'un type peut implémenter. Les traits sont similaires aux interfaces dans d'autres langages et constituent le fondement du polymorphisme en Rust.
Un trait déclare des signatures de méthodes que les types implémentant ce trait doivent fournir. Les traits peuvent également inclure des méthodes par défaut.

```rust
trait Affichable {
    fn afficher(&self) -> String;

    fn description(&self) -> String {
        format!("Objet affichable: {}", self.afficher())
    }
}
```

## Les traits permettent:

Le polymorphisme statique (génériques contraint par trait)
Le polymorphisme dynamique (trait objects)
D'étendre les types existants avec de nouvelles fonctionnalités
De définir des comportements partagés entre différents types

Les traits sont fondamentaux dans l'écosystème Rust et sont utilisés abondamment dans la bibliothèque standard pour des fonctionnalités comme **Copy**, **Clone**, **Debug**, et **Iterator**.

- https://doc.rust-lang.org/rust-by-example/trait.html
- https://doc.rust-lang.org/book/ch10-02-traits.html
- https://blog.guillaume-gomez.fr/Rust/2/2
