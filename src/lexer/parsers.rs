use super::Span;

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

fn parse_tokens(source: &str) -> Vec<Span> {
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

pub fn parse_program(i: &str) -> Vec<Span> {
    let program = parse_tokens(i);

    program
}
