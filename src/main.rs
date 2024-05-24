#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        run(&input.trim());
        input.clear();
    }
}

/** 
 * Try to run the command `input` if it's predefined, otherwise print an error message.
 */
fn run(input: &str) -> () {
    let words = input.split_whitespace().collect::<Vec<&str>>();
    match words[0] {
        "exit" => std::process::exit(words[1].parse().unwrap()),
        "echo" => println!("{}", words[1..].join(" ")),
        _ => println!("{}: command not found", input)
    }
}
