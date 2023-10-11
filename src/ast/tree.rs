use std::fmt::Display;

use crate::lexer::token::Ident;

mod expr;
pub mod control;
pub use expr::Expression;

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
    pub value: Option<Expression<'i>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Typedef<'i> {
    pub ty: Vec<Ident<'i>>,
    pub name: Ident<'i>,
}
impl<'i> Typedef<'i> {
    pub fn new(ty: Vec<Ident<'i>>, name: Ident<'i>) -> Self {
        Self { ty, name }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement<'i> {
    FuncDecl(FuncDecl<'i>),
    VarDecl(VarDecl<'i>),
    Typedef(Typedef<'i>),
    If(control::If<'i>),
}
impl<'i> Statement<'i> {
    pub const fn new_func_decl(ret: Ident<'i>, name: Ident<'i>, body: Vec<Statement<'i>>) -> Self {
        Self::FuncDecl(FuncDecl { ret, name, body })
    }
    pub const fn new_var_decl(ty: Ident<'i>, name: Ident<'i>, value: Option<Expression<'i>>) -> Self {
        Self::VarDecl(VarDecl { ty, name, value })
    }
    pub const fn new_typedef(ty: Vec<Ident<'i>>, name: Ident<'i>) -> Self {
        Self::Typedef(Typedef { ty, name })
    }

    #[must_use]
    pub fn as_typedef(&self) -> Option<&Typedef<'i>> {
        if let Self::Typedef(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl<'i> Display for Statement<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::FuncDecl(func) => write!(f, "{func}"),
            Statement::VarDecl(var) => {
                write!(f, "{ty} {name}", ty = var.ty, name = var.name)?;
                if let Some(ref value) = var.value {
                    write!(f, " = {value}")?;
                }
                write!(f, ";")?;
                Ok(())
            }
            Statement::Typedef(typedef) => {
                write!(f, "typedef ")?;
                for ty in typedef.ty.iter() {
                    write!(f, "{} ", ty)?;
                }
                write!(f, "{};", typedef.name)?;
                Ok(())
            }
            Statement::If(r#if) => {
                writeln!(f, "if ({cond}) {{", cond = r#if.condition)?;
                for stmt in r#if.body.iter() {
                    writeln!(f, "{stmt} ")?;
                }
                writeln!(f, "}}")?;
                Ok(())
            },
            
        }
    }
}
