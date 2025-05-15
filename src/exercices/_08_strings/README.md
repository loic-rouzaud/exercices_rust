# String vs &str

En Rust, il existe deux types principaux pour représenter des chaînes de caractères:

**String**: Type possédé, alloué sur le tas, modifiable et redimensionnable. C'est une structure qui possède ses données et les libère automatiquement quand elle sort de portée.
**&str**: Type référence (slice de chaîne), qui pointe vers une séquence UTF-8 stockée ailleurs. Il est immuable et ne possède pas les données. Les littéraux de chaîne ("texte") sont de type &'static str.

## Les principales différences:

**String** peut être modifiée et redimensionnée, &str est immuable
**String** possède ses données, &str emprunte des données
**String** a un coût en mémoire plus élevé que &str
**&str** est souvent utilisé pour les paramètres de fonction afin d'accepter à la fois String et &str

On peut convertir facilement entre les deux types avec & et to_string() ou String::from().

- https://blog.guillaume-gomez.fr/Rust/2/8
