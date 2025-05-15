# RefCell

En Rust, **RefCell<T>** est un conteneur qui fournit une mutabilité intérieure contrôlée à l'exécution plutôt qu'à la compilation. Il applique les règles de borrowing de Rust dynamiquement (à l'exécution) plutôt que statiquement (à la compilation).

Caractéristiques principales:

- Permet de modifier des données même quand elles sont référencées par des références immuables
- Vérifie les règles de borrowing à l'exécution, peut paniquer si violées
- Fonctionne uniquement en contexte mono-thread (non thread-safe)
- Utilise borrow() pour obtenir une référence immuable
- Utilise borrow_mut() pour obtenir une référence mutable

```rust
use std::cell::RefCell;

let cellule = RefCell::new(5);

// Modifie la valeur à travers une référence immuable
*cellule.borrow_mut() += 1;

// Accède à la valeur
println!("{}", *cellule.borrow());
```

RefCell est utile pour:

- Simuler la mutabilité dans des interfaces immuables
- Modifier des structures de données complexes avec références partagées
- Implémenter des modèles comme l'observateur ou memoization

Souvent combiné avec Rc pour des structures de données qui nécessitent à la fois propriété partagée et mutabilité intérieure.

- https://doc.rust-lang.org/std/cell/struct.RefCell.html
- https://blog.guillaume-gomez.fr/Rust/3/6
