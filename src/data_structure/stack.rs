use core::fmt::Display;
use std::fmt::Debug;
use std::mem::{self, MaybeUninit};
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

pub trait Stack<T> {
    fn push(&mut self, item: T) -> Result<(), PushError>;
    fn pop(&mut self) -> Result<T, PopError>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn capacity(&self) -> usize;
}

impl<T> Stack<T> for VecStack<T> {
    fn push(&mut self, item: T) -> Result<(), PushError> {
        if self.is_full() {
            return Err(PushError::IsFull);
        }

        self.items.push(item);
        Ok(())
    }

    fn pop(&mut self) -> Result<T, PopError> {
        self.items.pop().ok_or(PopError::IsEmpty)
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn is_full(&self) -> bool {
        self.capacity() <= self.items.len()
    }

    fn capacity(&self) -> usize {
        self.items.capacity()
    }
}

pub struct VecStack<T> {
    items: Vec<T>,
}

impl<T> VecStack<T> {
    pub fn new(capcacity: usize) -> Self {
        VecStack {
            items: Vec::with_capacity(capcacity),
        }
    }
}

const ARRAY_STACK_SIZE: usize = 256;

pub struct ArrayStack<T> {
    tail: usize,
    items: [MaybeUninit<T>; ARRAY_STACK_SIZE],
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        Self {
            tail: 0,
            items: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
}

impl<T> Stack<T> for ArrayStack<T> {
    fn push(&mut self, item: T) -> Result<(), PushError> {
        let target = self.items.get_mut(self.tail);
        match target {
            Some(target) => {
                unsafe {
                    *target.as_mut_ptr() = item;
                }
                self.tail += 1;
                Ok(())
            }
            None => Err(PushError::IsFull),
        }
    }
    fn pop(&mut self) -> Result<T, PopError> {
        if self.is_empty() {
            return Err(PopError::IsEmpty);
        }

        self.tail -= 1;
        let r = self.items.get_mut(self.tail).unwrap();

        let r = mem::replace(r, MaybeUninit::uninit());

        unsafe { Ok(r.assume_init()) }
    }
    fn is_empty(&self) -> bool {
        self.tail == 0
    }
    fn is_full(&self) -> bool {
        self.tail >= self.capacity()
    }
    fn capacity(&self) -> usize {
        ARRAY_STACK_SIZE
    }
}

fn compute(items: &[Ident]) -> Result<isize, Box<dyn Error>> {
    let mut stack = ArrayStack::new();
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
    fn test_array_stack() {
        let mut s = ArrayStack::new();
        assert!(s.is_empty());
        assert!(s.push(1).is_ok());
        assert!(s.push(2).is_ok());
        assert!(s.push(3).is_ok());

        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        assert_eq!(s.pop(), Ok(1));
        assert!(s.is_empty());
        assert!(!s.is_full());

        for x in 0..ARRAY_STACK_SIZE {
            assert!(s.push(x).is_ok());
        }
        assert!(s.is_full());
        assert_eq!(s.push(0), Err(PushError::IsFull));
        for x in (0..ARRAY_STACK_SIZE).rev() {
            assert_eq!(s.pop(), Ok(x));
        }

        assert_eq!(s.pop(), Err(PopError::IsEmpty));
    }

    #[test]
    fn test_vec_stack() {
        let mut s = VecStack::new(3);
        assert!(s.is_empty());
        assert!(!s.is_full());
        assert!(s.push(1).is_ok());
        assert!(s.push(2).is_ok());
        assert!(s.push(3).is_ok());
        assert!(s.is_full());
        assert_eq!(s.push(0), Err(PushError::IsFull));

        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        assert_eq!(s.pop(), Ok(1));
        assert!(s.is_empty());
        assert_eq!(s.pop(), Err(PopError::IsEmpty));
    }

    #[test]
    fn test1() {
        let input = "1 2 + 3 4 - *".to_string();
        let mut output = vec![];

        let result = input_stack(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "-3".to_string());
    }
}
