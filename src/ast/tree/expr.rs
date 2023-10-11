use crate::lexer::token::{Ident, Literal};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
pub enum Expression<'i> {
    #[display(fmt = "{lhs} == {rhs}")]
    Equals {
        lhs: Box<Expression<'i>>,
        rhs: Box<Expression<'i>>,
    },
    Literal(Literal),
    Ident(Ident<'i>),
}
impl<'i> Expression<'i> {
    pub fn new_equals(expr: (Expression<'i>, Expression<'i>)) -> Self {
        Self::Equals {
            lhs: Box::new(expr.0),
            rhs: Box::new(expr.1),
        }
    }
}
