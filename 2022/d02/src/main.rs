use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn score_a(pattern: &str) -> usize {
    let mut iter = pattern.chars();
    let other = iter.nth(0);
    match iter.nth(1) {
        Some('X') => 1 + match other {
            Some('A') => 3,
            Some('B') => 0,
            Some('C') => 6,
            _ => unreachable!("Never ever")
        },
        Some('Y') => 2 + match other {
            Some('A') => 6,
            Some('B') => 3,
            Some('C') => 0,
            _ => unreachable!("Never ever")
        },
        Some('Z') => 3 + match other {
            Some('A') => 0,
            Some('B') => 6,
            Some('C') => 3,
            _ => unreachable!("Never ever")
        },
        _ => unreachable!("Never ever")
    }
}

fn score_b(pattern: &str) -> usize {
    let mut iter = pattern.chars();
    let other = iter.nth(0);
    match iter.nth(1) {
        Some('X') => 0 + match other {
            Some('A') => 3,
            Some('B') => 1,
            Some('C') => 2,
            _ => unreachable!("Never ever")
        },
        Some('Y') => 3 + match other {
            Some('A') => 1,
            Some('B') => 2,
            Some('C') => 3,
            _ => unreachable!("Never ever")
        },
        Some('Z') => 6 + match other {
            Some('A') => 2,
            Some('B') => 3,
            Some('C') => 1,
            _ => unreachable!("Never ever")
        },
        _ => unreachable!("Never ever")
    }
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().fold(0usize, |acc, line| score_a(line) + acc))
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().fold(0usize, |acc, line| score_b(line) + acc))
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
        assert_eq!(solution_a(&data), Some(15));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(12));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(14297));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(10498));
    }
}
