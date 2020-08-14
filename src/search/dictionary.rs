use itertools::Itertools;
use std::{
    error::Error,
    io::{prelude::*, BufReader},
};

pub trait Dictionary<T: PartialEq, UIndex> {
    fn insert(&mut self, value: T);
    fn find(&self, value: &T) -> Option<UIndex>;
}

#[derive(Debug, Hash, PartialEq)]
struct VecDictionary<T: PartialEq> {
    items: Vec<T>,
}

impl<T: PartialEq> VecDictionary<T> {
    pub fn new(capaticy: usize) -> Self {
        Self {
            items: Vec::with_capacity(capaticy),
        }
    }
}

impl<T: PartialEq + Ord> Dictionary<T, usize> for VecDictionary<T> {
    fn insert(&mut self, value: T) {
        self.items.push(value);
        self.items.sort();
    }

    fn find(&self, value: &T) -> Option<usize> {
        self.items.binary_search(value).ok()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Command {
    Insert(String),
    Find(String),
}

impl Command {
    fn new(command_type: &String, value: String) -> Option<Self> {
        match command_type.as_str() {
            "insert" => Some(Command::Insert(value)),
            "find" => Some(Command::Find(value)),
            _ => None,
        }
    }
}

pub fn compute(commands: &[Command]) -> Result<Vec<bool>, Box<dyn Error>> {
    let mut result = vec![];

    let mut dict = VecDictionary::new(commands.len());
    for command in commands {
        match command {
            Command::Insert(x) => {
                dict.insert(x);
            }
            Command::Find(x) => {
                let r = dict.find(&x).map(|_| true).unwrap_or(false);
                result.push(r);
            }
        }
    }

    Ok(result)
}

pub fn input_dictinary(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    let len: usize = lines.next().unwrap()?.parse()?;
    let mut commands = Vec::with_capacity(len);

    for line in lines.map(|l| l.unwrap()) {
        let mut line = line.split_whitespace();
        let command_type = line.next().unwrap().to_string();
        let x = line.next().unwrap().to_string();
        commands.push(Command::new(&command_type, x).unwrap());
    }

    let result = compute(&commands[..])?;

    write!(
        writer,
        "{}",
        result
            .into_iter()
            .map(|b| if b { "yes" } else { "no" })
            .join("\n")
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let input = [
            "6",
            "insert AAA",
            "insert AAC",
            "find AAA",
            "find CCC",
            "insert CCC",
            "find CCC",
        ]
        .join("\n");

        let mut output: Vec<u8> = vec![];

        let result = input_dictinary(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            ["yes", "no", "yes"].join("\n")
        );
    }
}
