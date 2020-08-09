use std::fmt::Debug;
use std::str::FromStr;

pub fn bubble_sort<T: Ord + FromStr + Clone + Debug>(seq: &mut Vec<T>) -> usize
where
    <T as FromStr>::Err: Debug,
{
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

pub fn input_bubble_sort<T: Ord + FromStr + Clone + Debug>(
    input: &mut impl Iterator<Item = String>,
) -> (Vec<T>, usize)
where
    <T as FromStr>::Err: Clone + Debug,
{
    let _len: usize = input.next().unwrap().parse().unwrap();
    let mut seq: Vec<T> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let swap_count = bubble_sort(&mut seq);
    (seq, swap_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let input: Vec<String> = vec!["5", "5 3 2 4 1"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = input_bubble_sort(&mut input.into_iter());

        assert_eq!(result, (vec![1, 2, 3, 4, 5], 8))
    }
}
