use regex::Regex;
use std::fs::read_to_string;

#[macro_use]
extern crate lazy_static;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> u16 {
    lazy_static! {
        static ref RE_RULE: Regex =
            Regex::new(r"^([a-z ]+):\s+(\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        static ref RE_TICKET: Regex = Regex::new(r"^((\d+),)+(\d+)$").unwrap();
    }

    let mut rules: Vec<(String, u16, u16, u16, u16)> = Vec::new();
    let mut lookup = [0u32; 1024];
    let mut others: Vec<Vec<u16>> = Vec::new();
    let mut _ticket: Option<Vec<u16>> = None;
    let mut state = 0;
    for raw in input.lines() {
        let line = raw.trim();
        match line {
            "" => continue,
            "your ticket:" => {
                state = 1;
                continue;
            }
            "nearby tickets:" => {
                state = 2;
                continue;
            }
            _ => {
                if state == 0 {
                    let cap = RE_RULE
                        .captures(line)
                        .expect(&format!("Line doesn't match: {}", line));
                    let name = cap.get(1).unwrap().as_str();
                    let n: Vec<u16> = cap
                        .iter()
                        .skip(2)
                        .map(|x| x.unwrap().as_str().parse::<u16>().unwrap())
                        .collect();

                    rules.push((String::from(name), n[0], n[1], n[2], n[3]));
                    let j = 2u32.pow(rules.len() as u32);
                    for i in n[0]..=n[1] {
                        lookup[i as usize] |= j;
                    }
                    for i in n[2]..=n[3] {
                        lookup[i as usize] |= j;
                    }
                    println!("Rule {} {}", j, line);
                } else if RE_TICKET.is_match(line) {
                    let s: Vec<&str> = line.split(",").collect();
                    let numbers: Vec<u16> = s.iter().map(|x| x.parse::<u16>().unwrap()).collect();
                    if state == 1 {
                        _ticket = Some(numbers);
                    } else {
                        others.push(numbers);
                    }
                }
            }
        }
    }

    others.into_iter().flatten().filter(|x| lookup[*x as usize] == 0).sum()
}

fn solution_b(_input: &str) -> i32 {
    0
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {}", a);
    println!("Step B: {}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_solution_a() {
        let input = content().unwrap();

        assert_eq!(solution_a(&input), 20060);
    }

    #[test]
    fn test_a() {
        let input = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";

        assert_eq!(solution_a(&input), 71);
    }
}
