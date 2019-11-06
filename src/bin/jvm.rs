use instant_compiler::parse_input;
use compiler::jvm;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let main_class = "Main".to_string();
    let main_class = args.get(1).unwrap_or(&main_class);
    let ast = parse_input();
    let code = jvm::compile(ast, main_class);
    println!("{}", code);
}
