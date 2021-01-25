use std::collections::HashSet;
use std::fs::read_to_string;

type Dim3 = (i16, i16, i16);
type Dim4 = (i16, i16, i16, i16);

trait Neighbours<T> {
    fn neighbours(c: &T) -> Vec<T>;
}

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn to_hash<T: Eq + std::hash::Hash>(input: &str, conv: fn(i16, i16) -> T) -> HashSet<T> {
    let mut hs = HashSet::new();

    let mut x: i16 = 0;

    for line in input.lines() {
        line.trim()
            .chars()
            .enumerate()
            .filter(|x| x.1 == '#')
            .for_each(|y| {
                let t = conv(x, y.0 as i16);
                hs.insert(t);
            });
        x += 1;
    }
    hs
}

impl Neighbours<Dim3> for Dim3 {
    fn neighbours(c: &Dim3) -> Vec<Dim3> {
        let mut rtv = Vec::new();

        for x in c.0 - 1..=c.0 + 1 {
            for y in c.1 - 1..=c.1 + 1 {
                for z in c.2 - 1..=c.2 + 1 {
                    let n = (x, y, z);
                    if n != *c {
                        rtv.push(n);
                    }
                }
            }
        }
        rtv
    }
}

impl Neighbours<Dim4> for Dim4 {
    fn neighbours(c: &Dim4) -> Vec<Dim4> {
        let mut rtv = Vec::new();

        for x in c.0 - 1..=c.0 + 1 {
            for y in c.1 - 1..=c.1 + 1 {
                for z in c.2 - 1..=c.2 + 1 {
                    for w in c.3 - 1..=c.3 + 1 {
                        let n = (x, y, z, w);
                        if n != *c {
                            rtv.push(n);
                        }
                    }
                }
            }
        }
        rtv
    }
}

fn cycle<T: Neighbours<T> + Copy + Eq + std::hash::Hash>(input: &HashSet<T>) -> HashSet<T> {
    let mut hs = HashSet::<T>::new();

    for cube in input {
        let nb = T::neighbours(cube);
        let count = nb.iter().filter(|x| input.contains(*x)).count();
        match count {
            2 => {
                hs.insert(*cube);
            }
            3 => {
                hs.insert(*cube);
            }
            _ => (),
        }
        nb.iter().filter(|x| !input.contains(*x)).for_each(|x| {
            if !hs.contains(x)
                && T::neighbours(x)
                    .iter()
                    .filter(|x| input.contains(*x))
                    .count()
                    == 3
            {
                hs.insert(*x);
            }
        });
    }

    hs
}

fn to_dim3(x: i16, y: i16) -> Dim3 {
    (x, y, 0i16)
}

fn to_dim4(x: i16, y: i16) -> Dim4 {
    (x, y, 0i16, 0i16)
}

fn solution_a(input: &str) -> usize {
    let mut hs = to_hash::<Dim3>(input, to_dim3);
    for _ in 0..6 {
        hs = cycle(&hs);
    }
    hs.len()
}

fn solution_b(input: &str) -> usize {
    let mut hs = to_hash::<Dim4>(input, to_dim4);
    for _ in 0..6 {
        hs = cycle(&hs);
    }
    hs.len()
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {}", a);
    println!("Step B: {}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_cycle_1() {
        let input = to_hash(
            ".#.
        ..#
        ###",
            to_dim3,
        );

        assert_eq!(cycle(&input).len(), 11);
    }

    #[test]
    fn test_cycle_2() {
        let mut input = to_hash(
            ".#.
        ..#
        ###",
            to_dim3,
        );

        for _ in 0..2 {
            input = cycle(&input);
        }
        assert_eq!(input.len(), 21);
    }

    #[test]
    fn test_cycle_3() {
        let mut input = to_hash(
            ".#.
        ..#
        ###",
            to_dim3,
        );

        for _ in 0..3 {
            input = cycle(&input);
        }
        assert_eq!(input.len(), 38);
    }

    #[test]
    fn test_cycle_6() {
        let mut input = to_hash(
            ".#.
        ..#
        ###",
            to_dim3,
        );

        for _ in 0..6 {
            input = cycle(&input);
        }
        assert_eq!(input.len(), 112);
    }

    #[test]
    fn test_dim4() {
        let mut input = to_hash(
            ".#.
        ..#
        ###",
            to_dim4,
        );

        for _ in 0..6 {
            input = cycle(&input);
        }
        assert_eq!(input.len(), 848);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();

        assert_eq!(solution_a(&c), 280);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();

        assert_eq!(solution_b(&c), 1696);
    }
}
