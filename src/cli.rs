use std::io;
use std::io::Write;

use colored::ColoredString;

pub fn get_input(mut option: ColoredString) -> String {
    print!("{}", option);
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
