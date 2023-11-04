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

fn take_escape_seq<I>(iter: &mut I)
where
    I: Iterator<Item = (usize, char)>,
{
    let (_, c) = iter.next().unwrap();
    match c {
        '\\' | '\n' => {}
        _ => panic!("invalid escape sequence"),
    }
}

fn take_string<I>(iter: &mut I) -> usize
where
    I: Iterator<Item = (usize, char)>,
{
    loop {
        let (i, c) = iter.next().unwrap();
        match c {
            '\\' => take_escape_seq(iter),
            '"' => break i,
            '\n' => panic!("unterminated string literal"),
            _ => {}
        }
    }
}

fn take_operator<I>(iter: &mut I) -> usize
where
    I: Iterator<Item = (usize, char)>,
{
    loop {
        let (i, c) = iter.next().unwrap();
        match c {
            '\\' => take_escape_seq(iter),
            '"' => break i,
            '\n' => panic!("unterminated string literal"),
            _ => {}
        }
    }
}

pub(crate) fn split_source(source: &str) -> Vec<Span> {
    let mut start = 0usize;
    let mut spans = vec![];
    let mut iter = source.char_indices().peekable();
    loop {
        let (index, char) = match iter.by_ref().next() {
            Some(e) => e,
            None => {
                spans.push(Span::new_remaining(source, start));
                break;
            }
        };
        let next = iter.peek().map(|(_, c)| c).copied();
        match char {
            // Combine operators
            '+' | '-' | '<' | '>' | '=' => {
                spans.push(Span::new(source, start, index));
                start = index;
                if let Some(next) = next {
                    if next == char || next == '=' {
                        spans.push(Span::new(source, start, index + 2));
                        start = index + 2;
                        iter.next();
                    } else {
                        spans.push(Span::new(source, start, index + 1));
                        start = index + 1;
                    }
                } else {
                    spans.push(Span::new(source, start, index + 1));
                    start = index + 1;
                }
            }
            // Punctuation
            '(' | ')' | '{' | '}' | ',' => {
                spans.push(Span::new(source, start, index));
                spans.push(Span::new(source, index, index + 1));
                start = index + 1;
            }
            '"' => {
                let end = take_string(&mut iter);
                spans.push(Span::new(source, start, end));
                start = end + 1;
            }
            c if c.is_whitespace() => {
                spans.push(Span::new(source, start, index));
                start = index + 1;
            }
            _ => {}
        }
    }
    spans.retain(|span| span.len() > 0);

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span() {
        let input = "hello world";
        let span = Span::new(input, 0, 5);
        assert_eq!(span, "hello");
        assert_eq!(span.len(), 5);
    }
}
