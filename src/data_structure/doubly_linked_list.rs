use itertools::{Either, Itertools};
use std::{error::Error, fmt::Display, io::prelude::*};

#[derive(Debug, Copy, Clone, Hash)]
pub enum DoublyLinkedListError {
    IsEmpty,
    IsFull,
    ItemNotFound,
}

impl Display for DoublyLinkedListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DoublyLinkedListError: {}", self.to_string())
    }
}

impl Error for DoublyLinkedListError {}

pub trait DoublyLinkedList<T: PartialEq> {
    fn insert(&mut self, x: T) -> Result<(), DoublyLinkedListError>;
    fn delete(&mut self, x: &T) -> Result<(), DoublyLinkedListError>;
    fn delete_first(&mut self) -> Result<(), DoublyLinkedListError>;
    fn delete_last(&mut self) -> Result<(), DoublyLinkedListError>;
}

pub struct VecDoublyLinkedList<T> {
    items: Vec<T>,
}

impl<T> VecDoublyLinkedList<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T: PartialEq> DoublyLinkedList<T> for VecDoublyLinkedList<T> {
    fn insert(&mut self, x: T) -> Result<(), DoublyLinkedListError> {
        self.items.insert(0, x);
        Ok(())
    }
    fn delete(&mut self, x: &T) -> Result<(), DoublyLinkedListError> {
        if self.items.is_empty() {
            return Err(DoublyLinkedListError::IsEmpty);
        }

        for (index, item) in self.items.iter().enumerate() {
            if item == x {
                self.items.remove(index);
                return Ok(());
            }
        }

        Err(DoublyLinkedListError::ItemNotFound)
    }
    fn delete_first(&mut self) -> Result<(), DoublyLinkedListError> {
        if self.items.is_empty() {
            return Err(DoublyLinkedListError::IsEmpty);
        }

        self.items.remove(0);
        Ok(())
    }
    fn delete_last(&mut self) -> Result<(), DoublyLinkedListError> {
        if self.items.is_empty() {
            return Err(DoublyLinkedListError::IsEmpty);
        }

        self.items.pop();
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Command {
    Insert(usize),
    Delete(usize),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CommandError {
    UnsupportedType(String),
    InsertError,
    DeleteError,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CommandError {}

impl Command {
    fn new(command_type: String, num: Option<usize>) -> Result<Self, CommandError> {
        const COMMAND_INSERT: &str = "insert";
        const COMMAND_DELETE: &str = "delete";

        match (command_type.as_str(), num) {
            (COMMAND_INSERT, Some(num)) => Ok(Command::Insert(num)),
            (COMMAND_INSERT, None) => Err(CommandError::InsertError),
            (COMMAND_DELETE, Some(num)) => Ok(Command::Delete(num)),
            (COMMAND_DELETE, None) => Err(CommandError::DeleteError),
            (unsupport, _) => Err(CommandError::UnsupportedType(unsupport.into())),
        }
    }
}

pub fn compute(commands: &[Command]) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut result = VecDoublyLinkedList::new(commands.len());
    for cmd in commands {
        match cmd {
            &Command::Delete(x) => {
                result.delete(&x)?;
            }
            &Command::Insert(x) => {
                result.insert(x)?;
            }
        }
    }
    Ok(result.to_vec())
}

pub fn input_doubly_linked_list(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    use std::io::BufReader;
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut lines = buf.lines();

    let _len: usize = lines.next().unwrap().parse()?;

    let (commands, errors): (Vec<_>, Vec<_>) = lines
        .map(|l| l.split_whitespace())
        .map(|mut s| {
            Command::new(
                s.next().unwrap().into(),
                s.next().map(|x| x.parse::<usize>().ok()).flatten(),
            )
        })
        .partition_map(|x| match x {
            Ok(x) => Either::Left(x),
            Err(e) => Either::Right(e),
        });

    if !errors.is_empty() {
        return Err(Box::new(errors.get(0).unwrap().clone()));
    }
    let result = compute(&commands)?;
    writeln!(
        writer,
        "{}",
        result.into_iter().map(|x| x.to_string()).join(" ")
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = [
            "7", "insert 5", "insert 2", "insert 3", "insert 1", "delete 3", "insert 6", "delete 5",
        ]
        .join("\n");

        let mut output: Vec<u8> = vec![];

        let result = input_doubly_linked_list(&mut input.as_bytes(), &mut output);
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "6 1 2\n".to_string());
    }
}
