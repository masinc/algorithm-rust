use itertools::{Either, Itertools};
use std::{
    error::Error,
    io::{prelude::*, BufReader},
};
pub trait BinarySearch<T, TIndex> {
    fn binary_search2(&self, value: &T) -> Option<TIndex>;
}

impl<T: PartialEq + PartialOrd> BinarySearch<T, usize> for [T] {
    fn binary_search2(&self, value: &T) -> Option<usize> {
        let middle = self.len() - 1;
        match self.get(middle).unwrap() {
            x if x == value => Some(middle),
            x if x > value => self[..middle].binary_search2(value),
            x if x < value => self[middle..].binary_search2(value),
            _ => None,
        }
    }
}

pub fn input_binary_search(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let mut lines = BufReader::new(reader).lines();
    let _len: usize = lines.next().unwrap()?.parse()?;
    let (values, errors): (Vec<_>, Vec<_>) = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .partition_map(|x| match x.parse::<usize>() {
            Ok(ok) => Either::Left(ok),
            Err(e) => Either::Right(e),
        });

    if !errors.is_empty() {
        return Err(Box::new(errors.first().unwrap().clone()));
    }

    let _len: usize = lines.next().unwrap()?.parse()?;
    let (targets, errors): (Vec<_>, Vec<_>) = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .partition_map(|x| match x.parse::<usize>() {
            Ok(ok) => Either::Left(ok),
            Err(e) => Either::Right(e),
        });

    if !errors.is_empty() {
        return Err(Box::new(errors.first().unwrap().clone()));
    }

    let mut output = 0;
    for target in targets {
        if let Some(_) = values.binary_search2(&target) {
            output += 1;
        }
    }
    write!(writer, "{}", output)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = ["5", "1 2 3 4 5", "3", "3 4 1"].join("\n");
        let mut output: Vec<u8> = vec![];

        let result = input_binary_search(&mut input.as_bytes(), &mut output);
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "3");
    }
}
