use nom::{sequence::delimited, IResult, Parser};

use crate::lexer::stream::TokenStream;

use super::tags;

pub(super) fn braces<'i, P, O>(
    parser: P,
) -> impl FnMut(TokenStream<'i>) -> IResult<TokenStream<'i>, O, nom::error::Error<TokenStream<'i>>>
where
    P: Parser<TokenStream<'i>, O, nom::error::Error<TokenStream<'i>>>,
{
    delimited(tags::open_brace, parser, tags::close_brace)
}

pub(super) fn parens<'i, P, O>(
    parser: P,
) -> impl FnMut(TokenStream<'i>) -> IResult<TokenStream<'i>, O, nom::error::Error<TokenStream<'i>>>
where
    P: Parser<TokenStream<'i>, O, nom::error::Error<TokenStream<'i>>>,
{
    delimited(tags::open_paren, parser, tags::close_paren)
}

#[cfg(test)]
mod tests {
    /* use crate::{
        ast::{parse_stream, parser::parse_statement, tree::Statement},
        lexer::{
            parsers::parse_program,
            token::{Ident, Token},
        },
    };

    #[test]
    fn test_nested_blocks() {
        const TOKENS: &[Token] = parse_program("{{int a;}}");
        const AST: Statement = parse_statement(TOKENS);
        let only_one_ident = parse_stream(TOKENS);
        assert_eq!(only_one_ident.get(0).map(|t| t.), Some(Ident::new("a")));
    } */
}
