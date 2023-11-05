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
where
    R: AsRef<str>,
{
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

fn is_valid_for_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn take_span(source: &str, start: usize) -> Option<Span> {
    let mut iter = source.char_indices().skip(start).peekable();
    let (index, char) = iter.next()?;
    let next = iter.peek().map(|&c| c.1);
    match char {
        // Combine operators
        '+' | '-' | '<' | '>' | '=' => {
            if let Some(next) = next {
                if next == char || next == '=' {
                    return Some(Span::new(source, start, index + 2));
                } else {
                    return Some(Span::new(source, start, index + 1));
                }
            } else {
                return Some(Span::new(source, start, index + 1));
            }
        }
        // Punctuation
        '(' | ')' | '{' | '}' | ',' | '#' | '?' | ':' | ';' => {
            return Some(Span::new(source, index, index + 1));
        }
        '"' => {
            let end = take_string(&mut iter);
            return Some(Span::new(source, start, end));
        }
        c if c.is_ascii_whitespace() => {
            if let Some((next, _)) = iter.skip_while(|(_, c)| c.is_ascii_whitespace()).next() {
                return Some(Span::new(source, start, next));
            } else {
                return Some(Span::new_remaining(source, start));
            }
        }
        c if is_valid_for_ident(c) => {
            if let Some((next, _)) = iter.skip_while(|(_, c)| is_valid_for_ident(*c)).next() {
                return Some(Span::new(source, start, next));
            } else {
                return Some(Span::new_remaining(source, start));
            }
        }
        x => {
            todo!("Found char {x}")
        }
    }
}

pub struct SpanIter<'i> {
    source: &'i str,
    start: usize,
}
impl<'i> SpanIter<'i> {
    pub fn new(source: &'i str) -> Self {
        Self { source, start: 0 }
    }
}
impl<'i> Iterator for SpanIter<'i> {
    type Item = Span<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(span) = take_span(self.source, self.start) {
                self.start = span.end;
                if span.len() > 0 {
                    break Some(span);
                }
            } else {
                break None;
            }
        }
    }
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
