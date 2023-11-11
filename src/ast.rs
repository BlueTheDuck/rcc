mod parser;
mod tree;

pub use parser::parse_stream;

#[cfg(test)]
mod tests {
    use crate::{
        ast::tree::{control::If, Declarator, Expression, Statement},
        lexer::{
            stream::TokenStream,
            token::{Ident, Keyword, Token, TokenKind},
        },
        span::Span, preprocessor::preprocess,
    };

    use super::*;

    #[test]
    fn test_parse_empty_main() {
        const SOURCE: &str = "int main() {}";
        const IDENT_INT: Ident = Ident::new("int");
        const IDENT_MAIN: Ident = Ident::new("main");
        const IDENT_VOID: Ident = Ident::new("void");

        let tokens: Vec<_> = crate::lexer::parse_tokens(preprocess(SOURCE)).collect();

        let program = parse_stream(TokenStream::new(&tokens));
        assert_eq!(
            program,
            vec![Statement::new_func_decl(
                IDENT_INT,
                IDENT_MAIN,
                vec![],
                vec![],
            )]
        );
    }

    #[test]
    fn test_var_decl() {
        const SOURCE: &str = "typedef int int32_t;";

        let tokens: Vec<_> = crate::lexer::parse_tokens(preprocess(SOURCE)).collect();

        if let [t] = parse_stream(TokenStream::new(&tokens)).as_slice() {
            let ty = t.as_typedef().expect("Expected typedef");
            assert_eq!(ty.ty, vec![Ident::new("int")]);
            assert_eq!(ty.name, Ident::new("int32_t"));
        } else {
            panic!("Expected one statement")
        }
    }

    #[test]
    fn test_simple_equals() {
        const SOURCE: &str = "x == y";
        let tokens: Vec<_> = crate::lexer::parse_tokens(preprocess(SOURCE)).collect();
        let (rest, parsed) = parser::parse_top_level_expression(TokenStream::new(&tokens))
            .expect("Could not parse expression");
        assert_eq!(rest.tokens, &[TokenKind::Eof]);
        if let Expression::Equals { lhs, rhs } = parsed {
            assert_eq!(*lhs, Expression::Ident(Ident::new("x")));
            assert_eq!(*rhs, Expression::Ident(Ident::new("y")));
        } else {
            panic!("Expected equals expression")
        }
    }

    /* #[test]
    fn test_simple_if() {
        const IDENT_INT: Ident = Ident::new("int");
        const IDENT_X: Ident = Ident::new("x");
        const IDENT_Y: Ident = Ident::new("y");
        const TOKENS: &[TokenKind] = &[
            TokenKind::Keyword(Keyword::If),
            TokenKind::OpenParen,
            TokenKind::Ident(IDENT_X),
            TokenKind::Equals,
            TokenKind::Ident(IDENT_Y),
            TokenKind::CloseParen,
            TokenKind::OpenBrace,
            TokenKind::Ident(IDENT_INT),
            TokenKind::Ident(IDENT_X),
            TokenKind::Assign,
            TokenKind::Ident(IDENT_Y),
            TokenKind::SemiColon,
            TokenKind::CloseBrace,
            TokenKind::Eof,
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
    } */

    /* #[test]
    fn test_declarations_with_pointer() {
        const IDENT_INT: Ident = Ident::new("int");
        const IDENT_MAIN: Ident = Ident::new("main");
        const IDENT_ARGC: Ident = Ident::new("argc");
        const IDENT_ARGV: Ident = Ident::new("argv");
        const IDENT_CHAR: Ident = Ident::new("char");
        const IDENT_PTR: Ident = Ident::new("ptr");
        const PARSED: &[TokenKind] = &[
            TokenKind::Ident(IDENT_INT),
            TokenKind::Ident(IDENT_MAIN),
            TokenKind::OpenParen,
            TokenKind::Ident(IDENT_INT),
            TokenKind::Ident(IDENT_ARGC),
            TokenKind::Comma,
            TokenKind::Ident(IDENT_CHAR),
            TokenKind::Star,
            TokenKind::Star,
            TokenKind::Ident(IDENT_ARGV),
            TokenKind::CloseParen,
            TokenKind::OpenBrace,
            TokenKind::Ident(IDENT_INT),
            TokenKind::Star,
            TokenKind::Ident(IDENT_PTR),
            TokenKind::SemiColon,
            TokenKind::CloseBrace,
            TokenKind::Eof,
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
    } */
}
