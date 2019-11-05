#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Assignment(String, Box<Expression>)
}

#[derive(Debug)]
pub enum Expression {
    Number(i32),
    Identifier(String),
    Operation(Box<Expression>, OperationType, Box<Expression>)
}

#[derive(Debug, Copy, Clone)]
pub enum OperationType {
    Mul,
    Div,
    Add,
    Sub
}

