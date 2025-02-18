fn main() {
    // Explicitly annotate the type as `Vec<i32>`
    let mut numbers: Vec<i32> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into()); // `u8` -> `i32`
    let n2: i8 = -1;
    numbers.push(n2.into()); // `i8` -> `i32`

    println!("{numbers:?}");
}
