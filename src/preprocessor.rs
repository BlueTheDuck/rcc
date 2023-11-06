use crate::span::Span;

use self::parser::{take_preprocessing_seq, SpanType};

pub mod executor;
mod parser;

pub fn preprocess(source: &str) -> PreprocessorTokenIter {
    PreprocessorTokenIter::new(source)
}

pub struct PreprocessorTokenIter<'i> {
    source: &'i str,
    start: usize,
}
impl<'i> PreprocessorTokenIter<'i> {
    fn new(source: &'i str) -> Self {
        Self { source, start: 0 }
    }
}
impl<'i> Iterator for PreprocessorTokenIter<'i> {
    type Item = Span<'i, SpanType>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(span) = take_preprocessing_seq(self.source, self.start) {
                self.start = span.end();
                if span.len() > 0 {
                    break Some(span);
                }
            } else {
                break None;
            }
        }
    }
}
