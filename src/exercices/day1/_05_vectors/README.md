# Vecteurs

En Rust, un vecteur (Vec<T>) est une collection redimensionnable qui stocke des éléments de même type. Il est alloué sur le tas et peut grandir ou rétrécir pendant l'exécution.
Les vecteurs permettent d'ajouter, supprimer et accéder aux éléments efficacement. Ils peuvent être créés vides avec Vec::new() ou pré-remplis avec la macro vec![].

```rust
let mut v = Vec::new();  // Vecteur vide
let v = vec![1, 2, 3];   // Vecteur avec valeurs initiales
```
Les opérations courantes incluent l'ajout avec **push()**, l'accès par index avec [] ou **get()**, et l'itération sur les éléments. Les vecteurs sont automatiquement libérés lorsqu'ils sortent de portée.

- https://doc.rust-lang.org/std/vec/struct.Vec.html
