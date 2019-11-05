use parser::grammar;
use compiler::llvm;

pub fn main() {
    use std::io;

    let parser = grammar::ProgramParser::new();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let expressions = parser.parse(&line).unwrap();
    let code = llvm::compile(expressions);
    println!("{}", code);
}

#[cfg(test)]
mod tests {
    use parser::grammar;

    #[test]
    fn parse() {
        let parser = grammar::ProgramParser::new();
        let actual = parser.parse("22 * 44 + 66").unwrap();
        let expected = "[Operation(Operation(Number(22), Mul, Number(44)), Add, Number(66))]";
        assert_eq!(&format!("{:?}", actual), expected);
    }
}
