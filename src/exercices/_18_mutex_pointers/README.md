# Mutex & Arc

En Rust, Mutex et Arc sont des types essentiels pour la programmation concurrente sécurisée.

## Mutex<T>

**Un mutex** (exclusion mutuelle) protège les données partagées en garantissant qu'un seul thread à la fois peut y accéder. Les principales caractéristiques:

- Fournit un accès mutuellement exclusif à la donnée
- Les threads doivent verrouiller le mutex avant d'accéder à la donnée
- Déverrouille automatiquement quand le verrou sort de portée
- Respecte le système d'ownership de Rust

# Arc<T>

**Arc** (Atomic Reference Counting) est un pointeur intelligent permettant le partage de propriété entre plusieurs threads:

- Permet à plusieurs threads de posséder la même donnée
- Compte atomiquement les références pour une sécurité thread
- Libère la mémoire quand le dernier Arc est détruit

Ces deux types sont souvent combinés pour partager des données mutables entre threads:

```rust
let donnees_partagées = Arc::new(Mutex::new(valeur));
```

Cette combinaison permet un partage thread-safe des données mutables tout en prévenant les conditions de course à la compilation.

- https://itsallaboutthebit.com/arc-mutex/
- https://blog.guillaume-gomez.fr/Rust/3/7
