use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let start = input.lines().next().map(|x| x.trim()).unwrap();
    let rules: HashMap<&str, char> =
        input
            .lines()
            .skip_while(|x| !x.contains("->"))
            .fold(HashMap::new(), |mut acc, x| {
                let r: Vec<&str> = x.split(" -> ").collect();
                acc.insert(
                    *r.get(0).unwrap(),
                    r.get(1).map_or(' ', |x| x.chars().next().unwrap()),
                );
                acc
            });

    let ready: String = (0..10).fold(start.to_string(), |acc, _| {
        let a = acc.chars();
        let mut b = acc.chars();
        b.next();
        let mut r: Vec<char> = a
            .zip(b)
            .map(|(a, b)| {
                let mut k = String::new();
                k.push(a);
                k.push(b);
                let n = rules.get(&k.as_str()).unwrap();
                [a, *n]
            })
            .flatten()
            .collect();
        acc.chars().last().map(|c| r.push(c));
        String::from_iter(r.iter())
    });
    let counts: Vec<usize> = ready
        .chars()
        .unique()
        .map(|c| ready.chars().filter(|x| *x == c).count())
        .collect();
    counts
        .iter()
        .max()
        .map(|max| counts.iter().min().map(|min| *max - *min))
        .unwrap()
}

fn solution_b(input: &str) -> Option<usize> {
    let start: HashMap<String, usize> = {
        let start = input.lines().next().map(|x| x.trim()).unwrap();
        start
            .chars()
            .zip(start.chars().skip(1))
            .map(|ab| String::from_iter([ab.0, ab.1].iter()))
            .fold(HashMap::new(), |mut acc, k| {
                if acc.contains_key(&k) {
                    acc.get_mut(&k).map(|v| *v += 1);
                } else {
                    acc.insert(k, 1);
                }
                acc
            })
    };
    let rules: HashMap<String, char> =
        input
            .lines()
            .skip_while(|x| !x.contains("->"))
            .fold(HashMap::new(), |mut acc, x| {
                let r: Vec<&str> = x.split(" -> ").collect();
                acc.insert(
                    String::from(*r.get(0).unwrap()),
                    r.get(1).map_or(' ', |x| x.chars().next().unwrap()),
                );
                acc
            });

    let unique: HashMap<char, usize> = HashMap::from_iter({
        let start = input
            .lines()
            .nth(0)
            .map(|line| {
                [
                    line.chars().nth(0).unwrap(),
                    line.chars().rev().nth(0).unwrap(),
                ]
            })
            .unwrap();
        let counts: Vec<(char, usize)> = input
            .chars()
            .filter(|x| x.is_uppercase())
            .unique()
            .map(|c| (c, if start.contains(&c) { 1 } else { 0 }))
            .collect();
        counts
    });

    let ready: HashMap<String, usize> = (0..40).fold(start, |acc, _| {
        acc.iter()
            .map(|kv| {
                let mid = rules.get(kv.0).unwrap();
                let kl = String::from_iter([kv.0.chars().nth(0).unwrap(), *mid]);
                let kr = String::from_iter([*mid, kv.0.chars().nth(1).unwrap()]);
                [(kl, kv.1), (kr, kv.1)]
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, kv| {
                if acc.contains_key(&kv.0) {
                    acc.get_mut(&kv.0).map(|p| *p += *kv.1);
                } else {
                    acc.insert(kv.0, *kv.1);
                }
                acc
            })
    });
    let counts: HashMap<char, usize> = ready.iter().fold(unique, |mut acc, kv| {
        kv.0.chars().for_each(|c| {
            acc.get_mut(&c).map(|x| *x += kv.1);
        });
        acc
    });
    counts
        .values()
        .max()
        .map(|max| counts.values().min().map(|min| (*max - *min) / 2))
        .unwrap()
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
        assert_eq!(solution_a(&data), Some(1588));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(2188189693529));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(2112));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(3243771149914));
    }
}
