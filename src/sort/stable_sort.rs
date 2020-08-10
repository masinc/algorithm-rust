use itertools::Itertools;
use std::{error::Error, io::prelude::*, str::FromStr};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Trump {
    number: usize,
    suit: char,
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum TrumpParsingError {
    ParseError,
}

impl FromStr for Trump {
    type Err = TrumpParsingError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        if s.len() != 2 {
            return Err(TrumpParsingError::ParseError);
        }

        let mut c = s.chars();
        let suit = c.next().ok_or(TrumpParsingError::ParseError)?;
        let number = c.next().ok_or(TrumpParsingError::ParseError)?;
        let number: usize = number
            .to_string()
            .parse()
            .map_err(|_| TrumpParsingError::ParseError)?;

        Ok(Trump { suit, number })
    }
}

impl ToString for Trump {
    fn to_string(&self) -> String {
        format!("{}{}", self.suit, self.number)
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum SortStable {
    Stable,
    UnStable,
}

impl ToString for SortStable {
    fn to_string(&self) -> std::string::String {
        match self {
            SortStable::Stable => "Stable",
            SortStable::UnStable => "UnStable",
        }
        .into()
    }
}

pub fn is_stable(target: &Vec<Trump>, stable: &Vec<Trump>) -> SortStable {
    macro_rules! get {
        ($v:ident, $i:expr) => {
            $v.get($i).unwrap()
        };
    };
    println!("target:{:?}", target);
    println!("stable:{:?}", stable);
    let len = target.len();
    for i in 0..len {
        if get!(target, i).suit != get!(stable, i).suit {
            return SortStable::UnStable;
        }
    }

    SortStable::Stable
}

pub fn bubble_sort(trump: &mut Vec<Trump>) {
    macro_rules! get {
        ($i:expr) => {
            trump.get($i).unwrap()
        };
    }
    let len = trump.len();
    for i in 0..len {
        for j in ((i + 1)..len).rev() {
            if get!(j) < get!(j - 1) {
                trump.swap(j, j - 1)
            }
        }
    }
}

pub fn selection_sort(trump: &mut Vec<Trump>) {
    macro_rules! get {
        ($i:expr) => {
            trump.get($i).unwrap()
        };
    }

    let len = trump.len();

    let mut minj;
    for i in 0..len {
        minj = i;
        for j in (i..len).rev() {
            if get!(j) < get!(minj) {
                minj = j;
            }
        }
        if i != minj {
            trump.swap(i, minj);
        }
    }
}

pub fn input_stable_sort(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let mut buf = buf.lines();
    let _len: usize = buf.next().unwrap().parse()?;
    let seq: Vec<Trump> = buf
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut bubble_seq = seq.clone();
    bubble_sort(&mut bubble_seq);

    let mut selection_seq = seq.clone();
    selection_sort(&mut selection_seq);
    let bubble_sort_stable = SortStable::Stable.to_string();
    let selection_sort_stable = is_stable(&selection_seq, &bubble_seq).to_string();
    let bubble_seq: String = bubble_seq.iter().map(|t| t.to_string()).join(" ");
    let selection_seq: String = selection_seq.iter().map(|t| t.to_string()).join(" ");

    let output = [
        bubble_seq,
        bubble_sort_stable,
        selection_seq,
        selection_sort_stable,
    ]
    .join("\n");

    writer.write(output.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let input = vec!["5", "H4 C9 S4 D2 C3"].join("\n");
        let mut output = vec![];

        let result = input_stable_sort(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            // vec!["D2 C3 H4 S4 C9", "Stable", "D2 C3 S4 H4 C9", "Not Stable"].join("\n")
            vec!["D2 C3 H4 S4 C9", "Stable", "D2 C3 H4 S4 C9", "Stable"].join("\n")
        )
    }
}
