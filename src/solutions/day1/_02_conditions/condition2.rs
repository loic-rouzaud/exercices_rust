fn main() {
    let note = 14.5;

    if note >= 16.0 {
        println!("Excellent");
    } else if note >= 14.0 {
        println!("Good");
    } else if note >= 12.0 {
        println!("Ok");
    } else if note >= 10.0 {
        println!("Passable");
    } else {
        println!("Fail");
    }
}
