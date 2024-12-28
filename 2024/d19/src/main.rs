use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> usize {
    let patterns = input
        .trim()
        .lines()
        .take(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(", ")
        .collect::<Vec<&str>>();

    input
        .trim()
        .lines()
        .skip(2)
        .map(|design| {
            let n = design.len();
            let mut i = 0;
            let mut track: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut stack: Vec<usize> = Vec::new();
            while i < n {
                if !track.contains_key(&i) {
                    let start_char = design.chars().nth(i).unwrap();
                    let m = patterns
                        .iter()
                        .filter(|x| x.starts_with(start_char))
                        .map(|x| {
                            let l = x.len();
                            if design[i..].starts_with(x) {
                                l
                            } else {
                                0
                            }
                        })
                        .filter(|x| *x > 0)
                        .collect_vec();
                    track.insert(i, m);
                }
                if let Some(v) = track.get_mut(&i) {
                    if let Some(x) = v.pop() {
                        stack.push(x);
                        i += x;
                    } else if stack.is_empty() {
                        return 0;
                    } else {
                        i -= stack.pop().unwrap();
                    }
                } else {
                    unimplemented!("A vector should be present");
                }
            }
            1
        })
        .sum::<usize>()
}

fn tricky(design: &str, patterns: &Vec<&str>, memo: &mut HashMap<String, usize>) {
    if design.is_empty() || memo.contains_key(design) {
        return;
    }

    let count = patterns.iter().fold(0, |acc, p| {
        if design.starts_with(p) {
            let right = if p.len() == design.len() {
                1
            } else {
                let rsub = &design[p.len()..];
                if !memo.contains_key(rsub) {
                    tricky(rsub, patterns, memo);
                }
                *memo.get(rsub).unwrap()
            };
            acc + right
        } else {
            acc
        }
    });

    println!("|{}| {}", design, count);
    memo.insert(design.to_string(), count);
}

fn solution_b(input: &str) -> usize {
    let patterns = input
        .trim()
        .lines()
        .take(1)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(", ")
        .collect::<Vec<&str>>();

    let body = input.trim().lines().skip(2).join("\n");

    let patterns: Vec<&str> = patterns
        .iter()
        .filter(|p| body.contains(**p))
        .sorted_by(|a, b| a.len().cmp(&b.len()))
        .copied()
        .collect();

    let mut memo: HashMap<String, usize> = HashMap::new();

    body.lines()
        .map(|design| {
            let p = patterns
                .iter()
                .filter(|x| design.contains(**x))
                .copied()
                .collect();
            let design = design.trim().to_string();
            tricky(design.as_str(), &p, &mut memo);
            let val = *memo.get(&design).unwrap();
            println!("design: |{}| {}", design, val);

            val
        })
        .sum()
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
        assert_eq!(solution_a(&data), 6);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 16);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 333);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 678536865274732);
    }

    #[test]
    fn test_tricky_1() {
        let patterns = vec!["a", "b", "ab", "ba", "bc", "cd", "d"];
        let mut memo: HashMap<String, usize> = HashMap::from_iter(
            patterns
                .iter()
                .map(|p| (p.to_string(), naive_b(p, &patterns))),
        );
        println!("{:?}", memo);
        let design = "aba".to_string();
        tricky(design.as_str(), &patterns, &mut memo);
        assert_eq!(memo.get(&design), Some(&3));
    }

    #[test]
    fn test_tricky_2() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 16);
    }

    #[test]
    fn test_tricky_3() {
        let data = "r, wr, b, g, bwu, rb, gb, br

        gbbr
        rrbgbr";

        assert_eq!(solution_b(&data), 10);
    }
}
