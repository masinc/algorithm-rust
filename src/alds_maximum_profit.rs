type Int = isize;

pub fn get_max_profit(mut input: impl Iterator<Item = Int>) -> Option<Int> {
    let len = input.next().unwrap_or_else(|| 0);

    if [0, 1].contains(&len) {
        return None;
    }

    let mut min: Int;
    min = input.next().unwrap();

    let mut result: Option<Int> = None;

    for r in input {
        let dif = r - min;
        result = result.map_or(dif, |cur| cur.max(dif)).into();
        min = min.min(r);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![6, 5, 3, 1, 3, 4, 3];

        assert_eq!(Some(3), get_max_profit(input.into_iter()));
    }

    #[test]
    fn test2() {
        let input = vec![3, 4, 3, 2];
        assert_eq!(Some(-1), get_max_profit(input.into_iter()));
    }
}
