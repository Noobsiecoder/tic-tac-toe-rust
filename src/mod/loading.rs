use std::io::{self, Write};

#[path = "./sleep.rs"]
mod sleep;

pub fn loading_animation() {
    let msgs = vec![".", "..", "...", "...."];
    for msg in msgs.iter() {
        print!("Loading {}\r", msg);
        io::stdout().flush().unwrap();
        sleep::set_timeout(200);
    }
}
