use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

use nom::{InputIter, InputLength, InputTake, Slice};

use super::{
    token::{Token},
};

#[derive(Clone, Copy, Debug)]
pub struct TokenStream<'i> {
    tokens: &'i [Token<'i>],
    /* pos: usize, */
}

impl<'i> TokenStream<'i> {
    pub fn new(tokens: &'i [Token<'i>]) -> Self {
        Self {
            tokens, /* , pos: 0 */
        }
    }
}

impl<'i> Index<usize> for TokenStream<'i> {
    type Output = Token<'i>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

impl<'i> InputLength for TokenStream<'i> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'i> InputTake for TokenStream<'i> {
    fn take(&self, count: usize) -> Self {
        Self::new(&self.tokens[..count])
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (l, r) = self.tokens.split_at(count);
        (Self::new(l), Self::new(r))
    }
}
impl<'i> InputIter for TokenStream<'i> {
    type Item = &'i Token<'i>;

    type Iter = std::iter::Enumerate<std::slice::Iter<'i, Token<'i>>>;

    type IterElem = std::slice::Iter<'i, Token<'i>>;

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
        self.position(predicate)
    }

    fn slice_index(&self, _count: usize) -> Result<usize, nom::Needed> {
        todo!()
    }
}

impl Slice<Range<usize>> for TokenStream<'_> {
    fn slice(&self, range: Range<usize>) -> Self {
        Self::new(&self.tokens[range])
    }
}
impl Slice<RangeFrom<usize>> for TokenStream<'_> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        Self::new(&self.tokens[range])
    }
}
impl Slice<RangeTo<usize>> for TokenStream<'_> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        Self::new(&self.tokens[range])
    }
}
impl Slice<RangeFull> for TokenStream<'_> {
    fn slice(&self, range: RangeFull) -> Self {
        Self::new(&self.tokens[range])
    }
}
