use std::collections::HashMap;
use crate::parser::ast::OperationType::{Addition, Division, Multiplication, Subtraction};
use crate::parser::ast::{Expression, OperationType, Statement};
use crate::parser::ast::Expression::{Literal, Operation, Variable};
use crate::compiler::code::Code;

pub fn compile(statements: Vec<Statement>) -> String {
    let mut compiler = Compiler::new();
    compiler.compile(statements)
}

struct CompilerResult {
    code: Code,
    identifier: String
}

struct Compiler {
    next_variable_index: i32,
    variables: HashMap<String, String>,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            next_variable_index: 1,
            variables: HashMap::new(),
        }
    }

    fn compile(&mut self, statements: Vec<Statement>) -> String {
        let mut main_code = self.compile_statements(statements);
        let mut code = Code::new();
        code.add_lines(&[
            "declare void @printInt(i32)",
            "define i32 @main() {",
        ]);
        code.append(&mut main_code);
        code.add_lines(&[
            "ret i32 0",
            "}"
        ]);
        code.to_string()
    }

    fn compile_statements(&mut self, statements: Vec<Statement>) -> Code {
        let mut code = Code::new();
        for statement in statements {
            match statement {
                Statement::Expression(expression) => {
                    let mut expression = self.compile_expression(expression);
                    code.append(&mut expression.code);
                    let line = format!("call void @printInt(i32 {})", expression.identifier);
                    code.add_line(&line);
                }
                Statement::Assignment(name, expression) => {
                    let mut expression = self.compile_expression(expression);
                    code.append(&mut expression.code);
                    self.variables.insert(name, expression.identifier);
                }
            }
        }
        code
    }

    fn compile_expression(&mut self, expression: Box<Expression>) -> CompilerResult {
        match *expression {
            Literal(value) =>
                CompilerResult { code: Code::new(), identifier: value.to_string() },
            Variable(name) => {
                let identifier = self.variables.get(&*name)
                    .expect(format!("Unknown variable: {}", name).as_str())
                    .to_string();
                CompilerResult { code: Code::new(), identifier }
            },
            Operation(left_expression, operation_type, right_expression) =>
                self.compile_operation(left_expression, operation_type, right_expression)
        }
    }

    fn compile_operation(
        &mut self,
        left: Box<Expression>,
        operation_type: OperationType,
        right: Box<Expression>,
    ) -> CompilerResult {
        let mut left = self.compile_expression(left);
        let mut right = self.compile_expression(right);
        let mut code = Code::merge(&mut left.code, &mut right.code);
        let instruction = self.compile_operation_type(operation_type);
        let identifier = self.next_variable_identifier();
        let line = format!(
            "{} = {} i32 {}, {}",
            identifier,
            instruction,
            left.identifier,
            right.identifier
        );
        code.add_line(&line);
        CompilerResult { code, identifier }
    }

    fn next_variable_identifier(&mut self) -> String {
        let identifier = format!("%i{}", self.next_variable_index);
        self.next_variable_index += 1;
        identifier
    }

    fn compile_operation_type(&self, operation_type: OperationType) -> &'static str {
        match operation_type {
            Multiplication => "mul",
            Division => "sdiv",
            Addition => "add",
            Subtraction => "sub",
        }
    }
}
