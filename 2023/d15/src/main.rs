use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn rhash(input: &str) -> usize {
    input
        .trim()
        .chars()
        .fold(0, |accu, x| ((accu + (x as u8) as usize) * 17) % 256)
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.trim().split(',').map(rhash).sum())
}

struct Boxes {
    boxes: Vec<Vec<(String, u8)>>,
}

impl Boxes {
    fn new() -> Self {
        let boxes = std::iter::repeat(0)
            .take(256)
            .map(|_| Vec::new())
            .collect_vec();
        Boxes { boxes }
    }

    fn instruction(&mut self, code: &str) {
        let is_remove = code.ends_with('-');
        let label = if is_remove {
            code.strip_suffix('-').unwrap()
        } else {
            &code[..code.len() - 2]
        };
        let n = rhash(label);
        if let Some(boxi) = self.boxes.get_mut(n) {
            if is_remove {
                if let Some(index) = boxi.iter().enumerate().find_map(|x| {
                    if x.1 .0 == label {
                        Some(x.0)
                    } else {
                        None
                    }
                }) {
                    boxi.remove(index);
                };
            } else {
                let focal = code.chars().last().unwrap().to_digit(10).unwrap() as u8;
                if let Some(index) = boxi.iter().enumerate().find_map(|x| {
                    if x.1 .0 == label {
                        Some(x.0)
                    } else {
                        None
                    }
                }) {
                    if let Some(x) = boxi.get_mut(index) {
                        x.1 = focal;
                    }
                } else {
                    boxi.push((label.to_string(), focal));
                }
            }
        }
    }

    fn power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(bnum, boxi)| {
                boxi.iter()
                    .enumerate()
                    .map(|slot| ((bnum + 1) * (slot.0 + 1) * slot.1 .1 as usize))
                    .sum::<usize>()
            })
            .sum()
    }

    fn debug(&self) {
        self.boxes.iter().enumerate().for_each(|b| {
            if !b.1.is_empty() {
                println!("{} {:?}", b.0, b.1)
            }
        })
    }
}

fn solution_b(input: &str) -> Option<usize> {
    let store = input
        .trim()
        .split(',')
        .fold(Boxes::new(), |mut store, code| {
            store.instruction(code);
            store
        });
    store.debug();
    Some(store.power())
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
    fn test_hash() {
        assert_eq!(rhash("ab"), 3);
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(1320));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(145));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(517551));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(286097));
    }
}
