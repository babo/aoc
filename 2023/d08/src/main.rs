use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn as_number(x: (char, char, char)) -> u32 {
    let a = ((x.0 as u8) - 65) % 26;
    let b = ((x.1 as u8) - 65) % 26;
    let c = ((x.2 as u8) - 65) % 26;

    26 * 26 * a as u32 + 26 * b as u32 + c as u32
}

fn get_instructions(input: &str) -> Vec<bool> {
    input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| x == 'L')
        .collect_vec()
}

fn get_nodes(input: &str) -> HashMap<u32, (u32, u32)> {
    input
        .lines()
        .skip(2)
        .fold(HashMap::new(), |mut accu, line| {
            let letters = line
                .trim()
                .chars()
                .filter(|x| x.is_ascii_alphanumeric())
                .collect_vec();
            let k: (char, char, char) = letters.iter().take(3).copied().collect_tuple().unwrap();
            let l: (char, char, char) = letters
                .iter()
                .skip(3)
                .take(3)
                .copied()
                .collect_tuple()
                .unwrap();
            let r: (char, char, char) = letters
                .iter()
                .skip(6)
                .take(3)
                .copied()
                .collect_tuple()
                .unwrap();
            accu.insert(as_number(k), (as_number(l), as_number(r)));
            accu
        })
}

fn solution_a(input: &str) -> Option<usize> {
    let instr = get_instructions(input);
    let nodes = get_nodes(input);

    let n = instr.len();
    let mut node = as_number(('A', 'A', 'A'));
    let goal = as_number(('Z', 'Z', 'Z'));
    for i in 0.. {
        if node == goal {
            return Some(i);
        }
        node = nodes
            .get(&node)
            .map(|lr| {
                if instr.get(i % n) == Some(&true) {
                    lr.0
                } else {
                    lr.1
                }
            })
            .unwrap();
    }
    None
}

fn ends_a(x: &u32) -> bool {
    x % 26 == 0
}

fn ends_z(x: &u32) -> bool {
    x % 26 == 25
}

fn primes(until: usize) -> Vec<usize> {
    let mut p = Vec::new();
    p.push(2);
    (3..until + 1).filter(|x| x % 2 == 1).for_each(|x| {
        if !p.iter().any(|a| (x % *a) == 0) {
            p.push(x);
        }
    });

    p
}

fn solution_b(input: &str) -> Option<usize> {
    let instr = get_instructions(input);
    let n = instr.len();

    let nodes = get_nodes(input);
    let mut numbers: Vec<usize> = nodes.keys().filter(|x| ends_a(x))
        .map(|ghost| {
            let mut ghost = *ghost;

            for i in 0.. {
                if ends_z(&ghost) {
                    return i;
                }
                let lr = instr.get(i % n) == Some(&true);
                ghost = nodes
                    .get(&ghost)
                    .map(|v| if lr { v.0 } else { v.1 })
                    .unwrap();
            }
            0usize
        })
        .unique()
        .collect_vec();

    let divisors = primes(*numbers.iter().max().unwrap());
    let mut dit = divisors.iter();
    let mut dd = dit.next();
    let mut common = 1;

    while dd.is_some() && numbers.iter().any(|x| *x != 1) {
        let d = *dd.unwrap();
        let condi = numbers.iter().any(|x| *x % d == 0);
        if condi {
            common *= d;
            numbers.iter_mut().for_each(|it| {
                if (*it) % d == 0 {
                    *it /= d;
                }
            });
        } else {
            dd = dit.next();
        }
    }
    Some(common)
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    println!("Step A: {:?}", a);

    let b = solution_b(&c);
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
        let data = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solution_a(&data), Some(6));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(6));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(16697));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(10668805667831));
    }
}
