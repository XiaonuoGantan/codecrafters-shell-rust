use std::env;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        run(input.trim());
        input.clear();
    }
}

fn run(input: &str) {
    let words: Vec<&str> = input.split_whitespace().collect();
    match words.as_slice() {
        ["exit", code] => std::process::exit(code.parse().unwrap_or(0)),
        ["echo", args @ ..] => println!("{}", args.join(" ")),
        ["type", cmd] if ["exit", "echo", "type"].contains(cmd) => println!("{} is a shell builtin", cmd),
        ["type", cmd] => {
            if !check_path(cmd) {
                println!("{} not found", cmd);
            }
        }
        [cmd, args @ ..] => {
            if !run_external(cmd, args) {
                println!("{}: command not found", input);
            }
        }
        _ => {}
    }
}

fn run_external(cmd: &str, args: &[&str]) -> bool {
    get_cmd_path(cmd).map_or(false, |path| _run_external_cmd(&path, args))
}

fn _run_external_cmd(path: &str, args: &[&str]) -> bool {
    match std::process::Command::new(path).args(args).output() {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            true
        }
        Err(_) => false,
    }
}

fn check_path(cmd: &str) -> bool {
    get_cmd_path(cmd).map_or(false, |path| {
        println!("{} is in {}", cmd, path);
        true
    })
}

fn get_cmd_path(cmd: &str) -> Option<String> {
    if cmd.starts_with('/') {
        Some(cmd.to_string())
    } else {
        env::var("PATH").ok().and_then(|paths| {
            paths.split(':').find_map(|path| {
                let full_path = format!("{}/{}", path, cmd);
                if std::path::Path::new(&full_path).exists() {
                    Some(full_path)
                } else {
                    None
                }
            })
        })
    }
}
