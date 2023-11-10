use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

use nom::{InputIter, InputLength, InputTake, Needed, Slice};

use super::token::Token;

#[derive(Clone, Copy, Debug)]
pub struct TokenStream<'i, 't> {
    pub(crate) tokens: &'t [Token<'i>],
}

impl<'i, 't> TokenStream<'i, 't> {
    pub const fn new(tokens: &'t [Token<'i>]) -> Self {
        Self { tokens }
    }
}

impl<'i, 't> Index<usize> for TokenStream<'i, 't> {
    type Output = Token<'i>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

impl<'i, 't> InputLength for TokenStream<'i, 't> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'i: 't, 't> InputTake for TokenStream<'i, 't> {
    fn take(&self, count: usize) -> Self {
        Self::new(&self.tokens[..count])
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (first, second) = self.tokens.split_at(count);
        (Self::new(second), Self::new(first))
    }
}
impl<'i, 't> InputIter for TokenStream<'i, 't> {
    type Item = &'t Token<'i>;

    type Iter = std::iter::Enumerate<std::slice::Iter<'t, Token<'i>>>;

    type IterElem = std::slice::Iter<'t, Token<'i>>;

    fn iter_indices(&self) -> Self::Iter {
        self.tokens.iter().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tokens.iter().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.tokens.len()))
        }
    }
}

impl Slice<Range<usize>> for TokenStream<'_, '_> {
    fn slice(&self, _range: Range<usize>) -> Self {
        todo!()
    }
}
impl Slice<RangeFrom<usize>> for TokenStream<'_, '_> {
    fn slice(&self, _range: RangeFrom<usize>) -> Self {
        todo!()
    }
}
impl Slice<RangeTo<usize>> for TokenStream<'_, '_> {
    fn slice(&self, _range: RangeTo<usize>) -> Self {
        todo!()
    }
}
impl Slice<RangeFull> for TokenStream<'_, '_> {
    fn slice(&self, _range: RangeFull) -> Self {
        *self
    }
}
