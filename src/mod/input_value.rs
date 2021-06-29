use std::io::{self, Write};

pub fn input_value(input_message: &String) -> String {
    let mut input = String::new();
    print!("{}", input_message);

    io::stdout().flush().expect("Unable to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input, please try later");

    input.replace("\r\n", "")
}
