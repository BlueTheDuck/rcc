pub mod parsers;
pub mod stream;
pub mod token;

#[cfg(test)]
mod tests {
    use crate::lexer::{
        parsers::{parse_ident, parse_program},
        token::{Ident, Token},
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
}
