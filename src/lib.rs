pub mod ast;
pub mod lexer;
pub mod preprocessor;
pub mod span;

pub use preprocessor::{PreprocessorExecutor, preprocess};

pub fn is_valid_for_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
