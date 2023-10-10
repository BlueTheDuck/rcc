use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{eof, map, map_opt, verify},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

use crate::lexer::{
    stream::TokenStream,
    token::{Ident, Token},
};

use super::tree::{FuncDecl, Statement};

fn open_paren_tag<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::OpenParen)(input)
}
fn close_paren_tag<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::CloseParen)(input)
}
fn open_brace_tag<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::OpenBrace)(input)
}
fn close_brace_tag<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::CloseBrace)(input)
}

fn parse_ident<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, Ident> {
    map_opt(verify(take(1usize), |_| true), |t: TokenStream| {
        t[0].as_ident().copied()
    })(input)
}

fn parse_fn<'i>(input: TokenStream<'i>) -> IResult<TokenStream, FuncDecl> {
    map(
        tuple((
            parse_ident,
            parse_ident,
            open_paren_tag,
            close_paren_tag,
            open_brace_tag,
            close_brace_tag,
        )),
        |(ty, name, _, _, _, _)| FuncDecl {
            name,
            ret: ty,
            body: None,
        },
    )(input)
}

fn parse_statement<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, Statement<'i>> {
    alt((map(parse_fn, Statement::FuncDecl),))(input)
}

pub fn parse_stream<'i>(tokens: TokenStream<'i>) -> Vec<Statement<'i>> {
    let (_, program) = terminated(many0(parse_statement), eof)(tokens).unwrap();

    program
}
