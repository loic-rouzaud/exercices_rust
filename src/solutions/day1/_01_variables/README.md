# Variables

En Rust, les variables sont des emplacements mémoire nommés qui stockent des valeurs. Par défaut, elles sont immuables (non modifiables). Pour créer une variable modifiable, on utilise le mot-clé mut.
Rust est un langage à typage statique fort, mais offre l'inférence de types, permettant souvent d'omettre les annotations de type. Le langage supporte également le "shadowing", permettant de redéclarer une variable avec le même nom.

```rust
let x = 5;    // Variable immuable
let mut y = 10;   // Variable mutable
let x = x + 1;    // Shadowing (x vaut maintenant 6)
```

- https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
- https://blog.guillaume-gomez.fr/Rust/1/4
- https://doc.rust-lang.org/book/ch03-02-data-types.html
