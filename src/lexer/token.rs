use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ident<'i> {
    name: &'i str,
}
impl<'i> Ident<'i> {
    #[must_use]
    pub(crate) const fn new(name: &'i str) -> Self {
        Self { name }
    }
}
impl<'i> Display for Ident<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Literal {
    Int(i64),
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token<'i> {
    Ident(Ident<'i>),
    Literal(Literal),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Assign,
    SemiColon,
    Eof,
}
impl<'i> Token<'i> {
    #[must_use]
    pub fn as_ident(&self) -> Option<&Ident<'i>> {
        if let Self::Ident(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_literal(&self) -> Option<&Literal> {
        if let Self::Literal(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
impl<'i> Display for Token<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident) => write!(f, "{}", ident.name),
            Token::Literal(literal) => write!(f, "{}", literal),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::Assign => write!(f, "="),
            Token::SemiColon => write!(f, ";"),
            Token::Eof => write!(f, ""),
        }
    }
}
