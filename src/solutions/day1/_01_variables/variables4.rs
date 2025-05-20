fn main() {
    let nombre: i32 = 42;

    let nombre_f32: f32 = nombre as f32;

    let texte_nombre = nombre.to_string();

    println!("nombre: {} (i32)", nombre);
    println!("nombre_f32: {} (f32)", nombre_f32);
    println!("texte_nombre: {} (String)", texte_nombre);
}
