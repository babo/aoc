use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self};

#[macro_use]
extern crate lazy_static;

#[derive(Copy, Clone)]
enum Inst {
    NOP(i32),
    JMP(i32),
    ACC(i32),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_file_reading() -> Result<(), io::Error> {
        assert_ne!(content()?.len(), 0);
        Ok(())
    }

    #[test]
    fn test_step_a() -> Result<(), io::Error> {
        let c = content().unwrap();
        let inst = read_code(&c);
        let res = find_loop(&inst);
        assert_eq!(res, 1087);
        Ok(())
    }

    #[test]
    fn test_step_b() -> Result<(), io::Error> {
        let c = content().unwrap();
        let inst = read_code(&c);
        let res = fix_code(&inst);
        assert_eq!(res, 780);
        Ok(())
    }
}

fn main() {
    let c = content().unwrap();
    let inst = read_code(&c);
    let a = find_loop(&inst);
    let b = fix_code(&inst);
    println!("Step A: {}", a);
    println!("Step B: {}", b);
}

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn read_code(content: &str) -> Vec<Inst> {
    lazy_static! {
        static ref RE_SINGLE: Regex = Regex::new(r"^(nop|acc|jmp) ([+-]\d+)$").unwrap();
    }
    let p: Vec<Inst> = content
        .lines()
        .map(|line| -> Inst {
            let raw = RE_SINGLE.captures(line);
            let cap = raw.expect(&format!("Line doesn't match: {}", line));
            let left = cap.get(1).unwrap().as_str();
            let right = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

            match left {
                "nop" => Inst::NOP(right),
                "acc" => Inst::ACC(right),
                "jmp" => Inst::JMP(right),
                _ => panic!("Should not happen, already catched"),
            }
        })
        .collect();
    p
}

fn find_loop(inst: &Vec<Inst>) -> i32 {
    let mut pos = 0i32;
    let mut acc = 0i32;
    let mut seen = HashSet::new();

    loop {
        seen.insert(pos);
        match inst[pos as usize] {
            Inst::NOP(_) => pos += 1,
            Inst::JMP(p) => pos += p,
            Inst::ACC(a) => {
                pos += 1;
                acc += a;
            }
        }
        if seen.contains(&pos) {
            break;
        }
    }
    acc
}

fn fix_code(inst: &Vec<Inst>) -> i32 {
    let mut acc: i32;
    let mut pos = 0i32;
    let mut seen = HashSet::new();

    loop {
        seen.insert(pos);
        match inst[pos as usize] {
            Inst::NOP(_) => pos += 1,
            Inst::JMP(p) => pos += p,
            Inst::ACC(_) => pos += 1,
        }
        if seen.contains(&pos) {
            break;
        }
    }
    // One of these
    let positions: Vec<i32> = seen
        .iter()
        .filter(|x| match inst[**x as usize] {
            Inst::ACC(_) => false,
            _ => true,
        })
        .map(|x| *x)
        .collect();

    for fix_at in positions {
        acc = 0;
        pos = 0;
        seen.clear();

        loop {
            seen.insert(pos);
            if pos != fix_at {
                match inst[pos as usize] {
                    Inst::NOP(_) => pos += 1,
                    Inst::JMP(p) => pos += p,
                    Inst::ACC(a) => {
                        acc += a;
                        pos += 1;
                    }
                }
            } else {
                match inst[pos as usize] {
                    Inst::NOP(p) => pos += p,
                    Inst::JMP(_) => pos += 1,
                    Inst::ACC(_) => panic!("ACC should be filtered"),
                }
            }
            if seen.contains(&pos) {
                break;
            }
            if pos == inst.len() as i32 {
                return acc;
            }
        }
    }
    panic!("No solution?")
}
