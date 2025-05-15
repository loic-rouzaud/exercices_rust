# Option et Result

En Rust, **Option** et **Result** sont des types énumérés (enums) qui gèrent respectivement l'absence de valeur et les erreurs potentielles, constituant la base de la gestion d'erreurs.

## Option<T>

Représente une valeur optionnelle: soit présente **(Some(T))**, soit absente **(None)**.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Utilisé quand une valeur peut être absente, évitant les problèmes de null.

## Result<T, E>

Représente soit un succès (Ok(T)), soit une erreur (Err(E)).

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Utilisé pour les opérations pouvant échouer, forçant la gestion explicite des erreurs.

Les deux types offrent des méthodes comme **unwrap()**, **expect()**, **map()**, et l'opérateur **?** qui simplifie la propagation d'erreurs. Ils encouragent une gestion d'erreurs sûre et expressive sans exceptions ni valeurs nulles.

- https://blog.guillaume-gomez.fr/Rust/1/12
- https://doc.rust-lang.org/std/option/
- https://doc.rust-lang.org/std/result/
