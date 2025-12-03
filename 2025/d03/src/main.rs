use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solve_a(line: &str) -> usize {
    let nums: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let d1 = nums
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .max_by_key(|x| x.1)
        .unwrap();
    let d2 = nums.iter().skip(d1.0 + 1).max().unwrap();
    println!("{} {} {}", d1.0, d1.1, d2);
    (d1.1 * 10 + d2) as usize
}

fn solve_b(line: &str) -> usize {
    let nums: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    (0..12)
        .rev()
        .fold((0usize, 0usize), |(acc, lo), hi| {
            let (p, n) = nums
                .iter()
                .enumerate()
                .skip(lo)
                .rev()
                .skip(hi)
                .max_by_key(|x| x.1)
                .unwrap();
            (acc * 10 + *n as usize, p + 1)
        })
        .0
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
        assert_eq!(solution_a(&data), Some(357));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(3121910778619));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(17694));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(175659236361660));
    }
}
