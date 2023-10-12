use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, map_opt, opt, verify},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use crate::lexer::{
    stream::TokenStream,
    token::{Ident, Keyword, Literal},
};

use super::tree::{control::If, Assignment, Expression, FuncDecl, Statement, Typedef, VarDecl};

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

pub(crate) fn parse_top_level_expression<'i>(
    i: TokenStream<'i>,
) -> IResult<TokenStream<'i>, Expression<'i>> {
    let parse_equals = separated_pair(parse_value, tags::equals, parse_value);
    alt((map(parse_equals, Expression::new_equals), parse_value))(i)
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

fn parse_if(i: TokenStream) -> IResult<TokenStream, If> {
    fn block_or_stmt(
        i: TokenStream,
    ) -> IResult<TokenStream, Vec<Statement>> {
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

fn parse_assignment<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, Assignment<'i>> {
    map(
        terminated(
            separated_pair(parse_ident, tags::assign, parse_value),
            tags::semi_colon,
        ),
        |(lhs, rhs)| Assignment::from((lhs, rhs)),
    )(input)
}

fn parse_statement<'i>(input: TokenStream<'i>) -> IResult<TokenStream<'i>, Statement<'i>> {
    alt((
        map(parse_fn, Statement::FuncDecl),
        map(parse_var_decl, Statement::VarDecl),
        map(parse_typedef, Statement::Typedef),
        map(parse_if, Statement::If),
        map(parse_assignment, Statement::Assign),
    ))(input)
}

pub fn parse_stream(tokens: TokenStream) -> Vec<Statement> {
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
        assert_eq!(body, &Statement::new_var_decl(IDENT_INT, IDENT_B, None));
    }
}
