pub mod parsers;
pub mod stream;
pub mod token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<'i> {
    input: &'i str,
    start: usize,
    end: usize,
}
impl<'i> Span<'i> {
    pub(crate) fn new(input: &'i str, start: usize, end: usize) -> Self {
        Self { input, start, end }
    }
    pub(crate) fn new_remaining(input: &'i str, start: usize) -> Self {
        Self::new(input, start, input.len())
    }

    pub fn get(&self) -> &'i str {
        self.input.get(self.start..self.end).unwrap()
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}
impl<'i> std::fmt::Display for Span<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = self.input.get(self.start..self.end).unwrap();
        use termion::color::*;
        
        write!(f, "Span({}{slice:?}{})", Fg(Green), Fg(Reset))
    }
}
impl<'i, R> PartialEq<R> for Span<'i>
where R: AsRef<str> {
    fn eq(&self, other: &R) -> bool {
        self.get() == other.as_ref()
    }
}
impl<'i> std::hash::Hash for Span<'i> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

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

    #[test]
    fn test_parse_with_pointers() {
        const IDENT_INT: Ident = Ident::new("int");
        const PROGRAM_CODE: &str = r#"int main(int argc, char** argv) {int *ptr;}"#;
        const PARSED: &[Token] = &[
            Token::Ident(IDENT_INT),
            Token::Ident(Ident::new("main")),
            Token::OpenParen,
            Token::Ident(IDENT_INT),
            Token::Ident(Ident::new("argc")),
            Token::Comma,
            Token::Ident(Ident::new("char")),
            Token::Star,
            Token::Star,
            Token::Ident(Ident::new("argv")),
            Token::CloseParen,
            Token::OpenBrace,
            Token::Ident(IDENT_INT),
            Token::Star,
            Token::Ident(Ident::new("ptr")),
            Token::SemiColon,
            Token::CloseBrace,
            Token::Eof,
        ];

        let program = parse_program(PROGRAM_CODE);
        assert_eq!(program, PARSED);
    }
}
