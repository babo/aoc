use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_b(_line: &str) -> usize {
    0
}

type Coord = (i128, i128, i128);
type Velocity = (i128, i128, i128);
type Stone = (Coord, Velocity);
type Area = (i128, i128);

fn intersect2((pa, va): &Stone, (pb, vb): &Stone, area: &Area) -> bool {
    assert_ne!(va.0, 0);
    assert_ne!(va.1, 0);
    assert_ne!(vb.0, 0);
    assert_ne!(vb.1, 0);

    if va.0 * vb.1 == va.1 * vb.0 {
        return false;
    }
    let t2 =
        (va.0 * (pb.1 - pa.1) - va.1 * (pb.0 - pa.0)) as f64 / (va.1 * vb.0 - va.0 * vb.1) as f64;
    if t2 < 0. {
        return false;
    }
    let t1 = ((pb.0 - pa.0) as f64 + vb.0 as f64 * t2) / va.0 as f64;
    if t1 < 0. {
        return false;
    }
    let q = (
        pa.0 as f64 + va.0 as f64 * t1,
        pa.1 as f64 + va.1 as f64 * t1,
    );
    q.0 >= area.0 as f64 && q.0 <= area.1 as f64 && q.1 >= area.0 as f64 && q.1 <= area.1 as f64
}

fn solution_a(input: &str, area: Area) -> usize {
    let data = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|line| {
            let c: Coord = line
                .split(" @ ")
                .next()
                .unwrap()
                .split(", ")
                .map(|x| x.trim().parse::<i128>().unwrap())
                .collect_tuple()
                .unwrap();
            let v: Velocity = line
                .split(" @ ")
                .last()
                .unwrap()
                .split(", ")
                .map(|x| x.trim().parse::<i128>().unwrap())
                .collect_tuple()
                .unwrap();
            (c, v)
        })
        .collect_vec();

    data.iter()
        .enumerate()
        .skip(1)
        .map(|(i, a)| {
            data.iter()
                .take(i)
                .map(|b| intersect2(a, b, &area))
                .filter(|x| *x)
                .count()
        })
        .sum::<usize>()
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<usize>())
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c, (200000000000000, 400000000000000));
    let b = solution_b(&c);

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple() -> Option<String> {
        read_to_string("./simple.txt").ok()
    }

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data, (7, 27)), 2);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let r = solution_a(&c, (200000000000000, 400000000000000));
        assert_eq!(r, 12740);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
