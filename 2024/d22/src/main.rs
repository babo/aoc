use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn calc(n: usize) -> usize {
    let mut n = n;
    n ^= n << 6;
    n &= 0xFFFFFF;
    n ^= n >> 5;
    n &= 0xFFFFFF;
    n ^= n << 11;
    n &= 0xFFFFFF;
    n
}

fn solve_a(line: &str) -> usize {
    let mut n = line.parse::<usize>().unwrap();

    for _ in 0..2000 {
        n = calc(n);
    }
    n
}

fn solution_a(input: &str) -> usize {
    input.lines().map(|x| solve_a(x.trim())).sum::<usize>()
}

fn solution_b(input: &str) -> usize {
    let mut all = HashMap::new();
    let mut maxi = 0;

    for line in input.trim().lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut n = line.trim().parse::<usize>().unwrap();
        println!("Secret: {}", n);

        let mut p: [i8; 4] = [(n % 10) as i8, 0, 0, 0];
        let mut seen = HashSet::new();
        for i in 0..2000 {
            n = calc(n);
            let digi = (n % 10) as i8;
            let (p4, _) = (1..5).rev().fold((0u32, digi), |(acc, prev), j| {
                let curr = p[(i + j) % 4];
                let a = (9 + prev - curr) as u32;
                // println!("->{}: {} {}", (i+j) % 4, prev-curr, a);
                ((acc << 5) | a, curr)
            });
            if i > 2 {
                if seen.insert(p4) {
                    let c = if let Some(v) = all.get_mut(&p4) {
                        *v += digi as usize;
                        *v
                    } else {
                        all.insert(p4, digi as usize);
                        digi as usize
                    };
                    if c > maxi {
                        maxi = c;
                    }
                }
            }
            //println!("{}: {} ({}) {} {} {} {}", i, n % 10, (n % 10) as i8 - p[i % 4], (p4 & 0x1f) as i16 - 9, ((p4 >> 5) & 0x1f) as i16 - 9, ((p4 >> 10) & 0x1f) as i16 - 9, ((p4 >> 15) & 0x1f) as i16 - 9);
            //println!("  {} {} {} {}", (p4 & 0x1f) as i16 - 9, ((p4 >> 3) & 0x1f) as i16 - 9, ((p4 >> 6) & 0x1f) as i16 - 9, ((p4 >> 9) & 0x1f) as i16 - 9);
            p[(i + 1) % 4] = digi;
        }
    }
    all.iter().for_each(|(p4, v)| {
        if v == &maxi {
            println!(
                "0x{:04X}: {} {} {} {}",
                p4,
                (p4 & 0x1f) as i16 - 9,
                ((p4 >> 5) & 0x1f) as i16 - 9,
                ((p4 >> 10) & 0x1f) as i16 - 9,
                ((p4 >> 15) & 0x1f) as i16 - 9
            );
        }
    });
    maxi
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
    fn test_mini() {
        assert_eq!(solution_a("123"), 43);
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), 37327623);
    }

    #[test]
    fn test_mini_b() {
        let data = "123";
        assert_eq!(solution_b(data), 23);
    }

    #[test]
    fn test_simple_b() {
        let data = "1
                    2
                    3
                    2024";
        assert_eq!(solution_b(data), 23);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 13429191512);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 1582);
    }
}
