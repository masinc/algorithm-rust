use itertools::Itertools;
use std::error::Error;
use std::{
    cmp::Ordering,
    io::{prelude::*, BufReader},
};

type Int = usize;

pub fn merge_sort(seq: &mut [Int]) -> Result<usize, Box<dyn Error>> {
    fn merge(seq: &mut [Int], compare_count: &mut usize) {
        let len = seq.len();
        if [0, 1].contains(&len) {
            return;
        }
        let mid = len / 2;
        let mut left = vec![0; mid];
        let mut right = vec![0; len - mid];
        left.copy_from_slice(&seq[..mid]);
        right.copy_from_slice(&seq[mid..]);
        for x in seq.iter_mut() {
            *compare_count += 1;
            *x = match (left.first(), right.first()) {
                (Some(l), Some(r)) => match l.cmp(r) {
                    Ordering::Less | Ordering::Equal => left.remove(0),
                    Ordering::Greater => right.remove(0),
                },
                (Some(_), None) => left.remove(0),
                (None, Some(_)) => right.remove(0),
                (None, None) => unreachable!(),
            }
        }
    }

    fn inner_merge_sort(seq: &mut [Int], compare_count: &mut usize) {
        println!("{:0>2}:{:?}", compare_count, seq.to_vec());
        match seq.len() {
            0 | 1 => return,
            2 => {
                merge(seq, compare_count);
            }
            n => {
                let (left, right) = seq.split_at_mut(n / 2);
                inner_merge_sort(left, compare_count);
                inner_merge_sort(right, compare_count);

                merge(seq, compare_count);
            }
        }
    }

    let mut r = 0;
    inner_merge_sort(seq, &mut r);

    Ok(r)
}

pub fn input_merge_sort(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    let _len = lines.next().unwrap()?;
    let mut seq: Vec<usize> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let compare_count = merge_sort(&mut seq[..])?;

    writeln!(
        writer,
        "{}",
        seq.into_iter().map(|x| x.to_string()).join(" ")
    )?;

    writeln!(writer, "{}", compare_count)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let input = ["10", "8 5 9 2 6 3 7 1 10 4"].join("\n");
        let mut output: Vec<u8> = vec![];

        let result = input_merge_sort(&mut input.as_bytes(), &mut output);
        assert!(dbg!(result).is_ok());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            ["1 2 3 4 5 6 7 8 9 10", "34", ""].join("\n")
        );
    }
}
