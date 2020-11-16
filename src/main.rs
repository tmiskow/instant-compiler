use instant_compiler::parser::parse_input;

pub fn main() {
    let statements = parse_input();
    println!("{:?}", statements);
}
