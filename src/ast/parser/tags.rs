use nom::{bytes::complete::take, combinator::verify, IResult};

use crate::lexer::{
    stream::TokenStream,
    token::{Keyword, TokenKind},
};

macro_rules! def_tag {
    ($v:vis $name:ident => $value:expr) => {
        $v fn $name<'i, 't>(i: TokenStream<'i, 't>) -> IResult<TokenStream<'i, 't>, TokenStream<'i, 't>> {
            verify(take(1usize), |t: &TokenStream| t[0] == $value)(i)
        }
    };
}

def_tag!(pub(crate) open_paren => TokenKind::OpenParen);
def_tag!(pub(crate) close_paren => TokenKind::CloseParen);
def_tag!(pub(crate) open_brace => TokenKind::OpenBrace);
def_tag!(pub(crate) close_brace => TokenKind::CloseBrace);
def_tag!(pub(crate) equals => TokenKind::Equals);
def_tag!(pub(crate) assign => TokenKind::Assign);
def_tag!(pub(crate) semi_colon => TokenKind::SemiColon);
def_tag!(pub(crate) comma => TokenKind::Comma);
def_tag!(pub(crate) star => TokenKind::Star);

pub(crate) fn eof<'i, 't>(
    input: TokenStream<'i, 't>,
) -> IResult<TokenStream<'i, 't>, TokenStream<'i, 't>> {
    verify(take(1usize), |t: &TokenStream| t[0] == TokenKind::Eof)(input)
}

pub(crate) fn keyword<'i: 't, 't>(
    keyword: Keyword,
) -> impl FnMut(
    TokenStream<'i, 't>,
) -> IResult<TokenStream<'i, 't>, (), nom::error::Error<TokenStream<'i, 't>>> {
    move |i: TokenStream<'i, 't>| {
        let orig = i;
        let (i, t) = take(1usize)(i)?;
        match t[0].kind.as_keyword() {
            Some(&k) if k == keyword => Ok((i, ())),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                orig,
                nom::error::ErrorKind::Tag,
            ))),
        }
    }
}
