# Boucles For et While

En Rust, les boucles permettent d'exécuter un bloc de code plusieurs fois. Les deux principales structures sont for et while.

La boucle for en Rust est principalement utilisée pour itérer sur des collections ou des plages de valeurs.

## Boucle for

```rust
for i in 0..5 {
    // Exécuté pour i = 0, 1, 2, 3, 4
}
```

```rust
for élément in collection {
    // Accède à chaque élément
}
```


## Boucle while

La boucle while exécute un bloc de code tant qu'une condition est vraie.

```rust
while condition {
    // Code exécuté tant que la condition est vraie
}
```

Rust propose également loop pour créer une boucle infinie qu'on peut quitter avec break, et continue pour passer à l'itération suivante.

```rust
loop {
    // Boucle infinie
    if condition {
        break; // Sortie de la boucle
    }
    if autre_condition {
        continue; // Passe à l'itération suivante
    }
}
```
