use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, map_opt, opt, verify},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use crate::lexer::{
    stream::TokenStream,
    token::{Ident, Keyword},
};

use super::tree::{control::If, Assignment, Declarator, FuncDecl, Statement, Typedef, VarDecl};

mod blocks;
pub mod expr;
mod tags;

pub use expr::parse_top_level_expression;

pub(super) fn parse_ident<'i, 't>(
    i: TokenStream<'i, 't>,
) -> IResult<TokenStream<'i, 't>, Ident<'i>> {
    map_opt(take(1usize), |t: TokenStream| {
        t.tokens[0].as_ident().copied()
    })(i)
}

pub(super) fn parse_declarator<'i, 't>(
    i: TokenStream<'i, 't>,
) -> IResult<TokenStream<'i, 't>, Declarator<'i>> {
    alt((
        map(parse_ident, Declarator::Ident),
        map(preceded(tags::star, parse_declarator), |d| {
            Declarator::Pointer(Box::new(d))
        }),
    ))(i)
}

fn parse_fn<'i, 't>(input: TokenStream<'i, 't>) -> IResult<TokenStream<'i, 't>, FuncDecl<'i>> {
    let params_parser = alt((
        verify(parse_ident, |&ident| ident.name == "void").map(|_| vec![]),
        separated_list0(tags::comma, pair(parse_ident, parse_declarator)),
    ));

    map(
        tuple((
            parse_ident,
            parse_ident,
            blocks::parens(params_parser),
            blocks::braces(many0(parse_statement)),
        )),
        |(ty, name, args, body)| FuncDecl {
            ret: ty,
            name,
            args,
            body,
        },
    )(input)
}

fn parse_var_decl<'i, 't>(input: TokenStream<'i, 't>) -> IResult<TokenStream<'i, 't>, VarDecl<'i>> {
    map(
        tuple((
            parse_ident,
            parse_declarator,
            opt(preceded(tags::assign, parse_top_level_expression)),
            tags::semi_colon,
        )),
        |(ty, name, value, _)| VarDecl { ty, name, value },
    )(input)
}

fn parse_typedef<'i, 't>(i: TokenStream<'i, 't>) -> IResult<TokenStream<'i, 't>, Typedef<'i>> {
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

fn parse_if<'i, 't>(i: TokenStream<'i, 't>) -> IResult<TokenStream<'i, 't>, If<'i>> {
    fn block_or_stmt<'i, 't>(
        i: TokenStream<'i, 't>,
    ) -> IResult<TokenStream<'i, 't>, Vec<Statement<'i>>> {
        alt((
            blocks::braces(many0(parse_statement)),
            parse_statement.map(|s| vec![s]),
        ))(i)
    }

    preceded(
        tags::keyword(Keyword::If),
        tuple((
            blocks::parens(parse_top_level_expression),
            block_or_stmt,
            opt(preceded(tags::keyword(Keyword::Else), block_or_stmt)),
        )),
    )
    .map(|(cond, then, r#else)| If {
        condition: cond,
        body: then,
        else_body: r#else,
    })
    .parse(i)
}

fn parse_assignment<'i, 't>(
    input: TokenStream<'i, 't>,
) -> IResult<TokenStream<'i, 't>, Assignment<'i>> {
    map(
        terminated(
            separated_pair(parse_ident, tags::assign, parse_top_level_expression),
            tags::semi_colon,
        ),
        |(lhs, rhs)| Assignment::from((lhs, rhs)),
    )(input)
}

fn parse_statement<'i, 't>(
    input: TokenStream<'i, 't>,
) -> IResult<TokenStream<'i, 't>, Statement<'i>> {
    alt((
        map(parse_fn, Statement::FuncDecl),
        map(parse_var_decl, Statement::VarDecl),
        map(parse_typedef, Statement::Typedef),
        map(parse_if, Statement::If),
        map(parse_assignment, Statement::Assign),
    ))(input)
}

pub fn parse_stream<'i>(tokens: TokenStream<'i, '_>) -> Vec<Statement<'i>> {
    match terminated(many0(parse_statement), tags::eof)(tokens) {
        Ok((rest, program)) => {
            if !rest.tokens.is_empty() {
                println!("Warning: {} tokens left over", rest.tokens.len());
                for t in rest.tokens {
                    println!("  {:?}", t);
                }
            }

            program
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            parser::parse_if,
            tree::{Expression, Statement},
        },
        lexer::{
            stream::TokenStream,
            token::{Ident, Keyword, Token},
        },
    };

    #[test]
    fn test_parse_if() {
        const IDENT_INT: Ident = Ident::new("int");
        const IDENT_A: Ident = Ident::new("a");
        const IDENT_B: Ident = Ident::new("b");

        const TOKENS: &[Token] = &[
            Token::Keyword(Keyword::If),
            Token::OpenParen,
            Token::Ident(IDENT_A),
            Token::CloseParen,
            Token::OpenBrace,
            Token::Ident(IDENT_INT),
            Token::Ident(IDENT_B),
            Token::SemiColon,
            Token::CloseBrace,
        ];
        let (rest, r#if) =
            parse_if(TokenStream::new(TOKENS)).expect("Could not parse token stream");
        assert!(rest.tokens.is_empty());

        if let Expression::Ident(id) = r#if.condition {
            assert_eq!(id, IDENT_A);
        } else {
            panic!("Expected ident");
        }

        let body = match &r#if.body[..] {
            [body] => body,
            _ => panic!("Expected one statement"),
        };
        assert_eq!(
            body,
            &Statement::new_var_decl(IDENT_INT, IDENT_B.into(), None)
        );
    }
}
