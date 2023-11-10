use crate::{preprocessor::SpanType, span::Span};

use super::token::{Ident, Keyword, Literal, Token, TokenKind};

fn parse_token<'i>(span: &Span<'i, SpanType>) -> TokenKind<'i> {
    match span.get() {
        "typedef" => TokenKind::Keyword(Keyword::Typedef),
        "if" => TokenKind::Keyword(Keyword::If),
        "else" => TokenKind::Keyword(Keyword::Else),

        ident if span.extra.is_identifier() => TokenKind::Ident(Ident::new(ident)),
        literal if span.extra.is_literal_num() => {
            TokenKind::Literal(Literal::Int(literal.parse().unwrap()))
        }

        "(" => TokenKind::OpenParen,
        ")" => TokenKind::CloseParen,
        "{" => TokenKind::OpenBrace,
        "}" => TokenKind::CloseBrace,
        "==" => TokenKind::Equals,
        "=" => TokenKind::Assign,
        ";" => TokenKind::SemiColon,
        "," => TokenKind::Comma,
        "*" => TokenKind::Star,

        _ if span.extra.is_eof() => TokenKind::Eof,

        _ => unreachable!("{span} of type '{:?}'", span.extra),
    }
}

pub struct TokenIter<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    iter: I,
}

impl<'i, I> Iterator for TokenIter<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|s| !s.extra.is_whitespace())
            .map(|span| Token::new(parse_token(&span), span.with(())))
    }
}

pub fn parse_tokens<'i, I>(source: I) -> TokenIter<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    TokenIter { iter: source }
}
