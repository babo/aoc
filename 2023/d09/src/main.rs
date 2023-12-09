use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn diff_first(row: &Vec<i64>) -> i64 {
    if row.iter().all(|x| *x == 0) {
        return 0;
    }
    let d: Vec<i64> = row
        .iter()
        .skip(1)
        .fold((Vec::new(), 0), |(mut v, y), x| {
            v.push(x - y);
            (v, *x)
        })
        .0;

    row.iter().last().unwrap() + diff_first(&d)
}

fn diff_last(row: &Vec<i64>) -> i64 {
    if row.iter().all(|x| *x == 0) {
        return 0;
    }
    let a = *row.iter().next().unwrap();
    let d: Vec<i64> = row
        .iter()
        .skip(1)
        .fold((Vec::new(), a), |(mut v, y), x| {
            v.push(x - y);
            (v, *x)
        })
        .0;

    a - diff_last(&d)
}

fn solve_a(line: &str) -> i64 {
    let row = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    diff_first(&row)
}

fn solve_b(line: &str) -> i64 {
    let row = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    diff_last(&row)
}

fn solution_a(input: &str) -> Option<i64> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<i64>())
}

fn solution_b(input: &str) -> Option<i64> {
    let v = input.lines().map(|x| solve_b(x.trim())).collect_vec();
    println!("{:?}", v);
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<i64>())
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
        assert_eq!(solution_a(&data), Some(114));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(2));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1806615041));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1211));
    }
}
