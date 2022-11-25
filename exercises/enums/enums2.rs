// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: u8, y: u8 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting!"),
            Message::Move { x, y } => println!(
                "Moving horizontally {} steps and vertically {} steps.",
                x, y
            ),
            Message::Echo(s) => println!("'{}'", s),
            _ => println!("{:?}", &self),
        };
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
