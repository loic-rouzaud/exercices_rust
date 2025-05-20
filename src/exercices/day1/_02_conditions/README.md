# Conditions

En Rust, les structures de contrôle conditionnel permettent d'exécuter différents blocs de code selon qu'une condition est vraie ou fausse. Rust utilise if, else if et else pour réaliser des branchements conditionnels.
Les expressions de condition ne nécessitent pas de parenthèses, mais les accolades sont obligatoires pour délimiter les blocs de code. Les conditions doivent être des expressions booléennes (type bool).

```rust
if condition {
    // Code exécuté si la condition est vraie
} else if autre_condition {
    // Code exécuté si la première condition est fausse
    // et la seconde est vraie
} else {
    // Code exécuté si toutes les conditions sont fausses
}
```

En Rust, les expressions conditionnelles peuvent également retourner une valeur, ce qui permet une syntaxe concise pour l'affectation conditionnelle.

```rust
rustlet valeur = if condition { 5 } else { 10 };
```

- https://blog.guillaume-gomez.fr/Rust/1/5
