use crate::Value;

#[derive(Clone, Debug)]
pub enum Expression {
    List(Vec<Expression>),
    Literal(Value),
    Name(String),
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub name: String,
    pub value: Expression,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(Assignment),
}
