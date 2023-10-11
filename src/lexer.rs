pub mod parsers;
pub mod stream;
pub mod token;

#[cfg(test)]
mod tests {
    use crate::lexer::{
        parsers::{parse_ident, parse_program},
        token::{Ident, Keyword, Literal, Token},
    };

    #[test]
    fn test_parse_ident() {
        const IDENT: &str = "main";
        let (rest, ident) = parse_ident(IDENT).expect("Failed to parse ident");
        assert_eq!(ident, Ident::new("main"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_many_idents() {
        const IDENTS: &str = "int main";
        let (rest, ident) = parse_ident(IDENTS).expect("Failed to parse 1st ident");
        assert_eq!(ident, Ident::new("int"));
        assert_eq!(rest, " main");
        let (rest, ident) = parse_ident(rest.trim()).expect("Failed to parse 2nd ident");
        assert_eq!(ident, Ident::new("main"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_empty_main() {
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

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }

    #[test]
    fn test_parse_main_with_stmt() {
        const PROGRAM_CODE: &str = r#"int main() {int x = 2;}"#;
        const PARSED: &[Token] = &[
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("main")),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("x")),
            Token::Assign,
            Token::Literal(Literal::Int(2)),
            Token::SemiColon,
            Token::CloseBrace,
            Token::Eof,
        ];

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }
    #[test]
    fn test_parse_main_with_many_stmts() {
        const PROGRAM_CODE: &str = r#"void main() {
                                                 int x = 2;
                                                 int y = 3;
                                             }"#;
        const PARSED: &[Token] = &[
            Token::Ident(Ident::new("void")),
            Token::Ident(Ident::new("main")),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("x")),
            Token::Assign,
            Token::Literal(Literal::Int(2)),
            Token::SemiColon,
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("y")),
            Token::Assign,
            Token::Literal(Literal::Int(3)),
            Token::SemiColon,
            Token::CloseBrace,
            Token::Eof,
        ];

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }

    #[test]
    fn test_parse_var_decl() {
        const PROGRAM_CODE: &str = r#"int a;"#;
        const PARSED: &[Token] = &[
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("a")),
            Token::SemiColon,
            Token::Eof,
        ];

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }

    #[test]
    fn test_parse_typedef() {
        const PROGRAM_CODE: &str = r#"typedef int int32_t;"#;
        const PARSED: &[Token] = &[
            Token::Keyword(Keyword::Typedef),
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("int32_t")),
            Token::SemiColon,
            Token::Eof,
        ];

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }

    #[test]
    fn test_parse_typedef_space() {
        const PROGRAM_CODE: &str = r#"typedef unsigned int uint32_t;"#;
        const PARSED: &[Token] = &[
            Token::Keyword(Keyword::Typedef),
            Token::Ident(Ident::new("unsigned")),
            Token::Ident(Ident::new("int")),
            Token::Ident(Ident::new("uint32_t")),
            Token::SemiColon,
            Token::Eof,
        ];

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }

    #[test]
    fn test_parse_if() {
        const PROGRAM_CODE: &str = r#"if(1 == 2) {}"#;
        const PARSED: &[Token] = &[
            Token::Keyword(Keyword::If),
            Token::OpenParen,
            Token::Literal(Literal::Int(1)),
            Token::Equals,
            Token::Literal(Literal::Int(2)),
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::Eof,
        ];

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }
}
