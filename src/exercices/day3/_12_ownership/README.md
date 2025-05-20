# Ownership

En Rust, l'ownership (propriété) est un concept fondamental qui régit la gestion de la mémoire sans ramasse-miettes ni déallocation manuelle. Ce système permet à Rust de garantir la sécurité mémoire à la compilation.

## Principes clés:

Chaque valeur a un unique propriétaire
Quand le propriétaire sort de portée, la valeur est automatiquement libérée
La propriété peut être transférée (move) lors d'affectations ou passages de fonctions

```rust
let s1 = String::from("hello"); // s1 est propriétaire
let s2 = s1;                    // propriété transférée à s2, s1 n'est plus utilisable
```

L'ownership est la base du système de types de Rust et permet d'éviter les problèmes comme:

Libération double (double free)
Utilisation après libération (use after free)
Fuites mémoire (memory leaks)

Ce mécanisme est complété par le borrowing (emprunt) qui permet d'accéder temporairement aux valeurs sans en prendre la propriété.

- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- https://blog.guillaume-gomez.fr/Rust/2/5
