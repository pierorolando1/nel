use clap::Parser;
use colored::Colorize;
use std::io::{stdin, stdout, Write};

#[derive(Parser)]
pub struct Cli {
    pub file: Option<std::path::PathBuf>,
}

fn read_line(placeholder: &str) -> String {
    let mut s = String::new();
    print!("{}", format!("{} ", placeholder).blue());
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return s;
}

pub fn start_rpl() {
    let mut scanned = vec![""];

    loop {
        let input = read_line("> ");
        println!("You typed: {}", input);

        if input == "exit" {
            break;
        }
    }
}
