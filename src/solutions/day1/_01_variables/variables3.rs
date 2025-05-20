fn main() {
    let prenom = "Loïc";
    let nom = "Rouzaud";
    let nom_complet = format!("{} {}", prenom, nom);
    println!("Longueur: {}, Contenu: {}", nom_complet.len(), nom_complet);
}
