use std::io::{self, Write};

pub fn get_input(s1: String) -> String {
    print!("{}", s1);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}