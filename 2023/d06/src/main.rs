use itertools::Itertools;
use std::fs::read_to_string;
use std::iter;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn do_race(lasts: usize, record: usize) -> usize {
    (1..lasts)
        .map(|t| (lasts - t) * t > record)
        .filter(|x| *x)
        .count()
}

fn solve_a(line: &str) -> usize {
    let races = line
        .lines()
        .take(2)
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .skip(1)
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_tuple::<(Vec<_>, Vec<_>)>()
        .unwrap();
    iter::zip(races.0, races.1)
        .map(|x| do_race(x.0 as usize, x.1 as usize))
        .product::<usize>()
}

fn solution_a(input: &str) -> Option<usize> {
    Some(solve_a(input))
}

fn solution_b(input: &str) -> Option<usize> {
    let nums = input
        .lines()
        .take(2)
        .map(|line| {
            line.chars()
                .filter(|x| x.is_numeric())
                .fold(0usize, |accu, x| {
                    accu * 10 + x.to_digit(10).unwrap() as usize
                })
        })
        .collect_tuple::<(usize, usize)>()
        .unwrap();

    Some(do_race(nums.0, nums.1))
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
        assert_eq!(solution_a(&data), Some(288));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(71503));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(840336));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(41382569));
    }
}
