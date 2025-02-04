#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Resize { width: u64, height: u64 }, // Struct variant with named fields
    Move(Point),                         // Tuple variant with a `Point`
    Echo(String),                        // Tuple variant with a `String`
    ChangeColor(u8, u8, u8),             // Tuple variant with three `u8` values (RGB)
    Quit,                                // Unit variant
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}