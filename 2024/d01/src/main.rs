use itertools::Itertools;
use std::collections::HashMap;
use std::{fs::read_to_string, iter::zip};

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| {
            let t: (u64, u64) = x
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            t
        })
        .collect_vec();
    Some(
        zip(
            input.iter().map(|x| x.0).sorted(),
            input.iter().map(|x| x.1).sorted(),
        )
        .map(|(a, b)| {
            if a > b {
                (a - b) as usize
            } else {
                (b - a) as usize
            }
        })
        .sum::<usize>(),
    )
}

fn solution_b(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| {
            let t: (u64, u64) = x
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            t
        })
        .collect_vec();
    let count = input
        .iter()
        .map(|x| x.1)
        .fold(HashMap::new(), |mut acc, x| {
            if let Some(v) = acc.get_mut(&x) {
                *v += 1;
            } else {
                acc.insert(x, 1usize);
            };
            acc
        });
    Some(
        input
            .iter()
            .map(|x| x.0 as usize * count.get(&x.0).unwrap_or(&0))
            .sum::<usize>(),
    )
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
        assert_eq!(solution_a(&data), Some(11));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(31));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(765748));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(27732508));
    }
}
