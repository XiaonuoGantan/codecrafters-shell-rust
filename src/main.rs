#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;

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
        "type" => {
            if ["exit", "echo", "type"].contains(&words[1]) {
                println!("{} is a shell builtin", words[1])
            } else {
                if !check_path(words[1]) {
                    println!("{} not found", words[1])
                }
            }
        }
        _ => println!("{}: command not found", input),
    }
}

fn check_path(cmd: &str) -> bool {
    match (env::var("PATH"), cmd) {
        (Ok(paths), cmd) => {
            for path in paths.split(':') {
                let path = format!("{}/{}", path, cmd);
                if std::path::Path::new(&path).exists() {
                    println!("{} is in {}", cmd, path);
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}
