use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, digit1, multispace0},
    combinator::{opt, recognize},
    multi::many0,
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::token::{Ident, Literal, Token};

pub(super) fn parse_ident(i: &str) -> IResult<&str, Ident> {
    recognize(pair(alt((tag("_"), alpha1)), alphanumeric0))
        .map(Ident::new)
        .parse(i)
}

pub(super) fn parse_literal(i: &str) -> IResult<&str, Literal> {
    fn parse_decimal_literal(i: &str) -> IResult<&str, Literal> {
        recognize(pair(opt(tag("-")), digit1))
            .map(|i: &str| Literal::Int(i.parse().unwrap()))
            .parse(i)
    }
    alt((parse_decimal_literal,))(i)
}

pub(super) fn parse_token(i: &str) -> IResult<&str, Token> {
    alt((
        tag("(").map(|_| Token::OpenParen),
        tag(")").map(|_| Token::CloseParen),
        tag("{").map(|_| Token::OpenBrace),
        tag("}").map(|_| Token::CloseBrace),
        tag("=").map(|_| Token::Assign),
        tag(";").map(|_| Token::SemiColon),
        parse_ident.map(Token::Ident),
        parse_literal.map(Token::Literal),
    ))(i)
}

fn parse_tokens(i: &str) -> IResult<&str, Vec<Token>> {
    many0(delimited(multispace0, parse_token, multispace0))
        .map(|mut tokens| {
            tokens.push(Token::Eof);
            tokens
        })
        .parse(i)
}

pub fn parse_program(i: &str) -> Vec<Token> {
    let (rest, program) = parse_tokens(i).expect("Failed to parse program");
    assert_eq!(rest, "");

    program
}
