# HashMap

En Rust, **HashMap<K, V>** est une collection qui stocke des paires clé-valeur avec un accès rapide par clé. Elle est implémentée comme une table de hachage et fait partie de la bibliothèque standard (std::collections).
Les principales caractéristiques:

Stocke des paires clé-valeur sans ordre particulier
Les clés doivent implémenter les traits Eq et Hash
L'accès, l'insertion et la suppression se font en temps constant (O(1)) en moyenne
Redimensionnement automatique quand nécessaire

Principales opérations:

Création: **HashMap::new()**
Insertion: **insert(clé, valeur)**
Accès: **get(&clé)** retourne **Option<&V>**
Vérification: **contains_key(&clé)**
Suppression: **remove(&clé)**
Itération: **for (clé, valeur) in &map**

Les HashMaps sont utiles pour créer des dictionnaires, des caches, et toute structure nécessitant un accès rapide par identifiant.

- https://doc.rust-lang.org/std/collections/struct.HashMap.html
