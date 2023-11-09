mod r#macro;

use std::collections::HashMap;

use crate::{preprocessor::SpanType, span::Span};

pub use r#macro::Macro;

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
                    self.defines.insert(mac.name(), mac);
                } else {
                    todo!("preprocessor directive {macro_type} not implemented");
                }
            } else if let Some(r#macro) = self.defines.get(span.get()) {
                println!("Found macro {macro}");
                if r#macro.is_function_like() {
                    let mut arguments = Vec::with_capacity(r#macro.args());
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
                    
                    // TODO: Should this be reversed?
                    self.r#macro = r#macro.apply(arguments);
                } else {
                    
                    // TODO: Should this be reversed?
                    self.r#macro = r#macro.value();
                }

                return self.next();
            } else {
                return Some(span);
            }
        }

        None
    }
}
