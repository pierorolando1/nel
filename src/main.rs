use clap::StructOpt;
use repl::start_rpl;

use crate::{parser::Parser, repl::Cli};

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    let lexer = lexer::Lexer::new("".to_owned());
    let parser = Parser::new(lexer);
    let program = parser.parse_program();

    let args = Cli::parse();

    match args.file {
        Some(expr) => {
            println!("File ");
        }
        None => {
            start_rpl();
        }
    }
}
