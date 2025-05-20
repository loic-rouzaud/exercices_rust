// Les structures contiennent des données, mais peuvent aussi avoir de la logique.
// Dans cet exercice, nous avons défini la structure `Product` et nous voulons
// tester une logique qui lui est attachée.

#[derive(Debug)]
struct Product {
    name: String,
    price: u32, // prix en centimes
    in_stock: bool,
}

impl Product {
    fn new(name: String, price: u32, in_stock: bool) -> Self {
        if price == 0 {
            // Ce n'est pas la façon dont vous devriez gérer les erreurs en Rust,
            // mais nous apprendrons la gestion des erreurs plus tard.
            panic!("Le prix d'un produit ne peut pas être zéro");
        }
        Self {
            name,
            price,
            in_stock,
        }
    }

    // TODO(): Ajoutez le type de retour correct à la signature de la fonction.
    fn is_available(&self) {
        // TODO(): Lisez les tests qui utilisent cette méthode pour déterminer
        // quand un produit est considéré comme disponible.
    }

    // TODO(): Ajoutez le type de retour correct à la signature de la fonction.
    fn get_price_with_tax(&self, tax_percentage: u32) {
        // TODO(): Calculez le prix du produit avec la taxe.
        // Le prix avec taxe est le prix original + (prix original * pourcentage de taxe / 100)
    }
}

fn main() {
    // Vous pouvez expérimenter ici
    let product = Product::new(String::from("Ordinateur portable"), 120000, true);
    println!("Produit: {:?}", product);
}
