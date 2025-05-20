# Iterators

En Rust, les itérateurs sont des objets qui permettent de parcourir séquentiellement des collections. Ils implémentent le trait Iterator et sont paresseux (lazy) - ne calculant les valeurs que lorsqu'elles sont demandées.

Les méthodes principales des itérateurs incluent:

- map(): Transforme chaque élément en appliquant une fonction
- filter(): Conserve uniquement les éléments satisfaisant un prédicat
- collect(): Rassemble les éléments dans une nouvelle collection
- fold(): Accumule une valeur en traitant chaque élément
- enumerate(): Associe un index à chaque élément
- zip(): Combine deux itérateurs en paires d'éléments
- skip() et take(): Contrôlent combien d'éléments traiter

Ces méthodes peuvent être enchaînées pour créer des pipelines de traitement de données expressifs et efficaces.

```rust
let nombres = vec![1, 2, 3];
let doublés = nombres.iter().map(|x| x * 2);
```

Les itérateurs permettent d'écrire du code concis et expressif tout en maintenant des performances comparables aux boucles traditionnelles grâce aux optimisations du compilateur.

- https://doc.rust-lang.org/std/iter/
- https://blog.guillaume-gomez.fr/Rust/2/15
