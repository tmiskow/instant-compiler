use std::env;
use instant_compiler::parser;
use instant_compiler::compiler::jvm;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let main_class = "Main".to_string();
    let main_class = args.get(1).unwrap_or(&main_class);
    let ast = parser::parse_input();
    let code = jvm::compile(ast, main_class);
    println!("{}", code);
}
