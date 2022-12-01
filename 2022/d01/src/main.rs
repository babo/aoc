use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn collect_calories(input: &str) -> Vec<usize> {
    let (mut calories, last) = input
        .lines()
        .fold(
            (Vec::new(), 0usize),
            |(mut acc, prev), line| match usize::from_str_radix(line.trim(), 10)
                .ok()
                .map_or((Some(prev), 0), |c| (None, prev + c))
            {
                (Some(val), p) if val != 0 => {
                    acc.push(val);
                    (acc, p)
                }
                (_, p) => (acc, p),
            },
        );
    if last != 0 {
        calories.push(last);
    }
    calories
}

fn solution_a(input: &str) -> Option<usize> {
    collect_calories(input).into_iter().sorted().last()
}

fn solution_b(input: &str) -> Option<usize> {
    Some(
        collect_calories(input)
            .into_iter()
            .sorted()
            .rev()
            .take(3)
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
        assert_eq!(solution_a(&data), Some(24000));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(45000));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(66186));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(196804));
    }
}
