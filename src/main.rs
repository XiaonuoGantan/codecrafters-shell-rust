use std::env;
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
        "type" => {
            if ["exit", "echo", "type"].contains(&words[1]) {
                println!("{} is a shell builtin", words[1])
            } else {
                if !check_path(words[1]) {
                    println!("{} not found", words[1])
                }
            }
        }
        _ => {
            if !run_external(words) {
                println!("{}: command not found", input)
            }
        }
    }
}

fn run_external(words: Vec<&str>) -> bool {
    let cmd_path = get_cmd_path(words[0]);
    match cmd_path {
        Some(path) => _run_external_cmd(&path, words[1..].to_vec()),
        None => false,
    }
}

fn _run_external_cmd(path: &str, words: Vec<&str>) -> bool {
    let mut cmd = std::process::Command::new(path);
    let output = cmd.args(&words[1..]).output();
    match output {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            true
        }
        Err(_) => false,
    }
}

fn check_path(cmd: &str) -> bool {
    match get_cmd_path(cmd) {
        Some(path) => {
            println!("{} is in {}", cmd, path);
            true
        }
        None => false,
    }
}

fn get_cmd_path(cmd: &str) -> Option<String> {
    match env::var("PATH") {
        Ok(paths) => {
            for path in paths.split(':') {
                let path = format!("{}/{}", path, cmd);
                if std::path::Path::new(&path).exists() {
                    return Some(path);
                }
            }
            None
        }
        _ => None,
    }
}
