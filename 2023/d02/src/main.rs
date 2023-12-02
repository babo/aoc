use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn procline(line: &str) -> (usize, Vec<(usize, usize, usize)>) {
    let mut s = line.split_at(5).1.split(": ");
    let n = s.next().unwrap().parse::<usize>().unwrap();
    let v = s
        .next()
        .unwrap()
        .split("; ")
        .map(|x| {
            x.split(", ").fold((0, 0, 0), |rgb, x| {
                let mut it = x.split_ascii_whitespace();
                let n = it.next().unwrap().parse::<usize>().unwrap();
                match it.next().unwrap().chars().next() {
                    Some('r') => (rgb.0 + n, rgb.1, rgb.2),
                    Some('g') => (rgb.0, rgb.1 + n, rgb.2),
                    Some('b') => (rgb.0, rgb.1, rgb.2 + n),
                    _ => unimplemented!("What?"),
                }
            })
        })
        .collect_vec();
    (n, v)
}

fn solve_a(line: &str) -> usize {
    let rgb = (12, 13, 14);
    let nv = procline(line);
    if nv
        .1
        .iter()
        .any(|x| x.0 > rgb.0 || x.1 > rgb.1 || x.2 > rgb.2)
    {
        0
    } else {
        nv.0
    }
}

fn solve_b(line: &str) -> usize {
    let nv = procline(line);
    let rgb = nv.1.iter().fold((0, 0, 0), |accu, x| {
        (accu.0.max(x.0), accu.1.max(x.1), accu.2.max(x.2))
    });
    rgb.0 * rgb.1 * rgb.2
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<usize>())
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
        assert_eq!(solution_a(&data), Some(8));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(2286));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(2256));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(74229));
    }
}
