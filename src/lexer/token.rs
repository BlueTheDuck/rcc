use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Ident<'i> {
    name: &'i str,
}
impl<'i> Ident<'i> {
    #[must_use]
    pub(crate) fn new(name: &'i str) -> Self {
        Self { name }
    }
}
impl<'i> Display for Ident<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token<'i> {
    Ident(Ident<'i>),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
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

    /// Returns `true` if the token is [`Ident`].
    ///
    /// [`Ident`]: Token::Ident
    #[must_use]
    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(..))
    }
}
impl<'i> From<Ident<'i>> for Token<'i> {
    fn from(v: Ident<'i>) -> Self {
        Self::Ident(v)
    }
}
impl<'i> Display for Token<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident) => write!(f, "{}", ident.name),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
        }
    }
}
