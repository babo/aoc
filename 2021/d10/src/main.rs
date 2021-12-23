use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let score = input
        .lines()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            line.chars().find_map(|x| {
                match x {
                    '<' => stack.push(x),
                    '[' => stack.push(x),
                    '{' => stack.push(x),
                    '(' => stack.push(x),
                    ')' => match stack.pop() {
                        Some('(') => (),
                        _ => return Some(3),
                    },
                    ']' => match stack.pop() {
                        Some('[') => (),
                        _ => return Some(57),
                    },
                    '}' => match stack.pop() {
                        Some('{') => (),
                        _ => return Some(1197),
                    },
                    '>' => match stack.pop() {
                        Some('<') => (),
                        _ => return Some(25137),
                    },
                    _ => (),
                };
                None
            })
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sum();
    Some(score)
}

fn solution_b(input: &str) -> Option<usize> {
    let scores: Vec<usize> = input
        .lines()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            let corrupted = line.chars().find(|x| {
                match x {
                    '<' => stack.push(*x),
                    '[' => stack.push(*x),
                    '{' => stack.push(*x),
                    '(' => stack.push(*x),
                    ')' => match stack.pop() {
                        Some('(') => (),
                        _ => return true,
                    },
                    ']' => match stack.pop() {
                        Some('[') => (),
                        _ => return true,
                    },
                    '}' => match stack.pop() {
                        Some('{') => (),
                        _ => return true,
                    },
                    '>' => match stack.pop() {
                        Some('<') => (),
                        _ => return true,
                    },
                    _ => return true,
                };
                false
            });
            if corrupted.is_some() {
                None
            } else {
                let score = stack.iter().rev().fold(0, |acc, x| {
                    acc * 5
                        + match *x {
                            '<' => 4,
                            '[' => 2,
                            '{' => 3,
                            '(' => 1,
                            _ => 0,
                        }
                });
                Some(score)
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .sorted()
        .collect();
    scores.iter().nth(scores.len() / 2).map(|x| *x)
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
        assert_eq!(solution_a(&data), Some(26397));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(288957));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(374061));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1600104));
    }
}
