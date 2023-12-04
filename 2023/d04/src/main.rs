use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn nmatch(line: &str) -> u32 {
    let numbers: Vec<HashSet<u32>> = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .collect_vec()
        .iter()
        .map(|x| {
            let n = x
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();

            let h: HashSet<u32> = HashSet::from_iter(n.iter().map(|x| *x));
            h
        })
        .collect_vec();
    let mut it = numbers.iter();
    it.next()
        .map(|a| it.next().map(|b| a.intersection(b).count()).unwrap() as u32)
        .unwrap()
}

fn winning_number(m: u32) -> usize {
    if m == 0 {
        0
    } else {
        (1..m).fold(1, |accu, _| accu * 2)
    }
}

fn solve_a(line: &str) -> usize {
    winning_number(nmatch(line))
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    let tc = input.lines().filter(|x| !x.trim().is_empty()).count();

    Some(
        input
            .lines()
            .fold(
                (0, &mut iter::repeat(1).take(tc).collect_vec()),
                |(ln, v), line| {
                    let copies = *v.get(ln).unwrap();
                    let n = nmatch(line);
                    for i in 0..n as usize {
                        let index = ln + 1 + i;
                        if let Some(x) = v.get_mut(index) {
                            *x += copies;
                        }
                    }
                    println!("{ln} {copies} {n} {} {:?}", winning_number(n), v);
                    (ln + 1, v)
                },
            )
            .1
            .iter()
            .sum(),
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
        assert_eq!(solution_a(&data), Some(13));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(30));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(24848));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(7258152));
    }
}
