use std::fmt::Display;

use super::{Expression, Statement};

#[derive(Debug, PartialEq, Eq)]
pub struct If<'i> {
    pub condition: Expression<'i>,
    pub body: Vec<Statement<'i>>,
    pub else_body: Option<Vec<Expression<'i>>>,
}
impl<'i> Display for If<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if({}) {{", self.condition)?;
        for stmt in self.body.iter() {
            write!(f, "{stmt}")?;
        }
        write!(f, "}}")?;
        if let Some(ref else_body) = self.else_body {
            write!(f, " else {{")?;
            for stmt in else_body.iter() {
                write!(f, "{stmt}")?;
            }
            write!(f, "}}")?;
        }
        writeln!(f)?;
        Ok(())
    }
}
