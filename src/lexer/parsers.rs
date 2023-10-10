use nom::{
    self,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::{eof, opt, recognize},
    multi::{many0},
    sequence::{pair, terminated},
    IResult, Parser,
};

use super::token::{Ident, Token};

fn parse_ident(i: &str) -> IResult<&str, Ident> {
    recognize(pair(opt(tag("_")), alphanumeric1))(i).map(|(i, id)| (i, Ident::new(id)))
}

fn parse_token(i: &str) -> IResult<&str, Token> {
    nom::branch::alt((
        tag("(").map(|_| Token::OpenParen),
        tag(")").map(|_| Token::CloseParen),
        tag("{").map(|_| Token::OpenBrace),
        tag("}").map(|_| Token::CloseBrace),
        parse_ident.map(Token::Ident),
    ))(i)
}

pub fn parse_program(i: &str) -> Vec<Token> {
    let (_, program) = many0(terminated(parse_token, eof))(i).unwrap();

    program
}
