use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn priority(c: &char) -> u32 {
    if c.is_ascii_uppercase() {
        return 27u32 + *c as u32 - 'A' as u32
    }
    if c.is_ascii_lowercase() {
        return 1u32 + *c as u32 - 'a' as u32
    }
    unreachable!("It must be A-Za-z")
}

fn unique(line: &str) -> usize {
    let n = line.len() / 2;
    let a: HashSet<char> = HashSet::from_iter(line[0..n].chars());
    let b: HashSet<char> = HashSet::from_iter(line[n..].chars());
    let d = a.intersection(&b);
    priority(d.last().unwrap()) as usize
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| unique(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    let a = input.lines().fold((HashSet::new(), 0usize, 0usize), |accu: (HashSet<char>, usize, usize), line: &str| -> (HashSet<char>, usize, usize) {
        let h: HashSet<char> = HashSet::from_iter(line.chars());
        match accu.1 % 3 {
            0 => {
                (h, accu.1 + 1, accu.2)
            }
            1 => {
                let is = accu.0.intersection(&h);
                let b: HashSet<char> = HashSet::from_iter(is.map(|x| *x));
                (b, accu.1 + 1, accu.2)
            }
            2 => {
                let common = accu.0.intersection(&h);
                let p = priority(common.last().unwrap()) as usize;
                (HashSet::new(), accu.1 + 1, accu.2 + p)
            }
            _ => unreachable!("miracle")
        }
    });
    Some(a.2)
}

fn main() {
    let c = content().unwrap();

    let start = std::time::Instant::now();
    let a = solution_a(&c);
    let dur1 = start.elapsed().as_nanos();

    let start = std::time::Instant::now();
    let b = solution_b(&c);
    let dur2 = start.elapsed().as_nanos();

    println!("Step A: {dur1} ns {:?}", a);
    println!("Step B: {dur2} ns {:?}", b);
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
        assert_eq!(solution_a(&data), Some(157));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(70));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(7568));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(2780));
    }
}
