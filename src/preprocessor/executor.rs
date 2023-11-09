use std::collections::{HashMap, HashSet};

use crate::{preprocessor::SpanType, span::Span};

struct Macro<'i> {
    name: Span<'i>,
    args: Vec<Span<'i>>,
    body: Vec<Span<'i, SpanType>>,
}
impl<'i> Macro<'i> {
    fn new_from(iter: &mut impl Iterator<Item = Span<'i, SpanType>>) -> Self {
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

    fn apply(&self, args: Vec<Vec<Span<'i, SpanType>>>) -> Vec<Span<'i, SpanType>> {
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
    r#macro: Vec<Span<'i, SpanType>>,
}
impl<'i, I> PreprocessorExecutor<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            defines: Default::default(),
            r#macro: Vec::with_capacity(0),
        }
    }
}
impl<'i, I> Iterator for PreprocessorExecutor<'i, I>
where
    I: Iterator<Item = Span<'i, SpanType>>,
{
    type Item = Span<'i, SpanType>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(token) = self.r#macro.pop() {
            return Some(token);
        }
        while let Some(span) = self.iter.next() {
            if span == "#" {
                let macro_type = self.iter.next().unwrap();
                if macro_type == "define" {
                    let mac = Macro::new_from(self);
                    println!("Defining new macro {mac}");
                    self.defines.insert(mac.name.get(), mac);
                } else {
                    todo!("preprocessor directive {macro_type} not implemented");
                }
            } else if let Some(r#macro) = self.defines.get(span.get()) {
                println!("Found macro {macro}");
                let mut arguments = Vec::with_capacity(r#macro.args.len());
                if r#macro.is_function_like() {
                    let mut argument = Vec::new();

                    while let Some(span) = self.iter.next() {
                        if span.extra.is_whitespace() {
                            continue;
                        } else if span == "(" {
                            break;
                        } else {
                            todo!("unexpected token {span}");
                        }
                    }

                    while let Some(span) = self.iter.next() {
                        if span == ")" {
                            arguments.push(argument);
                            break;
                        } else if span == "," {
                            arguments.push(argument);
                            argument = Vec::new();
                            continue;
                        } else {
                            argument.push(span);
                        }
                    }
                }

                println!("Applying macro {} with arguments {}", r#macro, arguments.len());

                self.r#macro = r#macro.apply(arguments);

                return self.next();
            } else {
                return Some(span);
            }
        }

        None
    }
}
