use parser::ast::{Expression, OperationType};
use parser::ast::Expression::{Number, Operation};

pub fn compile(expressions: Vec<Box<Expression>>) -> String {
    let mut compiler = Compiler::new();
    compiler.compile(expressions)
}

struct Compiler {
    next_variable_index: i32,
    lines: Vec<String>
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            next_variable_index: 1,
            lines: vec![]
        }
    }

    fn compile(&mut self, expressions: Vec<Box<Expression>>) -> String {
        self.compile_expressions(expressions);
        let code = self.lines.join("\n");
        let code = vec![
            "declare void @printInt(i32)",
            "define i32 @main() {",
            code.as_str(),
            "ret i32 0",
            "}"
        ].join("\n");
        code
    }

    fn compile_expressions(&mut self, expressions: Vec<Box<Expression>>) {
        for expression in expressions {
            let identifier = self.compile_expression(expression);
            let line = format!("call void @printInt(i32 {})", identifier);
            self.lines.push(line);
        }
    }

    fn compile_expression(&mut self, expression: Box<Expression>) -> String {
        match *expression {
            Number(n) => n.to_string(),
            Operation(left, operation_type, right) => {
                let left_identifier = self.compile_expression(left);
                let right_identifier = self.compile_expression(right);
                let instruction = Compiler::get_instruction(operation_type);
                let identifier = self.next_variable_identifier();
                let line = format!(
                    "{} = {} i32 {}, {}",
                    identifier,
                    instruction,
                    left_identifier,
                    right_identifier
                );
                self.lines.push(line);
                identifier
            }
        }
    }

    fn next_variable_identifier(&mut self) -> String {
        let identifier = format!("%i{}", self.next_variable_index);
        self.next_variable_index += 1;
        identifier
    }

    fn get_instruction(operation_type: OperationType) -> &'static str {
        match operation_type {
            OperationType::Mul => "mul",
            OperationType::Div => "sdiv",
            OperationType::Add => "add",
            OperationType::Sub => "sub",
        }
    }
}
