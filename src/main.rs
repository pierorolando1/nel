mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    let lxer = lexer::Lexer::new("let five = 5;".to_owned());
    let prser = parser::Parser::new(lxer);
    let program = prser.parse_program();

    println!("Hello, neli!");
}
