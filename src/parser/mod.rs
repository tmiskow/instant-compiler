pub mod ast;
mod grammar;

use std::io::Read;
use std::io;
use ast::Statement;

pub fn parse_input() -> Vec<Statement> {
    let parser = grammar::ProgramParser::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read line");
    parser.parse(&input).unwrap_or_else(|error| panic!(format!("{}", error)))
}
