mod parser;
mod tree;

pub use parser::parse_stream;

#[cfg(test)]
mod tests {
    use crate::{
        ast::tree::{control::If, Declarator, Expression, Statement},
        lexer::{
            stream::TokenStream,
            token::{Ident, Keyword, Token},
        },
    };

    use super::*;

    #[test]
    fn test_parse_empty_main() {
        const IDENT_INT: Ident = Ident::new("int");
        const IDENT_MAIN: Ident = Ident::new("main");
        const IDENT_VOID: Ident = Ident::new("void");
        const TOKENS: &[Token] = &[
            Token::Ident(IDENT_INT),
            Token::Ident(IDENT_MAIN),
            Token::OpenParen,
            Token::Ident(IDENT_VOID),
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::Eof,
        ];
        const PARSED: &[Statement] = &[Statement::new_func_decl(
            IDENT_INT,
            IDENT_MAIN,
            vec![],
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

    #[test]
    fn test_simple_equals() {
        const TOKENS: &[Token] = &[
            Token::Ident(Ident::new("x")),
            Token::Equals,
            Token::Ident(Ident::new("y")),
            Token::SemiColon,
            Token::Eof,
        ];
        let (rest, parsed) = parser::parse_top_level_expression(TokenStream::new(TOKENS))
            .expect("Could not parse expression");
        assert_eq!(rest.tokens, &[Token::SemiColon, Token::Eof]);
        if let Expression::Equals { lhs, rhs } = parsed {
            assert_eq!(*lhs, Expression::Ident(Ident::new("x")));
            assert_eq!(*rhs, Expression::Ident(Ident::new("y")));
        } else {
            panic!("Expected equals expression")
        }
    }

    #[test]
    fn test_simple_if() {
        const IDENT_INT: Ident = Ident::new("int");
        const IDENT_X: Ident = Ident::new("x");
        const IDENT_Y: Ident = Ident::new("y");
        const TOKENS: &[Token] = &[
            Token::Keyword(Keyword::If),
            Token::OpenParen,
            Token::Ident(IDENT_X),
            Token::Equals,
            Token::Ident(IDENT_Y),
            Token::CloseParen,
            Token::OpenBrace,
            Token::Ident(IDENT_INT),
            Token::Ident(IDENT_X),
            Token::Assign,
            Token::Ident(IDENT_Y),
            Token::SemiColon,
            Token::CloseBrace,
            Token::Eof,
        ];
        let if_statement: Statement = Statement::If(If {
            condition: Expression::new_equals((IDENT_X.into(), IDENT_Y.into())),
            body: vec![Statement::new_var_decl(
                IDENT_INT,
                IDENT_X.into(),
                Some(IDENT_Y.into()),
            )],
            else_body: None,
        });

        let ast: Vec<Statement> = parser::parse_stream(TokenStream::new(TOKENS));

        let statement = match &ast[..] {
            [stmt] => stmt,
            _ => panic!("Expected one statement"),
        };
        assert_eq!(statement, &if_statement);
    }

    #[test]
    fn test_declarations_with_pointer() {
        const IDENT_INT: Ident = Ident::new("int");
        const IDENT_MAIN: Ident = Ident::new("main");
        const IDENT_ARGC: Ident = Ident::new("argc");
        const IDENT_ARGV: Ident = Ident::new("argv");
        const IDENT_CHAR: Ident = Ident::new("char");
        const IDENT_PTR: Ident = Ident::new("ptr");
        const PARSED: &[Token] = &[
            Token::Ident(IDENT_INT),
            Token::Ident(IDENT_MAIN),
            Token::OpenParen,
            Token::Ident(IDENT_INT),
            Token::Ident(IDENT_ARGC),
            Token::Comma,
            Token::Ident(IDENT_CHAR),
            Token::Star,
            Token::Star,
            Token::Ident(IDENT_ARGV),
            Token::CloseParen,
            Token::OpenBrace,
            Token::Ident(IDENT_INT),
            Token::Star,
            Token::Ident(IDENT_PTR),
            Token::SemiColon,
            Token::CloseBrace,
            Token::Eof,
        ];
        let parsed_ast: &[Statement] = &[Statement::new_func_decl(
            IDENT_INT,
            IDENT_MAIN,
            vec![
                (IDENT_INT, IDENT_ARGC.into()),
                (
                    IDENT_CHAR,
                    Declarator::Pointer(Box::new(Declarator::Pointer(Box::new(IDENT_ARGV.into())))),
                ),
            ],
            vec![Statement::new_var_decl(
                IDENT_INT,
                Declarator::Pointer(Box::new(IDENT_PTR.into())),
                None,
            )],
        )];

        let ast = parse_stream(TokenStream::new(PARSED));
        assert_eq!(ast, parsed_ast);
    }
}
