use crate::span::Span;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(derive_more::IsVariant)]
pub enum SpanType {
    Whitespace,

    Comment,

    /// Literal values such as numbers
    /// except string
    Literal,

    /// String literal
    String,

    /// Includes identifiers and keywords
    Identifier,

    Punctuation,

    Operator,

    #[default]
    None,
}

/// Takes a source string and an offset, then returns the next span that is a valid
/// preprocessing token.
pub(crate) fn take_preprocessing_seq(source: &str, start: usize) -> Option<Span<SpanType>> {
    let mut iter = source.char_indices().skip(start).peekable();
    let (index, char) = iter.next()?;
    let next = iter.peek().map(|&c| c.1);
    match char {
        // Combine operators
        '+' | '-' | '<' | '>' | '=' => {
            if let Some(next) = next {
                if next == char || next == '=' {
                    return Some(Span::new_with(source, start, index + 2, SpanType::Operator));
                } else {
                    return Some(Span::new_with(source, start, index + 1, SpanType::Operator));
                }
            } else {
                return Some(Span::new(source, start, index + 1));
            }
        }
        // Punctuation
        '(' | ')' | '{' | '}' | ',' | '#' | '?' | ':' | ';' => {
            return Some(Span::new_with(source, index, index + 1, SpanType::Punctuation));
        }
        '"' => {
            let end = take_string(&mut iter);
            return Some(Span::new_with(source, start, end, SpanType::String));
        }
        c if c.is_ascii_whitespace() => {
            if let Some((next, _)) = iter.skip_while(|(_, c)| c.is_ascii_whitespace()).next() {
                return Some(Span::new_with(source, start, next,SpanType::Whitespace));
            } else {
                return Some(Span::new_remaining_with(source, start, SpanType::Whitespace));
            }
        }
        c if crate::is_valid_for_ident(c) => {
            if let Some((next, _)) = iter
                .skip_while(|(_, c)| crate::is_valid_for_ident(*c))
                .next()
            {
                return Some(Span::new_with(source, start, next, SpanType::Identifier));
            } else {
                return Some(Span::new_remaining_with(source, start, SpanType::Identifier));
            }
        }
        x => {
            todo!("Found char {x}")
        }
    }
}
