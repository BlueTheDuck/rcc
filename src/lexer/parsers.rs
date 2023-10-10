use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, multispace0},
    combinator::recognize,
    multi::many0,
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::token::{Ident, Token};

fn parse_ident(i: &str) -> IResult<&str, Ident> {
    recognize(pair(alt((tag("_"), alpha1)), alphanumeric0))
        .map(Ident::new)
        .parse(i)
}

fn parse_token(i: &str) -> IResult<&str, Token> {
    alt((
        tag("(").map(|_| Token::OpenParen),
        tag(")").map(|_| Token::CloseParen),
        tag("{").map(|_| Token::OpenBrace),
        tag("}").map(|_| Token::CloseBrace),
        parse_ident.map(Token::Ident),
    ))(i)
}

fn parse_tokens(i: &str) -> IResult<&str, Vec<Token>> {
    many0(delimited(multispace0, parse_token, multispace0))
        .map(|mut tokens| {tokens.push(Token::Eof); tokens})
        .parse(i)
}

pub fn parse_program(i: &str) -> Vec<Token> {
    let (_, program) = parse_tokens(i).expect("Failed to parse program");

    program
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::{Ident, Token};

    #[test]
    fn parse_ident() {
        const IDENT: &str = "main";
        let (rest, ident) = super::parse_ident(IDENT).expect("Failed to parse ident");
        assert_eq!(ident, Ident::new("main"));
        assert_eq!(rest, "");
    }

    #[test]
    fn parse_many_idents() {
        const IDENTS: &str = "int main";
        let (rest, ident) = super::parse_ident(IDENTS).expect("Failed to parse 1st ident");
        assert_eq!(ident, Ident::new("int"));
        assert_eq!(rest, " main");
        let (rest, ident) = super::parse_ident(rest.trim()).expect("Failed to parse 2nd ident");
        assert_eq!(ident, Ident::new("main"));
        assert_eq!(rest, "");
    }

    #[test]
    fn parse_empty_main() {
        const PROGRAM_CODE: &str = r#"int main() {}"#;
        const PARSED: &[Token] = &[
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("main")),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::Eof,
        ];

        let program = super::parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }
}
