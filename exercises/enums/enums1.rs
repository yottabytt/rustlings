// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

/*
https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
*/

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
