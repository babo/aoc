use std::fs::read_to_string;
use std::collections::HashSet;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_a(line: &str) -> usize {
    0
}

fn solve_b(line: &str) -> usize {
    0
}

fn solution_a(input: &str) -> Option<usize> {
    if input.len() < 4 {
        None
    } else {
        let a = (0usize..input.len()).find_position(|i| {
            input.get(*i..(*i+4)).map_or(false, |x| {
                let s: HashSet<char> = HashSet::from_iter(x.chars());
                s.len() == 4
            })
        });
        println!("{:?}", a);
        a.map(|x| x.0+4)
    }
}

fn solution_b(input: &str) -> Option<usize> {
    if input.len() < 14 {
        None
    } else {
        let a = (0usize..input.len()).find_position(|i| {
            input.get(*i..(*i+14)).map_or(false, |x| {
                let s: HashSet<char> = HashSet::from_iter(x.chars());
                s.len() == 14
            })
        });
        println!("{:?}", a);
        a.map(|x| x.0+14)
    }
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
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
        let res = data.lines().map(|x| solution_a(x.trim()).map_or(0, |x| x)).sum::<usize>();

        assert_eq!(res, 7+5+6+10+11);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        let data = simple().unwrap();
        let res = data.lines().map(|x| solution_a(x.trim()).map_or(0, |x| x)).sum::<usize>();

        assert_eq!(res, 7+5+6+10+11);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(0));
    }

    #[test]
     fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
