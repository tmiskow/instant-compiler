use std::collections::HashMap;
use std::cmp::max;
use crate::parser::ast::OperationType::{Addition, Division, Multiplication, Subtraction};
use crate::parser::ast::{Statement, Expression, OperationType};
use crate::compiler::code::Code;

pub fn compile(statements: Vec<Statement>, main_class: &str) -> String {
    let mut compiler = Compiler::new();
    compiler.compile_program(statements, main_class)
}

struct CompilerResult {
    code: Code,
    stack_size: usize,
}

struct Compiler { variables: HashMap<String, usize> }

impl Compiler {
    fn new() -> Compiler {
        Compiler { variables: HashMap::new() }
    }

    fn locals(&self) -> usize {
        self.variables.len() + 1
    }

    fn compile_program(&mut self, statements: Vec<Statement>, main_class: &str) -> String {
        let mut main = self.compile_statements(statements);
        let mut code = Code::new();
        code.add_lines(&[
            &format!(".class public {}", main_class),
            ".super  java/lang/Object",
            ".method public <init>()V",
            "aload_0",
            "invokespecial java/lang/Object/<init>()V",
            "return",
            ".end method",
            ".method public static main([Ljava/lang/String;)V",
            &format!(".limit stack {}", main.stack_size),
            &format!(".limit locals {}", self.locals()),
        ]);
        code.append(&mut main.code);
        code.add_lines(&[
            "return",
            ".end method",
        ]);
        code.to_string()
    }

    fn compile_statements(&mut self, statements: Vec<Statement>) -> CompilerResult {
        let mut stack_size = 0;
        let mut code = Code::new();
        for statement in statements {
            let mut statement = self.compile_statement(statement);
            code.append(&mut statement.code);
            stack_size = max(stack_size, statement.stack_size);
        }
        CompilerResult { code, stack_size }
    }

    fn compile_statement(&mut self, statement: Statement) -> CompilerResult {
        match statement {
            Statement::Assignment(name, expression) => {
                let CompilerResult { mut code, stack_size } = self.compile_expression(expression);
                let variable_index = match self.variables.get(&name) {
                    Some(index) => *index,
                    None => {
                        let new_index = self.locals();
                        self.variables.insert(name.to_string(), new_index);
                        new_index
                    }
                };
                code.add_line(&format!("istore {}", variable_index));
                CompilerResult { code, stack_size: stack_size + 1 }
            }
            Statement::Expression(expression) => {
                let expression: Box<Expression> = expression;
                let mut expression = self.compile_expression(expression);
                let mut code = Code::new();
                code.add_line("getstatic java/lang/System/out Ljava/io/PrintStream;");
                code.append(&mut expression.code);
                code.add_line("invokevirtual java/io/PrintStream/println(I)V");
                CompilerResult { code, stack_size: expression.stack_size + 1 }
            }
        }
    }

    fn compile_expression(&mut self, expression: Box<Expression>) -> CompilerResult {
        match *expression {
            Expression::Literal(value) => self.compile_literal_expression(value),
            Expression::Variable(name) => self.compile_variable_expression(&name),
            Expression::Operation(left, operation_type, right) =>
                self.compile_operation_expression(left, right, operation_type)
        }
    }

    fn compile_literal_expression(&self, value: i32) -> CompilerResult {
        let instruction = match value {
            0..=5 => format!("iconst_{}", value),
            6..=127 => format!("bipush {}", value),
            128..=32767 => format!("sipush {}", value),
            _ => format!("ldc {}", value),
        };
        CompilerResult { code: Code::from_line(&instruction), stack_size: 1 }
    }

    fn compile_variable_expression(&mut self, variable_name: &str) -> CompilerResult {
        let variable_index = *self.variables.get(variable_name)
            .expect((format!("Unknown variable: {}", variable_name)).as_str());
        let instruction = match variable_index {
            0..=3 => format!("iload_{}", variable_index),
            _ => format!("iload {}", variable_index)
        };
        CompilerResult { code: Code::from_line(&instruction), stack_size: 1 }
    }

    fn compile_operation_expression(
        &mut self,
        left: Box<Expression>,
        right: Box<Expression>,
        operation_type: OperationType,
    ) -> CompilerResult {
        let left = self.compile_expression(left);
        let right = self.compile_expression(right);
        let should_reverse = right.stack_size > left.stack_size;
        let (first, second) = if should_reverse { (right, left) } else { (left, right) };
        let CompilerResult { mut code, stack_size } = self.compile_operands(first, second);
        if let Subtraction | Division = operation_type {
            if should_reverse {
                code.add_line("swap");
            }
        }
        let mut operation = self.compile_operation_type(operation_type);
        CompilerResult {
            code: Code::merge(&mut code, &mut operation.code),
            stack_size: stack_size + operation.stack_size
        }
    }

    fn compile_operands(
        &mut self,
        mut first: CompilerResult,
        mut second: CompilerResult
    ) -> CompilerResult {
        let mut stack_size = max(first.stack_size, second.stack_size);
        if first.stack_size == second.stack_size {
            stack_size += 1;
        }
        let code = Code::merge(&mut first.code, &mut second.code);
        CompilerResult { code, stack_size }
    }

    fn compile_operation_type(&self, operation_type: OperationType) -> CompilerResult {
        let instruction = match operation_type {
            Multiplication => "imul",
            Division => "idiv",
            Addition => "iadd",
            Subtraction => "isub",
        }.to_string();
        CompilerResult { code: Code::from_line(&instruction), stack_size: 0 }
    }
}
