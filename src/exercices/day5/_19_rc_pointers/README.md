# Rc Pointers

En Rust, **Rc<T>** (Reference Counted) est un pointeur intelligent qui permet à plusieurs propriétaires de partager la propriété d'une même donnée. Il maintient un compteur du nombre de références qui pointent vers les données.

Caractéristiques principales:

- Permet le partage de propriété dans un même thread
- Incrémente le compteur à chaque clone et le décrémente quand un Rc sort de portée
- Libère la mémoire uniquement quand le dernier Rc est détruit
- N'est pas thread-safe (utiliser Arc pour le multi-threading)

```rust
use std::rc::Rc;

let donnée = Rc::new(42);
let référence1 = Rc::clone(&donnée); // Incrémente le compteur
let référence2 = donnée.clone(); // Même chose que la ligne précédente
```

**Rc** est utile pour:

- Structures de données avec références partagées (graphes)
- Caches avec des données partagées
- Implémentation du pattern observateur
- Éviter la duplication des données volumineuses

À combiner avec RefCell pour la mutabilité intérieure dans un contexte mono-thread.

- https://blog.guillaume-gomez.fr/Rust/3/6
- https://doc.rust-lang.org/std/rc/struct.Rc.html
