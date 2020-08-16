use itertools::Itertools;
use std::{
    error::Error,
    io::{prelude::*, BufReader},
};

pub fn compute(seq: &[usize], targets: &[usize]) -> Result<Vec<bool>, Box<dyn Error>> {
    let combinations: Vec<Vec<Vec<&usize>>> = (0..seq.len())
        .map(|i| seq.iter().combinations(i + 1).into_iter().collect())
        .collect();
    let compute_inner = |target| {
        for i in 0..(seq.len()) {
            for c in combinations.get(i).unwrap() {
                if target == c.iter().map(|x| *x).sum::<usize>() {
                    return true;
                }
            }
        }
        return false;
    };

    let mut result = Vec::with_capacity(targets.len());
    for target in targets {
        result.push(compute_inner(*target));
    }

    Ok(result)
}

pub fn input_exhaustive_search(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();
    let _len: usize = lines.next().unwrap()?.parse()?;
    let seq: Vec<usize> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let _len: usize = lines.next().unwrap()?.parse()?;
    let target: Vec<usize> = lines
        .next()
        .unwrap()?
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = compute(&seq[..], &target[..])?;
    let result: String = result
        .into_iter()
        .map(|x| if x { "yes" } else { "no" })
        .join("\n");
    write!(writer, "{}", result)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = ["5", "1 5 7 10 21", "4", "2 4 17 8"].join("\n");
        let mut output: Vec<u8> = vec![];

        let result = input_exhaustive_search(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            ["no", "no", "yes", "yes"].join("\n")
        );
    }
}
