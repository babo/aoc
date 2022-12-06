use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_a(input: &str, n: usize) -> Option<usize> {
    (0usize..(input.len() - n))
        .position(|i| {
            input
                .get(i..(i + n))
                .map_or(false, |x| HashSet::<char>::from_iter(x.chars()).len() == n)
        })
        .map(|x| x + n)
}

fn solution_a(input: &str) -> Option<usize> {
    solve_a(input, 4)
}

fn solution_b(input: &str) -> Option<usize> {
    solve_a(input, 14)
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
        let res = data
            .lines()
            .map(|x| solution_a(x.trim()).map_or(0, |x| x))
            .sum::<usize>();

        assert_eq!(res, 7 + 5 + 6 + 10 + 11);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        let res = data
            .lines()
            .map(|x| solution_b(x.trim()).map_or(0, |x| x))
            .sum::<usize>();

        assert_eq!(res, 19 + 23 + 23 + 29 + 26);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1134));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(2263));
    }
}
