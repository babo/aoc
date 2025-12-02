use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solve_a(line: &str) -> usize {
    let ab: (usize, usize) = line
        .split('-')
        .map(|p| p.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut rtv = 0;
    for i in ab.0..=ab.1 {
        let x = i.to_string();
        if x.len() % 2 == 0 {
            let n = x.len() / 2;
            if x[0..n] == x[n..] {
                rtv += i;
            }
        }
    }
    rtv
}

fn silly(id: usize) -> bool {
    let s = id.to_string();
    let n = s.len();
    for i in 1..=n / 2 {
        if n % i == 0 {
            let mut valid = true;
            let y = n / i;
            for j in 1..y {
                if s[0..i] != s[j * i..(j + 1) * i] {
                    valid = false;
                    break;
                }
            }
            if valid {
                return true;
            }
        }
    }

    false
}

fn solve_b(line: &str) -> usize {
    let ab: (usize, usize) = line
        .split('-')
        .map(|p| p.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut rtv = 0;
    for i in ab.0..=ab.1 {
        if silly(i) {
            rtv += i;
        }
    }
    rtv
}

fn solution_a(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .replace(",", " ")
            .split_ascii_whitespace()
            .map(|x| solve_a(x.trim()))
            .sum::<usize>(),
    )
}

fn solution_b(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .replace(",", " ")
            .split_ascii_whitespace()
            .map(|x| solve_b(x.trim()))
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
        assert_eq!(solution_a(&data), Some(1227775554));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(4174379265));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(26255179562));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(31680313976));
    }
}
