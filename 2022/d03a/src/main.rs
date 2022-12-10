//#![feature(iter_array_chunks)]

use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}


fn main() {
    let c = content().unwrap();

    for func in [three::one, three::two] {
        let start = std::time::Instant::now();
        let res = func(c.as_str());
        let dur = start.elapsed().as_nanos();

        println!("{res} ({dur} ns)");
    }
}

mod three {
    fn unique_items(s: &str) -> u64 {
        s.bytes()
            .map(|b| match b {
                b'a'..=b'z' => 1 + b - b'a',
                b'A'..=b'Z' => 27 + b - b'A',
                _ => unreachable!(),
            })
            .fold(0, |acc, b| acc | (1u64 << b))
    }

    pub fn one(input: &str) -> u32 {
        input
            .lines()
            .map(|bag| bag.split_at(bag.len() / 2))
            .map(|(l, r)| [l, r].map(unique_items))
            .map(|[l, r]| u64::trailing_zeros(l & r))
            .sum()
    }

    pub fn two(input: &str) -> u32 {
        input
            .lines()
            .array_chunks::<3>()
            .map(|bags| bags.map(unique_items))
            .map(|[a, b, c]| a & b & c)
            .map(u64::trailing_zeros)
            .sum()
    }
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
        assert_eq!(solution_a(&data), Some(0));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(0));
    }

    #[test]
     fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
