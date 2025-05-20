fn main() {
    let temperature: i32 = 22;
    let is_sunny: bool = true;

    if temperature >= 20 && temperature <= 30 && is_sunny {
        println!("perfect for a walk");
    } else if temperature >= 15 && temperature <= 25 && !is_sunny {
        println!("good for a walk");
    } else {
        println!("stay at home");
    }
}
