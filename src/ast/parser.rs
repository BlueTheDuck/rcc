use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, map_opt, opt, verify},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use crate::lexer::{
    stream::TokenStream,
    token::{Ident, Keyword, Literal},
};

use super::tree::{FuncDecl, Statement, Typedef, VarDecl};

mod blocks;
mod tags;

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

pub(crate) fn parse_value<'i>(i: TokenStream<'i>) -> IResult<TokenStream<'i>, Expression<'i>> {
    alt((
        map(parse_literal, Expression::Literal),
        map(parse_ident, Expression::Ident),
    ))(i)
}

pub(crate) fn parse_top_level_expression<'i>(i: TokenStream<'i>) -> IResult<TokenStream<'i>, Expression<'i>> {
    let parse_equals = separated_pair(parse_value, tags::equals, parse_value);
    alt((
        map(parse_equals, |(lhs, rhs)| Expression::Equals {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }),
    ))(i)
}

fn parse_var_decl<'i>(input: TokenStream<'i>) -> IResult<TokenStream, VarDecl> {
    map(
        tuple((
            parse_ident,
            parse_ident,
            opt(preceded(tags::assign, parse_top_level_expression)),
            tags::semi_colon,
        )),
        |(ty, name, value, _)| VarDecl { ty, name, value },
    )(input)
}

fn parse_fn<'i>(input: TokenStream<'i>) -> IResult<TokenStream, FuncDecl> {
    map(
        tuple((
            parse_ident,
            parse_ident,
            tags::open_paren,
            tags::close_paren,
            blocks::braces(many0(parse_statement)),
        )),
        |(ty, name, _, _, body)| FuncDecl {
            ret: ty,
            name,
            body,
        },
    )(input)
}

fn parse_typedef<'i>(i: TokenStream<'i>) -> IResult<TokenStream<'i>, Typedef<'i>> {
    delimited(
        tags::keyword(Keyword::Typedef),
        verify(many1(parse_ident), |t: &Vec<Ident>| t.len() >= 2),
        tags::semi_colon,
    )
    .map(|mut idents| {
        let name = idents.pop().unwrap();
        Typedef { ty: idents, name }
    })
    .parse(i)
}

fn parse_statement<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, Statement<'i>> {
    alt((
        map(parse_fn, Statement::FuncDecl),
        map(parse_var_decl, Statement::VarDecl),
        map(parse_typedef, Statement::Typedef),
    ))(input)
}

pub fn parse_stream<'i>(tokens: TokenStream<'i>) -> Vec<Statement<'i>> {
    match terminated(many0(parse_statement), tags::eof)(tokens) {
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
