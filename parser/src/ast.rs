#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Assignment(String, Box<Expression>)
}

#[derive(Debug)]
pub enum Expression {
    Literal(i32),
    Variable(String),
    Operation(Box<Expression>, OperationType, Box<Expression>)
}

#[derive(Debug, Copy, Clone)]
pub enum OperationType {
    Multiplication,
    Division,
    Addition,
    Subtraction
}

