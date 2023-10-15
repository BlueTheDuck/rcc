use std::fmt::Display;

use crate::lexer::token::Ident;

pub mod control;
mod expr;
pub use expr::Expression;

#[derive(Debug, PartialEq, Eq, derive_more::From, derive_more::Display)]
pub enum Declarator<'i> {
    Ident(Ident<'i>),

    #[display(fmt = "*{}", _0)]
    Pointer(Box<Declarator<'i>>),
    /*
    #[display(fmt = "{}[{}]", _0, _1)]
    Array(Box<Declarator<'i>>, usize),

    Function(Box<Declarator<'i>>, Vec<Ident<'i>>),
    */
}

#[derive(Debug, PartialEq, Eq)]
pub struct FuncDecl<'i> {
    pub ret: Ident<'i>,
    pub name: Ident<'i>,
    pub args: Vec<(Ident<'i>, Declarator<'i>)>,
    pub body: Vec<Statement<'i>>,
}
impl<'i> Display for FuncDecl<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}(", self.ret, self.name)?;
        for (ty, name) in self.args.iter() {
            write!(f, "{ty} {name}, ", ty = ty, name = name)?;
        }
        writeln!(f, ") {{")?;
        for stmt in self.body.iter() {
            writeln!(f, "{stmt} ")?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarDecl<'i> {
    pub ty: Ident<'i>,
    pub name: Declarator<'i>,
    pub value: Option<Expression<'i>>,
}
impl<'i> Display for VarDecl<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{ty} {name}", ty = self.ty, name = self.name)?;
        if let Some(ref value) = self.value {
            write!(f, " = {value}")?;
        }
        writeln!(f, ";")?;
        Ok(())
    }
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
impl<'i> Display for Typedef<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "typedef ")?;
        for ty in self.ty.iter() {
            write!(f, "{ty} ")?;
        }
        writeln!(f, "{};", self.name)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[display(fmt = "{lhs} = {rhs};")]
pub struct Assignment<'i> {
    lhs: Ident<'i>,
    rhs: Expression<'i>,
}

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
pub enum Statement<'i> {
    FuncDecl(FuncDecl<'i>),
    VarDecl(VarDecl<'i>),
    Typedef(Typedef<'i>),
    If(control::If<'i>),
    Assign(Assignment<'i>),
}
impl<'i> Statement<'i> {
    pub const fn new_func_decl(
        ret: Ident<'i>,
        name: Ident<'i>,
        args: Vec<(Ident<'i>, Declarator<'i>)>,
        body: Vec<Statement<'i>>,
    ) -> Self {
        Self::FuncDecl(FuncDecl {
            ret,
            name,
            args,
            body,
        })
    }
    pub const fn new_var_decl(
        ty: Ident<'i>,
        name: Declarator<'i>,
        value: Option<Expression<'i>>,
    ) -> Self {
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
