use nom::{bytes::complete::take, combinator::verify, IResult};

use crate::lexer::{
    stream::TokenStream,
    token::{Keyword, Token},
};

macro_rules! def_tag {
    ($v:vis $name:ident => $value:expr) => {
        $v fn $name<'i>(i: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
            verify(take(1usize), |t: &TokenStream| t[0] == $value)(i)
        }
    };
}

def_tag!(pub(crate) open_paren => Token::OpenParen);
def_tag!(pub(crate) close_paren => Token::CloseParen);
def_tag!(pub(crate) open_brace => Token::OpenBrace);
def_tag!(pub(crate) close_brace => Token::CloseBrace);
def_tag!(pub(crate) assign => Token::Assign);
def_tag!(pub(crate) semi_colon => Token::SemiColon);

pub(crate) fn eof<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::Eof)(input)
}
