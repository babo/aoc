use std::collections::HashSet;
use std::fs::read_to_string;

type Coord = i16;
type Coords = (Coord, Coord, Coord);

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn to_hash(input: &str) -> HashSet<Coords> {
    let mut hs = HashSet::new();

    let mut x: Coord = 0;

    for line in input.lines() {
        line.trim()
            .chars()
            .enumerate()
            .filter(|x| x.1 == '#')
            .for_each(|y| {
                hs.insert((x, y.0 as Coord, 0 as Coord));
            });
        x += 1;
    }
    hs
}

fn neighbours(c: &Coords) -> [Coords; 26] {
    let mut rtv: [Coords; 26] = [*c; 26];
    let mut i = 0usize;

    for x in c.0 - 1..=c.0 + 1 {
        for y in c.1 - 1..=c.1 + 1 {
            for z in c.2 - 1..=c.2 + 1 {
                let n = (x, y, z);
                if n != *c {
                    rtv[i] = n;
                    i += 1;
                }
            }
        }
    }
    rtv
}

fn cycle(input: &HashSet<Coords>) -> HashSet<Coords> {
    let mut hs = HashSet::new();

    for cube in input {
        let nb = neighbours(cube);
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
            if !hs.contains(x) && neighbours(x).iter().filter(|x| input.contains(*x)).count() == 3 {
                hs.insert(*x);
            }
        });
    }

    hs
}

fn solution_a(input: &str) -> usize {
    let mut hs = to_hash(input);
    for _ in 0..6 {
        hs = cycle(&hs);
    }
    hs.len()
}

fn solution_b(_input: &str) -> i32 {
    0
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
        );

        assert_eq!(cycle(&input).len(), 11);
    }

    #[test]
    fn test_cycle_2() {
        let mut input = to_hash(
            ".#.
        ..#
        ###",
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
        );

        for _ in 0..6 {
            input = cycle(&input);
        }
        assert_eq!(input.len(), 112);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();

        assert_eq!(solution_a(&c), 280);
    }
}
