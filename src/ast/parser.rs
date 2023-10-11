use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, map_opt, verify, opt},
    multi::many0,
    sequence::{terminated, tuple, preceded},
    IResult,
};

use crate::lexer::{
    stream::TokenStream,
    token::{Ident, Literal, Token},
};

use super::tree::{FuncDecl, Statement, VarDecl};

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
fn assign_tag<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::Assign)(input)
}
fn semi_colon_tag<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::SemiColon)(input)
}
fn eof_tag<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, TokenStream<'i>> {
    verify(take(1usize), |t: &TokenStream| t[0] == Token::Eof)(input)
}

pub(super) fn parse_ident<'i>(i: TokenStream<'i>) -> IResult<TokenStream<'i>, Ident> {
    map_opt(take(1usize), |t: TokenStream| {
        t.tokens[0].as_ident().copied()
    })(i)
}

fn parse_literal<'i>(i: TokenStream<'i>) -> IResult<TokenStream<'i>, Literal> {
    map_opt(take(1usize), |t: TokenStream| {
        t.tokens[0].as_literal().copied()
    })(i)
}

fn parse_var_decl<'i>(input: TokenStream<'i>) -> IResult<TokenStream, VarDecl> {
    map(
        tuple((parse_ident, parse_ident, opt(preceded(assign_tag, parse_literal)), semi_colon_tag)),
        |(ty, name, value, _)| VarDecl {
            ty,
            name,
            value,
        },
    )(input)
}

fn parse_fn<'i>(input: TokenStream<'i>) -> IResult<TokenStream, FuncDecl> {
    map(
        tuple((
            parse_ident,
            parse_ident,
            open_paren_tag,
            close_paren_tag,
            open_brace_tag,
            many0(parse_statement),
            close_brace_tag,
        )),
        |(ty, name, _, _, _, body, _)| FuncDecl {
            ret: ty,
            name,
            body,
        },
    )(input)
}

fn parse_statement<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, Statement<'i>> {
    alt((
        map(parse_fn, Statement::FuncDecl),
        map(parse_var_decl, Statement::VarDecl),
        /* |i: TokenStream| { panic!("Unimplemented: {:?}", i.tokens) } */
    ))(input)
}

pub fn parse_stream<'i>(tokens: TokenStream<'i>) -> Vec<Statement<'i>> {
    match terminated(many0(parse_statement), eof_tag)(tokens) {
        Ok((rest, program)) => {
            if !rest.tokens.is_empty() {
                println!("Warning: {} tokens left over", rest.tokens.len());
                for t in rest.tokens {
                    println!("  {:?}", t);
                }
            }
            return program;
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}
