use std::env;

fn main() {
    let compile_solutions =
        env::var("COMPILE_SOLUTIONS").unwrap_or_else(|_| "0".to_string()) == "1";

    if compile_solutions {
        println!("cargo:rustc-cfg=solutions");
    }
}
