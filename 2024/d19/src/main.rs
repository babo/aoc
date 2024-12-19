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

    let mut pattern_map: HashMap<char, Vec<&str>> = HashMap::new();

    patterns.iter().for_each(|p| {
        let k: char = p.chars().next().unwrap();
        if let Some(v) = pattern_map.get_mut(&k) {
            v.push(p);
        } else {
            pattern_map.insert(k, vec![p]);
        }
    });
    pattern_map.values_mut().for_each(|v| v.sort());

    input
        .trim()
        .lines()
        .skip(2)
        .map(|design| {
            let mut count = 0;
            let n = design.len();
            let mut open_set = vec![0];

            while !open_set.is_empty() {
                let start = open_set.pop().unwrap();
                let start_char = design.chars().nth(start).unwrap();
                if let Some(patterns) = pattern_map.get(&start_char) {
                    for p in patterns {
                        let l = p.len();
                        if start + l <= n && design[start..].starts_with(p) {
                            let new_start = start + l;
                            if new_start == n {
                                count += 1;
                            } else {
                                open_set.push(new_start);
                            }
                        }
                    }
                }
            }
            count
        })
        .sum::<usize>()
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
        assert_eq!(solution_b(&c), 0);
    }
}
