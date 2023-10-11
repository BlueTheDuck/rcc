mod parser;
mod tree;

pub use parser::parse_stream;

#[cfg(test)]
mod tests {
    use crate::{lexer::{token::{Token, Ident}, stream::TokenStream}, ast::tree::{Statement}};

    use super::*;

    #[test]
    fn test_parse_empty_main() {
        const TOKENS: &[Token] = &[
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("main")),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::Eof,
        ];
        const PARSED: &[Statement] = &[
            Statement::new_func_decl(
                Ident::new("int"), Ident::new("main"), vec![])
        ];

        let program = parse_stream(TokenStream::new(TOKENS));
        assert_eq!(program, PARSED);
    }
}
