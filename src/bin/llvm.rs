use instant_compiler::parse_input;
use compiler::llvm;

pub fn main() {
    let ast = parse_input();
    let code = llvm::compile(ast);
    println!("{}", code);
}