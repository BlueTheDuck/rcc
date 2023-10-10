use std::fmt::Display;

use crate::lexer::token::Ident;

pub struct FuncDecl<'i> {
    pub ret: Ident<'i>,
    pub name: Ident<'i>,
    pub body: Option<Vec<Statement<'i>>>,
}
impl<'i> Display for FuncDecl<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}() {{ ", self.ret, self.name)?;
        for stmt in self.body.iter().flatten() {
            write!(f, "{stmt} ")?;
        }
        write!(f, "}}")
    }
}

pub enum Statement<'i> {
    FuncDecl(FuncDecl<'i>),
}

impl<'i> Display for Statement<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::FuncDecl(func) => write!(f, "{func}"),
        }
    }
}
