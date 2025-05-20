# Pattern Matching

En Rust, le pattern matching est un mécanisme puissant permettant de décomposer et d'analyser des valeurs selon leur structure. Il est principalement utilisé avec l'expression match qui compare une valeur à une série de patterns.

Le pattern matching peut extraire des valeurs de structures complexes, vérifier des conditions, et garantir l'exhaustivité des cas traités.
Les patterns peuvent inclure des littéraux, des variables, des wildcards (_), des déstructurations de tuples/structs, et des garde-conditions avec if.

```rust
match valeur {
    Pattern1 => expression1,
    Pattern2 if condition => expression2,
    _ => expression_par_défaut,
}
```

Le pattern matching est également disponible dans les expressions if let et while let pour traiter des cas uniques sans nécessiter l'exhaustivité des match.

- https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html
- https://blog.guillaume-gomez.fr/Rust/1/5
