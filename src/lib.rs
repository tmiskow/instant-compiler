use parser::ast::Statement;
use parser::grammar;
use std::io::Read;
use std::io;

pub fn parse_input() -> Vec<Statement> {
    let parser = grammar::ProgramParser::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read line");
    parser.parse(&input).unwrap()
}
