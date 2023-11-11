use crate::span::Span;

mod executor;
mod iterator;
mod parser;

pub(crate) use executor::Executor;
pub(crate) use parser::SpanType;

pub fn preprocess<'i>(source: &'i str) -> executor::Executor<'i, iterator::PreprocessorTokenIter> {
    execute(parse_preprocessor(source))
}

pub(crate) fn parse_preprocessor(source: &str) -> iterator::PreprocessorTokenIter {
    iterator::PreprocessorTokenIter::new(source)
}
pub(crate) fn execute<'i, I>(iter: I) -> executor::Executor<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    executor::Executor::new(iter)
}
