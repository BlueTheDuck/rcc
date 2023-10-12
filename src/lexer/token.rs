#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Display)]
pub enum Keyword {
    #[display(fmt = "typedef")]
    Typedef,

    #[display(fmt = "if")]
    If,

    #[display(fmt = "else")]
    Else,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Display)]
pub struct Ident<'i> {
    name: &'i str,
}
impl<'i> Ident<'i> {
    #[must_use]
    pub(crate) const fn new(name: &'i str) -> Self {
        Self { name }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Display)]
pub enum Literal {
    Int(i64),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Display)]
pub enum Token<'i> {
    Keyword(Keyword),
    Ident(Ident<'i>),
    Literal(Literal),

    #[display(fmt = "(")]
    OpenParen,

    #[display(fmt = ")")]
    CloseParen,

    #[display(fmt = "{{")]
    OpenBrace,

    #[display(fmt = "}}")]
    CloseBrace,

    #[display(fmt = "==")]
    Equals,

    #[display(fmt = "=")]
    Assign,

    #[display(fmt = ";")]
    SemiColon,

    #[display(fmt = "$")]
    Eof,
}
impl<'i> Token<'i> {
    #[must_use]
    pub fn as_keyword(&self) -> Option<&Keyword> {
        if let Self::Keyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

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
