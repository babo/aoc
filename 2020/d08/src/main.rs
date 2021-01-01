use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self};

#[macro_use]
extern crate lazy_static;

enum Inst {
    NOP,
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
        let res = execute(0, 0, &inst);
        assert_eq!(res, 1087);
        Ok(())
    }
}

fn main() {
    let c = content().unwrap();
    let inst = read_code(&c);
    let res = execute(0, 0, &inst);
    println!("Step A: {}", res);
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
            if raw.is_none() {
                println!("{}", line);
            }
            let cap = raw.unwrap();
            let left = cap.get(1).unwrap().as_str();
            let right = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

            match left {
                "nop" => Inst::NOP,
                "acc" => Inst::ACC(right),
                "jmp" => Inst::JMP(right),
                _ => Inst::NOP,
            }
        })
        .collect();
    p
}

fn execute(pos: usize, acc: i32, inst: &Vec<Inst>) -> i32 {
    let mut pos = pos as i32;
    let mut acc = acc;
    let mut seen = HashSet::new();

    loop {
        seen.insert(pos);
        match inst[pos as usize] {
            Inst::NOP => pos += 1,
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
