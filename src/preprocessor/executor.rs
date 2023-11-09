use std::collections::{HashMap, HashSet};

use crate::{preprocessor::SpanType, span::Span};

struct Macro<'i> {
    name: Span<'i>,
    args: HashSet<Span<'i>>,
    body: Vec<Span<'i, SpanType>>,
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
                args = HashSet::new();

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
                            debug_assert!(args.insert(span.with(())), "Repeated argument names");
                        }
                        _ => {
                            todo!("unexpected token {span}");
                        }
                    }
                }
            }
            _ => {
                args = HashSet::with_capacity(0);
            }
        }

        // take body
        while let Some(span) = iter.next() {
            match span {
                span if span == "\n" => {
                    break;
                }
                _ => {
                    body.push(span);
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

pub struct PreprocessorExecutor<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    iter: I,
    defines: HashMap<&'i str, Macro<'i>>,
}
impl<'i, I> PreprocessorExecutor<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            defines: Default::default(),
        }
    }
}
impl<'i, I> Iterator for PreprocessorExecutor<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    type Item = Span<'i, SpanType>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(span) = self.iter.next() {
            if span == "#" {
                let macro_type = self.iter.next().unwrap();
                if macro_type == "define" {
                    let mac = Macro::new_from(&mut self.iter);
                    self.defines.insert(mac.name.get(), mac);
                } else {
                    todo!("preprocessor directive {macro_type} not implemented");
                }
            } else if self.defines.contains_key(span.get()) {
                println!("Found {span}");
            } else {
                return Some(span);
            }
        }

        None
    }
}
