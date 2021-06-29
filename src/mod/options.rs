#[path = "./input_value.rs"]
mod input;

pub fn type_of_play() -> u8 {
    println!("Types of play");
    println!("-----------------");
    println!("1. Singleplayer(beta)");
    println!("2. Multiplayer");
    println!("3. Exit");

    input::input_value(&String::from("Select type of play [1/2]: "))
        .trim()
        .parse::<u8>()
        .expect("Wrong number format!")
}
