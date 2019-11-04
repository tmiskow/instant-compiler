use parser::grammar;

pub fn main() {
    use std::io;
    let parser = grammar::ProgramParser::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let result = parser.parse(&line).unwrap();
        println!("{:?}", result);
    }
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
