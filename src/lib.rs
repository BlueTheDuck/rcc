pub mod ast;
pub mod lexer;
pub mod span;
pub mod preprocessor;

pub use preprocessor::{
    preprocess, 
};

pub fn is_valid_for_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
