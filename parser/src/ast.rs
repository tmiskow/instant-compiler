#[derive(Debug)]
pub enum Expression {
    Number(i32),
    Operation(Box<Expression>, OperationType, Box<Expression>),
    Error,
}

#[derive(Debug, Copy, Clone)]
pub enum OperationType {
    Mul,
    Div,
    Add,
    Sub,
}

