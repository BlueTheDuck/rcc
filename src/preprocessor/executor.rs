use std::collections::HashSet;

use crate::span::Span;

use super::parser::SpanType;

struct Macro<'i> {
    name: Span<'i>,
    args: Vec<Span<'i>>,
    body: Vec<Span<'i>>,
}
impl<'i> Macro<'i> {
    fn new_from(iter: &mut impl Iterator<Item = Span<'i, SpanType>>) -> Self {
        iter.next(); // skip whitespace
        let name = iter.next().unwrap();
        let mut args;
        let mut body = Vec::new();

        // Does it have arguments?

        match iter.skip_while(|s| s.extra.is_whitespace()).next() {
            Some(span) if span == "(" => {
                args = Vec::new();
                body = Vec::new();
                // take arguments
                while let Some(span) = iter.skip_while(|s| s.extra.is_whitespace()).next() {
                    match span {
                        span if span == ")" => {
                            break;
                        }
                        span if span == "," => {
                            continue;
                        }
                        span if span.extra.is_identifier() => {
                            args.push(span.with(()));
                        }
                        _ => {
                            todo!("unexpected token {span}");
                        }
                    }
                }
            }
            _ => {
                args = Vec::with_capacity(0);
            }
        }

        // take body
        while let Some(span) = iter.next() {
            match span {
                span if span == "\n" => {
                    break;
                }
                _ => {
                    body.push(span.with(()));
                }
            }
        }

        Self {
            name: name.with(()),
            args,
            body,
        }
    }

    pub fn is_function_like(&self) -> bool {
        self.args.len() > 0
    }
    pub fn has_body(&self) -> bool {
        self.body.len() > 0
    }
}
impl<'i> core::fmt::Display for Macro<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#define {}", self.name.get())?;
        
        if self.is_function_like() {
            write!(f, "(")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", arg.get())?;
            }
            write!(f, ")")?;
        }
        if self.has_body() {
            for body in &self.body {
                write!(f, " {}", body.get())?;
            }
        }
        Ok(())
    }
}

pub fn execute_preprocessor<'i>(mut iter: impl Iterator<Item = Span<'i, SpanType>>) {
    let mut defines = HashSet::new();
    while let Some(span) = iter.next() {
        if span == "#" {
            let token_type = iter.next().unwrap();
            if token_type == "define" {
                let mac = Macro::new_from(&mut iter);
                println!("{mac}");
                defines.insert(mac.name.get());
            } else {
                todo!("preprocessor directive {token_type} not implemented");
            }
        } else if defines.contains(span.get()) {
            println!("Found {span}");
        } else {
        }
    }
}
