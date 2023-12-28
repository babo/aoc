use itertools::Itertools;
// use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
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

fn _intersect3((pa, va): &Stone, (pb, vb): &Stone) -> Option<(f64, f64, f64)> {
    assert_ne!(va.0, 0);
    assert_ne!(va.1, 0);
    assert_ne!(vb.0, 0);
    assert_ne!(vb.1, 0);

    let t2 =
        (va.0 * (pb.1 - pa.1) - va.1 * (pb.0 - pa.0)) as f64 / (va.1 * vb.0 - va.0 * vb.1) as f64;
    let t1 = ((pb.0 - pa.0) as f64 + vb.0 as f64 * t2) / va.0 as f64;

    if t1 == t2 {
        let q = (
            pa.0 as f64 + va.0 as f64 * t1,
            pa.1 as f64 + va.1 as f64 * t1,
            pa.2 as f64 + va.2 as f64 * t1,
        );
        Some(q)
    } else {
        None
    }
}

fn intersect3a((pa, va): &Stone, (pb, vb): &Stone, t0: i128, t1: i128) -> bool {
    if va.0 == vb.0 {
        return false;
    }
    let t = (pb.0 - pa.0 + (vb.0 - va.0) * t0) as f64 / (va.0 - vb.0) as f64;
    let t0 = t0 as f64;
    let t1 = t1 as f64;
    t > t0
        && t < t1
        && pb.1 as f64 + vb.1 as f64 * (t0 + t) == pa.1 as f64 + va.1 as f64 * (t0 + t)
        && pb.2 as f64 + vb.2 as f64 * (t0 + t) == pa.2 as f64 + va.2 as f64 * (t0 + t)
}

fn data_reader(input: &str) -> Vec<(Coord, Velocity)> {
    input
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
        .collect_vec()
}

fn solution_a(input: &str, area: Area) -> usize {
    let data = data_reader(input);

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

fn try_t(ai: usize, data: &[Stone], t_range: (i128, i128), step: i128) -> Option<i128> {
    let (pa, va) = data[ai];

    //println!("{:?} {:?}", pa, va);

    for t0 in t_range.0..t_range.1 {
        let qx = pa.0 + va.0 * t0 * step;
        let qy = pa.1 + va.1 * t0 * step;
        let qz = pa.2 + va.2 * t0 * step;

        let solution = data
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != ai)
            .map(|(i, (pb, vb))| {
                let mut m = 0;
                for t1 in (t0 + 1)..=t_range.1 {
                    let vx = vb.0 + (pb.0 - pa.0 + (vb.0 - va.0) * t0 * step) / t1 / step;
                    let vy = vb.1 + (pb.1 - pa.1 + (vb.1 - va.1) * t0 * step) / t1 / step;
                    let vz = vb.2 + (pb.2 - pa.2 + (vb.2 - va.2) * t0 * step) / t1 / step;
                    let pq = (qx - vx * t0 * step, qy - vy * t0 * step, qz - vz * t0 * step);
                    let vq = (vx, vy, vz);

                    let c = data
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != ai && *j != i)
                        .filter(|(_, b)| intersect3a(&(pq, vq), b, t0 * step, t1 * step))
                        .count();

                    if c > m {
                        m = c;
                        println!("{c} {}", data.len());
                        if c + 2 == data.len() {
                            let r = pq.0 + pq.1 + pq.2;
                            println!("{r}");
                            return Some(r);
                        }
                    }
                }
                None
            })
            .find(|x| x.is_some());

        if let Some(raw) = solution {
            return raw;
        }
    }

    None
}

fn _sol_1(data: &[Stone]) -> Option<i128> {
    let base = 100000000;
    //let base = 80000000000;
    println!("{}", base);
    println!("80000000000");
    println!("{}", 10000 * base);
    for st in 1000..=10000 {
        if let Some(res) = data
            .iter()
            .enumerate()
            .map(|(i, _)| try_t(i, &data, (1, 200), st * base))
            .find(|x| x.is_some())
            .flatten()
        {
            return Some(res);
        }
    }

    None
}

fn sol_2(data: &[Stone]) -> Option<i128> {
    let base = 1;
    for step in 1..=40 {
        if let Some(res) = data
            .iter()
            .enumerate()
            .map(|(i, _)| try_t(i, &data, (0, 200), step * base))
            .find(|x| x.is_some())
            .flatten()
        {
            return Some(res);
        }
    }

    None
}

fn _magnitude(a: &Coord, b: &Coord) -> i128 {
    let x = b.0 - a.0;
    let y = b.1 - a.1;
    let z = b.2 - a.2;
    ((x * x + y * y + z * z) as f64).sqrt().ceil() as i128
}

fn solution_b(input: &str) -> Option<i128> {
    let data = data_reader(input);

    sol_2(&data)
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
        assert_eq!(solution_b(&data), Some(47));
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
        let r = solution_b(&c);
        println!("{:?}", r);
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r < 752666693060129);
        assert!(r > 503650032999997);
        assert_eq!(r, 42);
    }
}
