use super::{Expression, Statement};

#[derive(Debug, PartialEq, Eq)]
pub struct If<'i> {
    pub condition: Expression<'i>,
    pub body: Vec<Statement<'i>>,
    pub else_body: Option<Vec<Expression<'i>>>,
}
