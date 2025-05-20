# Functions

En Rust, les fonctions sont définies avec le mot-clé fn et permettent d'organiser le code en blocs réutilisables. Une fonction peut accepter des paramètres (avec types explicites obligatoires) et peut retourner une valeur.


```rust
fn function_name(paramètre1: Type1, paramètre2: Type2) -> TypeRetour {
    // Corps de la fonction
    return_value // Expression de retour (sans point-virgule)
}
```

Les fonctions en Rust suivent plusieurs principes importants:

La dernière expression sans point-virgule est implicitement retournée
Le mot-clé return peut être utilisé pour un retour anticipé
Le type de retour est spécifié après une flèche (->)
Les fonctions peuvent être passées comme arguments à d'autres fonctions

```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b  // Retourne a + b (pas de point-virgule)
}
```

- https://blog.guillaume-gomez.fr/Rust/1/6
- https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
