use std::collections::HashMap;

use crate::{span::Span, preprocessor::parser::SpanType};

pub struct Macro<'i> {
    name: Span<'i>,
    args: Vec<Span<'i>>,
    body: Vec<Span<'i, SpanType>>,
}
impl<'i> Macro<'i> {
    pub(crate) fn new_from(iter: &mut impl Iterator<Item = Span<'i, SpanType>>) -> Self {
        // iter.next(); // skip whitespace
        let name = iter
            .skip_while(|s| s.extra.is_whitespace())
            .next()
            .filter(|s| s.extra.is_identifier())
            .expect("Expected identifier");
        let mut args;
        let mut body = Vec::new();

        // Does it have arguments?

        match iter.skip_while(|s| s.extra.is_whitespace()).next() {
            Some(span) if span == "(" => {
                args = Vec::new();

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
                            // debug_assert!(args.insert(span.with(())), "Repeated argument names");
                        }
                        _ => {
                            todo!("unexpected token {span}");
                        }
                    }
                }
            }
            Some(span) => {
                args = Vec::with_capacity(0);
                body.push(span);
            }
            None => {
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

    pub fn apply(&self, args: Vec<Vec<Span<'i, SpanType>>>) -> Vec<Span<'i, SpanType>> {
        assert_eq!(self.args.len(), args.len());

        let mut body = Vec::with_capacity(self.body.len());
        let mut arg_map = HashMap::with_capacity(self.args.len());

        for (index, name) in self.args.iter().enumerate() {
            arg_map.insert(name.get(), index);
        }

        for body_token in &self.body {
            if let Some(argument) = arg_map.get(body_token.get()).map(|&index| &args[index]) {
                for t in argument {
                    body.push(*t);
                }
            } else {
                body.push(*body_token);
            }
        }

        body.reverse();

        body
    }


    pub fn name(&self) -> &'i str {
        self.name.get()
    }
    pub fn args(&self) -> usize {
        self.args.len()
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

