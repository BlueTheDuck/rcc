pub struct Span<'i, X = ()> {
    input: &'i str,
    start: usize,
    end: usize,

    pub extra: X,
}
impl<'i, X> Span<'i, X>
where
    X: Default,
{
    pub(crate) fn new(input: &'i str, start: usize, end: usize) -> Self {
        Self::new_with(input, start, end, Default::default())
    }
    pub(crate) fn new_remaining(input: &'i str, start: usize) -> Self {
        Self::new_remaining_with(input, start, Default::default())
    }
}
impl<'i, X> Span<'i, X> {
    pub(crate) fn new_with(input: &'i str, start: usize, end: usize, extra: X) -> Self {
        assert!(start <= end, "Attempted to create `Span` with start > end. {start} > {end}");
        assert!(end <= input.len(), "Attempted to create `Span` with end > input.len(). {end} > {}", input.len());
        Self {
            input,
            start,
            end,
            extra,
        }
    }

    pub(crate) fn new_remaining_with(input: &'i str, start: usize, extra: X) -> Self {
        Self::new_with(input, start, input.len(), extra)
    }

    pub fn with<Y>(self, extra: Y) -> Span<'i, Y> {
        Span {
            input: self.input,
            start: self.start,
            end: self.end,
            extra,
        }
    }

    pub fn get(&self) -> &'i str {
        self.input.get(self.start..self.end).unwrap()
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    #[must_use]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    #[must_use]
    pub fn end(&self) -> usize {
        self.end
    }
}
impl<'i, X> std::fmt::Display for Span<'i, X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = self.input.get(self.start..self.end).unwrap();
        use termion::color::*;

        write!(f, "Span({}{slice:?}{})", Fg(Green), Fg(Reset))
    }
}
impl<'i, R, X> PartialEq<R> for Span<'i, X>
where
    R: AsRef<str>,
{
    fn eq(&self, other: &R) -> bool {
        self.get() == other.as_ref()
    }
}
impl<'i, X> PartialEq for Span<'i, X> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl<'i, X> Eq for Span<'i, X> {}
impl<'i, X> std::hash::Hash for Span<'i, X> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}
impl<'i, X> Copy for Span<'i, X> where X: Copy {}
impl<'i, X> Clone for Span<'i, X>
where
    X: Clone,
{
    fn clone(&self) -> Self {
        Self {
            input: self.input,
            start: self.start,
            end: self.end,
            extra: self.extra.clone(),
        }
    }
}
impl<'i, X: std::fmt::Debug> std::fmt::Debug for Span<'i, X>
where
    X: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("input", &self.input)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("extra", &self.extra)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span() {
        let input = "hello world";
        let span = Span::new_with(input, 0, 5, ());
        assert_eq!(span, "hello");
        assert_eq!(span.len(), 5);
    }
}
