use core::fmt::Display;
use std::{error::Error, io::prelude::*};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Ident {
    Int(isize),
    Op(Op),
}

impl Ident {
    pub fn from(s: &str) -> Option<Self> {
        if s.len() == 1 {
            if let Some(op) = Op::from(&s.chars().next().unwrap()) {
                return Some(Ident::Op(op));
            }
        }

        s.parse().map(Ident::Int).ok()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Op {
    Plus,
    Minus,
    Mul,
}

impl Op {
    pub fn from(op: &char) -> Option<Self> {
        match op {
            &'+' => Some(Op::Plus),
            &'-' => Some(Op::Minus),
            &'*' => Some(Op::Mul),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum PushError {
    IsFull,
}

impl Display for PushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "PushError: {}", self.to_string())
    }
}
impl Error for PushError {}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum PopError {
    IsEmpty,
}

impl Display for PopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "PopError: {}", self.to_string())
    }
}
impl Error for PopError {}

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(capcacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capcacity),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), PushError> {
        if self.is_full() {
            return Err(PushError::IsFull);
        }
        self.items.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, PopError> {
        if self.is_empty() {
            return Err(PopError::IsEmpty);
        }
        Ok(self.items.pop().unwrap())
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.items.len() == self.items.capacity()
    }
}

const STACK_SIZE: usize = 16;

fn compute(items: &[Ident]) -> Result<isize, Box<dyn Error>> {
    let mut stack = Stack::new(STACK_SIZE);
    for item in items {
        match item {
            Ident::Int(item) => {
                if let Err(e) = stack.push(*item) {
                    return Err(Box::new(e));
                }
            }
            Ident::Op(op) => match (stack.pop(), stack.pop()) {
                (Ok(op1), Ok(op2)) => match op {
                    Op::Plus => stack.push(op2 + op1),
                    Op::Minus => stack.push(op2 - op1),
                    Op::Mul => stack.push(op2 * op1),
                }?,
                (_, Err(e)) => return Err(Box::new(e)),
                (Err(e), _) => return Err(Box::new(e)),
            },
        }
    }

    Ok(stack.pop()?)
}

pub fn input_stack(reader: &mut impl Read, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let items: Vec<Ident> = buf
        .split_whitespace()
        .map(|s| Ident::from(s).unwrap())
        .collect();
    let result = compute(&items)?;
    write!(writer, "{}", result)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let input = "1 2 + 3 4 - *".to_string();
        let mut output = vec![];

        let result = input_stack(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "-3".to_string());
    }
}
