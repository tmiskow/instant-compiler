use parser::ast::{Expression, OperationType, Statement};
use parser::ast::Expression::{Number, Operation};
use std::collections::HashMap;

pub fn compile(statements: Vec<Statement>) -> String {
    let mut compiler = Compiler::new();
    compiler.compile(statements)
}

struct Compiler {
    next_variable_index: i32,
    lines: Vec<String>,
    variables: HashMap<String, String>,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            next_variable_index: 1,
            lines: vec![],
            variables: HashMap::new(),
        }
    }

    fn compile(&mut self, statements: Vec<Statement>) -> String {
        self.compile_statements(statements);
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

    fn compile_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::Expression(expression) => {
                    let identifier = self.compile_expression(expression);
                    let line = format!("call void @printInt(i32 {})", identifier);
                    self.lines.push(line);
                }
                Statement::Assignment(name, expression) => {
                    let identifier = self.compile_expression(expression);
                    self.variables.insert(name, identifier);
                }
            }
        }
    }

    fn compile_expression(&mut self, expression: Box<Expression>) -> String {
        match *expression {
            Number(n) => n.to_string(),
            Expression::Identifier(name) => self.variables.get(&*name).unwrap().to_string(),
            Operation(left_expression, operation_type, right_expression) =>
                self.compile_operation(left_expression, operation_type, right_expression)
        }
    }

    fn compile_operation(
        &mut self,
        left_expression: Box<Expression>,
        operation_type: OperationType,
        right_expression: Box<Expression>,
    ) -> String {
        let left_identifier = self.compile_expression(left_expression);
        let right_identifier = self.compile_expression(right_expression);
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
