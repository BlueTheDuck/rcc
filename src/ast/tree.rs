use std::fmt::Display;

use crate::lexer::token::{Ident, Literal};

#[derive(Debug, PartialEq, Eq)]
pub struct FuncDecl<'i> {
    pub ret: Ident<'i>,
    pub name: Ident<'i>,
    pub body: Vec<Statement<'i>>,
}
impl<'i> Display for FuncDecl<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}() {{\n", self.ret, self.name)?;
        for stmt in self.body.iter() {
            write!(f, "{stmt} \n")?;
        }
        write!(f, "}}\n")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarDecl<'i> {
    pub ty: Ident<'i>,
    pub name: Ident<'i>,
    pub value: Option<Literal>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement<'i> {
    FuncDecl(FuncDecl<'i>),
    VarDecl(VarDecl<'i>),
}

impl<'i> Statement<'i> {
    pub const fn new_func_decl(ret: Ident<'i>, name: Ident<'i>, body: Vec<Statement<'i>>) -> Self {
        Self::FuncDecl(FuncDecl {
            ret,
            name,
            body,
        })
    }
    pub const fn new_var_decl(ty: Ident<'i>, name: Ident<'i>, value: Option<Literal>) -> Self {
        Self::VarDecl(VarDecl {
            ty,
            name,
            value,
        })
    }
}

impl<'i> Display for Statement<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::FuncDecl(func) => write!(f, "{func}"),
            Statement::VarDecl(var) => {
                write!(f, "{ty} {name}", ty = var.ty, name = var.name)?;
                if let Some(value) = var.value {
                    write!(f, " = {value}")?;
                }
                write!(f, ";")?;
                Ok(())
            },
            
        }
    }
}
