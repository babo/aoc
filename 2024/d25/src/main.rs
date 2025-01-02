use itertools::Itertools;
use std::{fs::read_to_string, vec};

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solve_b(_line: &str) -> usize {
    0
}

type pinlock = (u8, u8, u8, u8, u8);

fn solution_a(input: &str) -> Option<usize> {
    let mut locks =Vec::<pinlock>::new();
    let mut keys = Vec::<pinlock>::new();

    let mut add_line = |pattern: &String| {
        let is_key = pattern.starts_with(".....");
        let mut pins = vec![];
        for col in 0..5 {
            let start = if is_key { 0 } else { 1 };
            pins.push((start..6).fold(0, |acc, row| if pattern.chars().nth(col + row * 5).unwrap() == '#' { acc + 1 } else { acc }));
        }
        if is_key {
            keys.push((pins[0], pins[1], pins[2], pins[3], pins[4]));
        } else {
            locks.push((pins[0], pins[1], pins[2], pins[3], pins[4]));
        }
    };

    let last = input.trim().lines().fold(String::new(), |acc, line| {
        if line.is_empty() {
            add_line(&acc);
            String::new()
        } else {
            acc + line.trim()
        }
    });
    add_line(&last);

    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if lock.0 + key.0 <= 5 && lock.1 + key.1 <= 5 && lock.2 + key.2 <= 5 && lock.3 + key.3 <= 5 && lock.4 + key.4 <= 5 {
                println!("Fit: {:?} {:?}", lock, key);
                count += 1;
            } else {
                println!("Unfit: {:?} {:?}", lock, key);
            }
        }
    }


    Some(count)
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<usize>())
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
        assert_eq!(solution_a(&data), Some(3));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(3317));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
