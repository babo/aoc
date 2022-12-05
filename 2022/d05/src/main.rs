use std::collections::VecDeque;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> String {
    let mut cranes: [VecDeque<char>; 9] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    input
        .lines()
        .filter(|line| line.contains('['))
        .for_each(|line| {
            for i in 0..9 {
                let p = 1 + i * 4;
                if p < line.len() {
                    match line.chars().nth(p) {
                        Some(c) if c != ' ' => cranes[i].push_back(c),
                        _ => (),
                    }
                }
            }
        });

    input
        .lines()
        .filter(|line| line.contains("move"))
        .for_each(|line| {
            let mut p = line.split(' ');
            p.next();
            let count = p
                .next()
                .map(|x| u16::from_str_radix(x, 10).unwrap())
                .unwrap();
            p.next();
            let f = p
                .next()
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .unwrap();
            p.next();
            let t = p
                .next()
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .unwrap();

            println!("move {count} from {f} to {t}");

            for _ in 0..count {
                match cranes[f - 1].pop_front() {
                    Some(c) => {
                        println!("{f} => {c} => {t}");
                        cranes[t - 1].push_front(c);
                    }
                    None => unimplemented!("WTH"),
                }
            }
        });

    let mut ret = (0..9)
        .map(|x| cranes[x].pop_front().map_or(' ', |c| c))
        .join("");
    ret.replace(" ", "")
}

fn solution_b(input: &str) -> String {
    let mut cranes: [VecDeque<char>; 9] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    input
        .lines()
        .filter(|line| line.contains('['))
        .for_each(|line| {
            for i in 0..9 {
                let p = 1 + i * 4;
                if p < line.len() {
                    match line.chars().nth(p) {
                        Some(c) if c != ' ' => cranes[i].push_back(c),
                        _ => (),
                    }
                }
            }
        });

    input
        .lines()
        .filter(|line| line.contains("move"))
        .for_each(|line| {
            let mut p = line.split(' ');
            p.next();
            let count = p
                .next()
                .map(|x| u16::from_str_radix(x, 10).unwrap())
                .unwrap();
            p.next();
            let f = p
                .next()
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .unwrap();
            p.next();
            let t = p
                .next()
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .unwrap();

            println!("move {count} from {f} to {t}");

            let mut q: VecDeque<char> = VecDeque::new();
            for _ in 0..count {
                match cranes[f - 1].pop_front() {
                    Some(c) => {
                        println!("{f} => {c} => {t}");
                        q.push_front(c);
                    }
                    None => unimplemented!("WTH"),
                }
            }
            for _ in 0..count {
                cranes[t-1].push_front(q.pop_front().unwrap())
            }
        });

    let ret = (0..9)
        .map(|x| cranes[x].pop_front().map_or(' ', |c| c))
        .join("");
    ret.replace(" ", "")
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
        let res = solution_a(&data);
        println!("{res}");
        assert!(res.eq("CMZ"));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        let res = solution_b(&data);
        println!("{res}");
        assert!(res.eq("MCD"));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let res = solution_a(&c);
        assert!(res.eq(""));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let res = solution_b(&c);
        assert!(res.eq(""));
    }
}
