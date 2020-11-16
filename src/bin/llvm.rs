use instant_compiler::parser;
use instant_compiler::compiler::llvm;

pub fn main() {
    let ast = parser::parse_input();
    let code = llvm::compile(ast);
    println!("{}", code);
}
