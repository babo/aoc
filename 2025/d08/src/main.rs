use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

type Coord = (u64, u64, u64);

fn sq(a: u64, b: u64) -> usize {
    let a = a as usize;
    let b = b as usize;
    if a >= b {
        (a - b) * (a - b)
    } else {
        (b - a) * (b - a)
    }
}
fn distance(a: &Coord, b: &Coord) -> usize {
    sq(a.0, b.0) + sq(a.1, b.1) + sq(a.2, b.2)
}

fn solution_a(input: &str, limit: usize) -> Option<usize> {
    let points = input
        .lines()
        .take(1000)
        .map(|line| {
            line.trim()
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple::<Coord>()
                .unwrap()
        })
        .collect::<Vec<Coord>>();
    let mut dist = Vec::<((usize, usize), usize)>::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            dist.push((
                (i, j),
                distance(&points.get(i).unwrap(), &points.get(j).unwrap()),
            ));
        }
    }
    dist.sort_by_key(|p| p.1);

    let mut belongs = HashMap::<usize, usize>::new();
    let mut count = Vec::new();
    dist.iter().take(limit).for_each(|((a, b), _)| {
        if !belongs.contains_key(a) && !belongs.contains_key(b) {
            let cid = count.len();
            let r1 = belongs.insert(*a, cid);
            let r2 = belongs.insert(*b, cid);
            assert_eq!(r1, None);
            assert_eq!(r2, None);
            count.push(2);
        } else if belongs.contains_key(a) && !belongs.contains_key(b) {
            let circuit = *belongs.get(a).unwrap();
            let r1 = belongs.insert(*b, circuit);
            assert_eq!(r1, None);

            count.get_mut(circuit).map(|c| *c += 1);
        } else if !belongs.contains_key(a) && belongs.contains_key(b) {
            let circuit = *belongs.get(b).unwrap();
            let r1 = belongs.insert(*a, circuit);
            assert_eq!(r1, None);

            count.get_mut(circuit).map(|c| *c += 1);
        } else {
            let circuit = *belongs.get(a).unwrap();
            let mrg = *belongs.get(b).unwrap();
            if circuit != mrg {
                belongs.iter_mut().filter(|x| *x.1 == mrg).for_each(|p| {
                    *p.1 = circuit;
                    count.get_mut(circuit).map(|c| *c += 1);
                    count.get_mut(mrg).map(|c| *c -= 1);
                });
            }
        }
    });
    count.sort();
    Some(count.iter().rev().take(3).fold(1, |acc, x| acc * x))
}

fn solution_b(input: &str) -> Option<usize> {
    let points = input
        .lines()
        .take(1000)
        .map(|line| {
            line.trim()
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple::<Coord>()
                .unwrap()
        })
        .collect::<Vec<Coord>>();
    let np = points.len();
    let mut dist = Vec::<((usize, usize), usize)>::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            dist.push((
                (i, j),
                distance(&points.get(i).unwrap(), &points.get(j).unwrap()),
            ));
        }
    }
    dist.sort_by_key(|p| p.1);

    let mut belongs = HashMap::<usize, usize>::new();
    let mut count = Vec::new();
    dist.iter().for_each(|((a, b), _)| {
        if !belongs.contains_key(a) && !belongs.contains_key(b) {
            let cid = count.len();
            let r1 = belongs.insert(*a, cid);
            let r2 = belongs.insert(*b, cid);
            assert_eq!(r1, None);
            assert_eq!(r2, None);
            count.push(2);
        } else if belongs.contains_key(a) && !belongs.contains_key(b) {
            let circuit = *belongs.get(a).unwrap();
            let r1 = belongs.insert(*b, circuit);
            assert_eq!(r1, None);

            count.get_mut(circuit).map(|c| *c += 1);
        } else if !belongs.contains_key(a) && belongs.contains_key(b) {
            let circuit = *belongs.get(b).unwrap();
            let r1 = belongs.insert(*a, circuit);
            assert_eq!(r1, None);

            count.get_mut(circuit).map(|c| *c += 1);
        } else {
            let circuit = *belongs.get(a).unwrap();
            let mrg = *belongs.get(b).unwrap();
            if circuit != mrg {
                belongs.iter_mut().filter(|x| *x.1 == mrg).for_each(|p| {
                    *p.1 = circuit;
                    count.get_mut(circuit).map(|c| *c += 1);
                    count.get_mut(mrg).map(|c| *c -= 1);
                });
            }
        }

        if count.iter().max() >= Some(&np) {
            let rtv = points.get(*a).unwrap().0 as usize * points.get(*b).unwrap().0 as usize;
            println!("{rtv}");
        }
    });
    None
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c, 1000);
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
        assert_eq!(solution_a(&data, 10), Some(40));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(25272));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c, 1000), Some(66912));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(724454082));
    }
}
