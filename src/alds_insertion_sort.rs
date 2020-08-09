type Int = isize;
pub fn insersion_sort(input: &mut impl Iterator<Item = String>) -> Vec<Vec<Int>> {
    let len: usize = input.next().unwrap().parse().unwrap();
    let mut seq: Vec<Int> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

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

    let mut result = Vec::with_capacity(len);

    for i in dbg!(1..(seq.len() as isize)) {
        result.push(seq.clone());
        let v = get!(i).clone();
        let mut j: isize = i - 1;
        while j >= 0 && get!(j) > &v {
            update!(j + 1, j);
            j -= 1;
        }
        set!(j + 1, v);
    }
    result.push(seq);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input: Vec<String> = vec!["6", "5 2 4 6 1 3"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = insersion_sort(&mut input.into_iter());

        assert_eq!(
            result,
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
