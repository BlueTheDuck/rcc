use nom::{
    branch::alt,
    bytes::complete::take,
    combinator::{map, map_opt},
    sequence::separated_pair,
    IResult,
};

use crate::{
    ast::tree::Expression,
    lexer::{stream::TokenStream, token::Literal},
};

use super::{parse_ident, tags};

pub(crate) fn parse_literal(i: TokenStream) -> IResult<TokenStream, Literal> {
    map_opt(take(1usize), |t: TokenStream| {
        t.tokens[0].as_literal().copied()
    })(i)
}

pub(crate) fn parse_value(i: TokenStream) -> IResult<TokenStream, Expression> {
    alt((
        map(parse_literal, Expression::Literal),
        map(parse_ident, Expression::Ident),
    ))(i)
}

pub fn parse_top_level_expression(i: TokenStream) -> IResult<TokenStream, Expression> {
    let parse_equals = separated_pair(parse_value, tags::equals, parse_value);
    alt((map(parse_equals, Expression::new_equals), parse_value))(i)
}
