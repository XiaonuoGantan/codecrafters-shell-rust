#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    check_existence(&input.trim());
}

/** Check if the input cmd exists in the predefined list of commands of the shell */
fn check_existence(input: &str) -> () {
    println!("{}: command not found", input)
}
