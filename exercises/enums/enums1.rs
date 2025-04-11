// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move {x: i32, y: i32},
    ChangeColor(i32, i32, i32)
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("helloworld")));
    println!("{:?}", Message::Move{x: 2, y:43});
    println!("{:?}", Message::ChangeColor(2, 32, 1));
}
