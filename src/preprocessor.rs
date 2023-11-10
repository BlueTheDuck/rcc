use crate::span::Span;

mod executor;
mod parser;

pub use executor::PreprocessorExecutor;
pub use parser::SpanType;

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
            if let Some(span) = parser::take_preprocessing_seq(self.source, self.start) {
                self.start = span.end();
                if !span.is_empty() {
                    break Some(span);
                }
            } else if self.start == self.source.len() {
                let span = Span::new_remaining_with(self.source, self.start, SpanType::Eof);
                self.start = span.end() + 1;
                break Some(span);
            } else if self.start < self.source.len() {
                break Some(Span::new_remaining_with(
                    self.source,
                    self.start,
                    SpanType::None,
                ));
            } else {
                break None;
            }
        }
    }
}
