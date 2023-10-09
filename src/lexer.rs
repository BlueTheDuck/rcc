use std::fmt::Display;

use nom::{
    self,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::{map, opt, recognize},
    multi::fold_many1,
    sequence::pair,
};

pub enum Token<'i> {
    Ident(&'i str),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
}

impl<'i> Display for Token<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
        }
    }
}

fn lex_ident(i: &str) -> nom::IResult<&str, Token> {
    let (i, _) = nom::character::complete::multispace0(i)?;

    let (i, ident) = recognize(pair(opt(tag("_")), alphanumeric1))(i)?;

    let (i, _) = nom::character::complete::multispace0(i)?;

    Ok((i, Token::Ident(ident)))
}

fn lex_token(i: &str) -> nom::IResult<&str, Token> {
    let (i, _) = nom::character::complete::multispace0(i)?;
    let (i, token) = nom::branch::alt((
        map(tag("("), |_| Token::OpenParen),
        map(tag(")"), |_| Token::CloseParen),
        map(tag("{"), |_| Token::OpenBrace),
        map(tag("}"), |_| Token::CloseBrace),
        lex_ident,
    ))(i)?;
    let (i, _) = nom::character::complete::multispace0(i)?;

    Ok((i, token))
}

pub fn lex<'i>(i: &'i str) -> Result<Vec<Token<'i>>, ()> {
    let (_, tokens) = fold_many1(lex_token, Vec::new, |mut v, t| {
        v.push(t);
        v
    })(i)
    .unwrap();

    Ok(tokens)
}
