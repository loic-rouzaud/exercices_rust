# Function Pointers & Closures

En Rust, les fonctions peuvent être passées comme valeurs via différents types de traits pour closures et pointeurs de fonction:

## Fn, FnMut, FnOnce

Ce sont trois traits que les closures (fonctions anonymes) peuvent implémenter:

- FnOnce: Peut être appelée une seule fois et consomme les valeurs capturées
- FnMut: Peut modifier les valeurs capturées, peut être appelée plusieurs fois
- Fn: Ne modifie pas les valeurs capturées, peut être appelée plusieurs fois

```rust
fn appliquer<F>(f: F, x: i32) -> i32
where F: Fn(i32) -> i32
{
    f(x)
}
```

## Pointeurs de fonction (fn)

Le type fn représente un pointeur vers une fonction définie (non une closure). Il ne capture pas d'environnement.

```rust
fn ajouter_un(x: i32) -> i32 { x + 1 }

// Type fn(i32) -> i32
let pointeur_fonction: fn(i32) -> i32 = ajouter_un;
```

Ces traits permettent d'implémenter des modèles de conception fonctionnels comme les callbacks, les itérateurs et la programmation générique.

- https://blog.guillaume-gomez.fr/Rust/2/11
- https://doc.rust-lang.org/stable/std/ops/trait.FnOnce.html
- https://doc.rust-lang.org/std/ops/trait.FnMut.html
- https://doc.rust-lang.org/std/ops/trait.Fn.html
