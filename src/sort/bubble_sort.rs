use itertools::Itertools;
use std::{error::Error, io::prelude::*};

type Int = usize;
pub fn bubble_sort(seq: &mut Vec<Int>) -> usize {
    macro_rules! get {
        ($i:expr) => {
            seq.get($i).unwrap()
        };
    }

    let mut is_continue = true;
    let mut swap_count = 0;

    macro_rules! swap {
        ($i:expr, $j:expr) => {
            seq.swap($i, $j);
            swap_count += 1;
        };
    }

    while is_continue {
        is_continue = false;
        for i in (1..(seq.len())).rev() {
            if get!(i) < get!(i - 1) {
                println!("{:?}", seq);
                swap!(i, i - 1);
                is_continue = true;
            }
        }
    }

    swap_count
}

pub fn input_bubble_sort(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<usize, Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut buf = buf.lines();
    let _len: usize = buf.next().unwrap().parse()?;
    let mut seq: Vec<Int> = buf
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let swap_conunt = bubble_sort(&mut seq);

    let output: String = seq.into_iter().map(|x| x.to_string()).join(" ");
    writer.write(output.as_bytes())?;

    Ok(swap_conunt)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let input: String = vec!["5", "5 3 2 4 1"].join("\n");
        let mut output = vec![];
        let result = input_bubble_sort(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8);
        assert_eq!(String::from_utf8(output).unwrap(), "1 2 3 4 5");
    }
}
