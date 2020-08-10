use itertools::{Either, Itertools};
use std::{error::Error, io::prelude::*};

type Int = usize;
pub fn selection_sort(seq: &mut Vec<Int>) -> usize {
    macro_rules! get {
        ($i:expr) => {
            seq.get($i).unwrap()
        };
    }
    let len = seq.len();

    let mut swap_count = 0;

    for i in 0..len {
        let mut minj = i;
        for j in i..len {
            if get!(j) < get!(minj) {
                minj = j;
            }
        }
        if i != minj {
            seq.swap(i, minj);
            swap_count += 1;
        }
    }

    swap_count
}

pub fn input_selection_sort(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let mut buf = buf.split("\n");
    let _length: usize = buf.next().unwrap().parse()?;
    let (mut seq, err): (Vec<Int>, Vec<_>) =
        buf.next()
            .unwrap()
            .split_whitespace()
            .partition_map(|x| match x.parse::<Int>() {
                Ok(x) => Either::Left(x),
                Err(e) => Either::Right(e),
            });
    if err.len() > 0 {
        return Err(Box::new(err.get(0).unwrap().clone()));
    }
    let swap_count = selection_sort(&mut seq).to_string();

    let output: String = [seq.into_iter().map(|x| x.to_string()).join(" "), swap_count].join("\n");

    writer.write_all(output.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = ["6", "5 6 4 2 1 3"].join("\n");
        let mut output = vec![];

        assert!(input_selection_sort(&mut input.as_bytes(), &mut output).is_ok());

        assert_eq!(
            String::from_utf8(output).unwrap(),
            vec!["1 2 3 4 5 6", "4"].join("\n")
        );
    }
}
