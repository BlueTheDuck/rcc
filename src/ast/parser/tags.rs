use nom::{bytes::complete::take, combinator::verify, IResult};

use crate::lexer::{
    stream::TokenStream,
    token::{Keyword, Token},
};

macro_rules! def_tag {
    ($v:vis $name:ident => $value:expr) => {
        $v fn $name(i: TokenStream) -> IResult<TokenStream, TokenStream> {
            verify(take(1usize), |t: &TokenStream| t[0] == $value)(i)
        }
    };
}

def_tag!(pub(crate) open_paren => Token::OpenParen);
def_tag!(pub(crate) close_paren => Token::CloseParen);
def_tag!(pub(crate) open_brace => Token::OpenBrace);
def_tag!(pub(crate) close_brace => Token::CloseBrace);
def_tag!(pub(crate) equals => Token::Equals);
def_tag!(pub(crate) assign => Token::Assign);
def_tag!(pub(crate) semi_colon => Token::SemiColon);
def_tag!(pub(crate) comma => Token::Comma);
def_tag!(pub(crate) star => Token::Star);

pub(crate) fn eof(input: TokenStream) -> IResult<TokenStream, TokenStream> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::Eof)(input)
}

pub(crate) fn keyword<'i>(
    keyword: Keyword,
) -> impl FnMut(TokenStream<'i>) -> IResult<TokenStream<'i>, (), nom::error::Error<TokenStream<'i>>>
{
    move |i: TokenStream<'i>| {
        let orig = i;
        let (i, t) = take(1usize)(i)?;
        match t[0].as_keyword() {
            Some(&k) if k == keyword => Ok((i, ())),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                orig,
                nom::error::ErrorKind::Tag,
            ))),
        }
    }
}
