// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
    Alpha,
    Beta,
    Gamma,
    Delta,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

//SOLution