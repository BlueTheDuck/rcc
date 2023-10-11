mod parser;
mod tree;

pub use parser::parse_stream;

#[cfg(test)]
mod tests {
    use crate::{
        ast::tree::Statement,
        lexer::{
            stream::TokenStream,
            token::{Ident, Keyword, Token},
        },
    };

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
        const PARSED: &[Statement] = &[Statement::new_func_decl(
            Ident::new("int"),
            Ident::new("main"),
            vec![],
        )];

        let program = parse_stream(TokenStream::new(TOKENS));
        assert_eq!(program, PARSED);
    }

    #[test]
    fn test_var_decl() {
        const TOKENS: &[Token] = &[
            Token::Keyword(Keyword::Typedef),
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("int32_t")),
            Token::SemiColon,
            Token::Eof,
        ];

        if let [t] = parse_stream(TokenStream::new(TOKENS)).as_slice() {
            let ty = t.as_typedef().expect("Expected typedef");
            assert_eq!(ty.ty, vec![Ident::new("int")]);
            assert_eq!(ty.name, Ident::new("int32_t"));
        } else {
            panic!("Expected one statement")
        }
    }
}
