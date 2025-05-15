# Lifetimes

En Rust, les lifetimes (durées de vie) sont des annotations qui permettent au compilateur de vérifier que les références restent valides pendant la durée nécessaire. Elles font partie du système de borrowing et sont représentées par des apostrophes.

Les lifetimes permettent de spécifier combien de temps une référence sera valide et d'établir des relations entre les durées de vie de différentes références. Le compilateur utilise ces annotations pour prévenir les références pendantes (dangling references).

```rust
// 'a est un paramètre de lifetime
fn fonction<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Code garantissant que la référence retournée
    // vit au moins aussi longtemps que x et y
}
```

Les lifetimes sont souvent implicites grâce à l'élision des lifetimes, mais doivent parfois être explicitement annotées, notamment:

- Dans les structures contenant des références
- Pour les fonctions avec plusieurs paramètres de référence
- Quand les règles d'élision ne suffisent pas

Les lifetimes sont une partie essentielle du modèle de sécurité mémoire de Rust.

- https://blog.guillaume-gomez.fr/Rust/2/6
