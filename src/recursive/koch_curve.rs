use std::fmt::Debug;
use std::{error::Error, f64, io::prelude::*};

type PointF = Point<f64>;
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point<T>
where
    T: PartialEq + Clone + Debug,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: PartialEq + Clone + Debug,
{
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

fn compute(start: &PointF, end: &PointF, depth: usize) -> Vec<PointF> {
    fn compute_left(start: &PointF, end: &PointF) -> PointF {
        Point::new(
            start.x + ((end.x - start.x) / 3.),
            start.y + ((end.y - start.y) / 3.),
        )
    }

    fn compute_right(start: &PointF, end: &PointF) -> PointF {
        Point::new(
            start.x + ((end.x - start.x) / 3. * 2.),
            start.y + ((end.y - start.y) / 3. * 2.),
        )
    }

    fn compute_vertex(left: &PointF, right: &PointF) -> PointF {
        let length = right.x - left.x;
        Point::new(
            (left.x + right.x) / 2.,
            (length.powi(2) - (length / 2.).powi(2)).sqrt(),
        )
    }

    fn compute_inner(
        v: &mut Vec<PointF>,
        left: &PointF,
        right: &PointF,
        vertex: &PointF,
        depth: usize,
    ) {
        v.push(*left);
        if depth > 0 {
            let inner_left = compute_left(left, vertex);
            let inner_right = compute_right(left, vertex);
            let inner_vertex = compute_vertex(left, vertex);
            compute_inner(v, &inner_left, &inner_right, &inner_vertex, depth - 1);
        }
        v.push(*vertex);
        if depth > 0 {
            let inner_left = compute_left(right, vertex);
            let inner_right = compute_right(right, vertex);
            let inner_vertex = compute_vertex(right, vertex);
            compute_inner(v, &inner_left, &inner_right, &inner_vertex, depth - 1);
        }
        v.push(*right);

        // let inner_start = Point::new(
        //     left.x + ((right.x - left.x) / 3.),
        //     left.y + ((right.y - left.y) / 3.),
        // );
        // let inner_end = Point::new(
        //     left.x + ((right.x - left.x) / 3. * 2.),
        //     left.y + ((right.y - left.y) / 3. * 2.),
        // );

        // if depth > 0 {
        //     v.push(*left);
        //     compute_inner(v, &inner_start, &inner_end, depth - 1);
        //     v.push(*right);
        // } else {
        //     v.push(

        //     )
        //     let distance = inner_end.x - inner_start.x;
        // }
    }

    let mut v: Vec<PointF> = vec![];

    v.push(*start);

    let left = compute_left(start, end);

    let right = compute_right(start, end);

    let vertex = compute_vertex(&left, &right);

    compute_inner(&mut v, &left, &right, &vertex, depth - 1);
    v.push(*end);

    v
}

pub fn input_koch_curve(
    reader: &mut impl Read,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let n: usize = s.parse()?;

    let start = Point::new(0.00000, 0.00000);
    let end = Point::new(100.00000, 0.00000);

    let v = compute(&start, &end, n);
    for p in v {
        writeln!(writer, "{:.5} {:.5}", p.x, p.y)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let input = "1";
        let mut output: Vec<u8> = vec![];

        let result = input_koch_curve(&mut input.as_bytes(), &mut output);

        assert!(result.is_ok());
        assert_eq!(
            String::from_utf8(output).unwrap(),
            [
                "0.00000 0.00000",
                "33.33333 0.00000",
                "50.00000 28.86751",
                "66.66667 0.00000",
                "100.00000 0.00000",
                ""
            ]
            .join("\n")
        )
    }
}
