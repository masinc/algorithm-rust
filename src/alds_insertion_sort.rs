use itertools::Itertools;
use std::{error::Error, io::prelude::*};

type Int = isize;

fn insersion_sort_core(seq: &mut Vec<Int>, progress: &mut Option<Vec<Vec<Int>>>) {
    macro_rules! get {
        ($i:expr) => {
            seq.get($i as usize).unwrap()
        };
    }
    macro_rules! update {
        ($i:expr, $j:expr) => {
            *seq.get_mut($i as usize).unwrap() = get!($j as usize).clone();
        };
    }

    macro_rules! set {
        ($i:expr, $x:ident) => {
            *seq.get_mut($i as usize).unwrap() = $x;
        };
    }

    for i in dbg!(1..(seq.len() as isize)) {
        if let Some(progress) = progress {
            progress.push(seq.clone());
        }

        let v = get!(i).clone();
        let mut j: isize = i - 1;
        while j >= 0 && get!(j) > &v {
            update!(j + 1, j);
            j -= 1;
        }
        set!(j + 1, v);
    }
    if let Some(progress) = progress {
        progress.push(seq.clone());
    }
}

pub fn insertion_sort(seq: &mut Vec<Int>) {
    insersion_sort_core(seq, &mut None);
}

pub fn insersion_sort2(seq: &mut Vec<Int>) -> Vec<Vec<Int>> {
    let mut result = Some(Vec::with_capacity(seq.len()));
    insersion_sort_core(seq, &mut result);
    result.unwrap()
}

pub fn input_insertion_sort(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<Vec<Vec<isize>>, Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let mut buf = buf.split("\n");
    let _len: usize = buf.next().unwrap().parse()?;
    let mut seq: Vec<Int> = buf
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let r = insersion_sort2(&mut seq);
    let output: String = seq.into_iter().map(|x| x.to_string()).join(" ");
    writer.write(output.as_bytes())?;

    Ok(r)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = vec!["6", "5 2 4 6 1 3"].join("\n");
        let mut output = vec![];
        let result = input_insertion_sort(&mut input.as_bytes(), &mut output);

        assert_eq!(String::from_utf8(output).unwrap(), "1 2 3 4 5 6");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                vec![5, 2, 4, 6, 1, 3],
                vec![2, 5, 4, 6, 1, 3],
                vec![2, 4, 5, 6, 1, 3],
                vec![2, 4, 5, 6, 1, 3],
                vec![1, 2, 4, 5, 6, 3],
                vec![1, 2, 3, 4, 5, 6],
            ]
        );
    }
}
