#![allow(clippy::ptr_arg)]

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // Pass by reference to avoid ownership move

    string_uppercase(data); // Move ownership since we modify the string
}
