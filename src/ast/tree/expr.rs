use std::fmt::Display;

use crate::lexer::token::{Ident, Literal};

#[derive(Debug, PartialEq, Eq)]
pub enum Expression<'i> {
    Equals {
        lhs: Box<Expression<'i>>,
        rhs: Box<Expression<'i>>,
    },
    Literal(Literal),
    Ident(Ident<'i>),
}

impl<'i> Display for Expression<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Equals { lhs, rhs } => write!(f, "{lhs} == {rhs}"),
            Expression::Literal(lit) => write!(f, "{lit}"),
            Expression::Ident(ident) => write!(f, "{ident}"),
        }
    }
}
