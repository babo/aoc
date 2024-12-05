use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let ordering = input.lines().filter(|x| x.contains("|")).fold(
        HashMap::new(),
        |mut acc: HashMap<u8, HashSet<u8>>, x| {
            let (a, b) = x
                .split("|")
                .map(|x| x.parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap();
            if let Some(v) = acc.get_mut(&a) {
                v.insert(b);
            } else {
                let mut v = HashSet::new();
                v.insert(b);
                acc.insert(a, v);
            }
            acc
        },
    );

    let rtv = input
        .lines()
        .filter(|x| x.contains(","))
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<u8>().unwrap())
                .collect_vec()
        })
        .map(|pages| {
            let mid = pages.len() / 2;
            let mut correct = true;
            for i in 1..pages.len() {
                if let Some(fb) = ordering.get(&pages[i]) {
                    for j in 0..i {
                        if fb.contains(&pages[j]) {
                            correct = false;
                            break;
                        }
                    }
                }
                if !correct {
                    break;
                }
            }
            if correct {
                pages[mid] as usize
            } else {
                0
            }
        })
        .sum();

    Some(rtv)
}

fn solution_b(input: &str) -> Option<usize> {
    let ordering = input.lines().filter(|x| x.contains("|")).fold(
        HashMap::new(),
        |mut acc: HashMap<u8, HashSet<u8>>, x| {
            let (a, b) = x
                .split("|")
                .map(|x| x.parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap();
            if let Some(v) = acc.get_mut(&a) {
                v.insert(b);
            } else {
                let mut v = HashSet::new();
                v.insert(b);
                acc.insert(a, v);
            }
            acc
        },
    );

    let rtv = input
        .lines()
        .filter(|x| x.contains(","))
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<u8>().unwrap())
                .collect_vec()
        })
        .map(|pages| {
            let mid = pages.len() / 2;
            let mut correct = true;
            for i in 1..pages.len() {
                if let Some(fb) = ordering.get(&pages[i]) {
                    for j in 0..i {
                        if fb.contains(&pages[j]) {
                            correct = false;
                            break;
                        }
                    }
                }
                if !correct {
                    break;
                }
            }
            if correct {
                0
            } else {
                let mut pages = pages.clone();
                pages.sort_by(|a, b| {
                    if let Some(v) = ordering.get(a) {
                        if v.contains(b) {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Less
                        }
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
                pages[mid] as usize
            }
        })
        .sum();

    Some(rtv)
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
        assert_eq!(solution_a(&data), Some(143));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(123));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(6949));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(4145));
    }
}
