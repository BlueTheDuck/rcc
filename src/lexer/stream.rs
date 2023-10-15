use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

use nom::{InputIter, InputLength, InputTake, Needed, Slice};

use super::token::Token;

#[derive(Clone, Copy, Debug)]
pub struct TokenStream<'i> {
    pub(crate) tokens: &'i [Token<'i>],
}

impl<'i> TokenStream<'i> {
    pub const fn new(tokens: &'i [Token<'i>]) -> Self {
        Self { tokens }
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
        let (pre, post) = self.tokens.split_at(count);
        (Self::new(post), Self::new(pre))
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

impl Slice<Range<usize>> for TokenStream<'_> {
    fn slice(&self, _range: Range<usize>) -> Self {
        todo!()
    }
}
impl Slice<RangeFrom<usize>> for TokenStream<'_> {
    fn slice(&self, _range: RangeFrom<usize>) -> Self {
        todo!()
    }
}
impl Slice<RangeTo<usize>> for TokenStream<'_> {
    fn slice(&self, _range: RangeTo<usize>) -> Self {
        todo!()
    }
}
impl Slice<RangeFull> for TokenStream<'_> {
    fn slice(&self, _range: RangeFull) -> Self {
        *self
    }
}
