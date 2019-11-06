use std::io;
use std::io::Read;
use parser::grammar;

pub fn main() {
    let parser = grammar::ProgramParser::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read line");
    let expressions = parser.parse(&input).unwrap();
    println!("{:?}", expressions);
}
