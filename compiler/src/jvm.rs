use parser::ast::{Statement, Expression, OperationType};
use std::collections::HashMap;

pub fn compile(statements: Vec<Statement>) -> String {
    let mut compiler = Compiler::new();
    compiler.compile(statements)
}

struct Compiler {
    locals: i32,
    stack: i32,
    lines: Vec<String>,
    variables: HashMap<String, i32>
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            locals: 1,
            stack: 0,
            lines: vec![],
            variables: HashMap::new()
        }
    }

    fn compile(&mut self, statements: Vec<Statement>) -> String {
        self.compile_statements(statements);
        let code = self.lines.join("\n");
        let code = vec![
            ".class  public Main",
            ".super  java/lang/Object",
            ".method public <init>()V",
            "aload_0",
            "invokespecial java/lang/Object/<init>()V",
            "return",
            ".end method",
            ".method public static main([Ljava/lang/String;)V",
            ".limit stack 1000",
            ".limit locals 1000",
            code.as_str(),
            "return",
            ".end method",
        ].join("\n");
        code
    }

    fn compile_statements(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.lines.push(String::from("getstatic  java/lang/System/out Ljava/io/PrintStream;"));
            match statement {
                Statement::Assignment(name, expression) => {
                    self.locals += 1;
                    self.variables.insert(name, self.locals);
                    self.compile_expression(expression);
                    self.lines.push(format!("istore {}", self.locals));
                }
                Statement::Expression(expression) => {
                    self.lines.push(String::from("getstatic  java/lang/System/out Ljava/io/PrintStream;"));
                    self.compile_expression(expression);
                    self.lines.push(String::from("invokevirtual  java/io/PrintStream/println(I)V"))
                }
            };
        }
    }

    fn compile_expression(&mut self, expression: Box<Expression>) {
        match *expression {
            Expression::Number(n) => self.lines.push(format!("bipush {}", n)),
            Expression::Identifier(name) => {
                let variable_index = self.variables.get(name.as_str()).unwrap();
                self.lines.push(format!("iload {}", variable_index))
            }
            Expression::Operation(left, operation_type, right) => {
                self.compile_expression(left);
                self.compile_expression(right);
                let instruction = Compiler::get_instruction(operation_type);
                self.lines.push(String::from(instruction));
            }
        }
    }

    fn get_instruction(operation_type: OperationType) -> &'static str {
        match operation_type {
            OperationType::Mul => "imul",
            OperationType::Div => "idiv",
            OperationType::Add => "iadd",
            OperationType::Sub => "isub",
        }
    }
}
