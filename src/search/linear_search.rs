use std::{
    error::Error,
    io::{prelude::*, BufReader},
};

use itertools::{Either, Itertools};

pub trait LinearSearch<T, TIndex> {
    fn linear_search(&self, value: &T) -> Option<TIndex>;
    fn linear_search_with(&self, value: &T, start: TIndex) -> Option<TIndex>;
}

impl<T: PartialEq> LinearSearch<T, usize> for [T] {
    fn linear_search(&self, value: &T) -> Option<usize> {
        self.linear_search_with(value, 0)
    }
    fn linear_search_with(&self, value: &T, start: usize) -> Option<usize> {
        let mut index = start;
        for x in &self[start..] {
            if x == value {
                return Some(index);
            }
            index += 1;
        }

        None
    }
}

pub fn input_linear_search(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    let _len: usize = lines.next().unwrap()?.parse()?;
    let (values, errors): (Vec<_>, Vec<_>) = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .partition_map(|r| match r {
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
        if let Some(_) = values.linear_search(&target) {
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

        let result = input_linear_search(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "3");
    }
}
