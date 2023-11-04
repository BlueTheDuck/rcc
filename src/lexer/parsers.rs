use crate::{Span, span::split_source};

use super::token::Token;

fn parse_tokens(spans: Vec<Span>) -> Vec<Token> {
    let mut tokens = Vec::new();

    

    tokens
}

pub fn parse_program(i: &str) -> Vec<Token> {
    let program = parse_tokens(split_source(i));
    

    program
}
