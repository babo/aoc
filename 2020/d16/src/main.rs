use regex::Regex;
use std::fs::read_to_string;

#[macro_use]
extern crate lazy_static;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn read_instructions(input: &str) -> (Vec<(String, u16, u16, u16, u16)>, Vec<u16>, Vec<Vec<u16>>) {
    lazy_static! {
        static ref RE_RULE: Regex =
            Regex::new(r"^([a-z ]+):\s+(\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        static ref RE_TICKET: Regex = Regex::new(r"^((\d+),)+(\d+)$").unwrap();
    }

    let mut rules: Vec<(String, u16, u16, u16, u16)> = Vec::new();
    let mut others: Vec<Vec<u16>> = Vec::new();
    let mut ticket: Vec<u16> = Vec::new();
    let mut state = 0;

    for raw in input.lines() {
        let line = raw.trim();
        match line {
            "" => (),
            "your ticket:" => {
                state = 1;
            }
            "nearby tickets:" => {
                state = 2;
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
                } else if RE_TICKET.is_match(line) {
                    let numbers: Vec<u16> =
                        line.split(",").map(|x| x.parse::<u16>().unwrap()).collect();
                    if state == 1 {
                        ticket.extend_from_slice(&numbers);
                    } else {
                        others.push(numbers);
                    }
                }
            }
        }
    }

    (rules, ticket, others)
}

fn solution(input: &str) -> (u16, u64) {
    lazy_static! {
        static ref RE_RULE: Regex =
            Regex::new(r"^([a-z ]+):\s+(\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        static ref RE_TICKET: Regex = Regex::new(r"^((\d+),)+(\d+)$").unwrap();
    }

    let (rules, ticket, others) = read_instructions(input);

    let mut lookup = [0u32; 1024];
    for rn in rules.iter().enumerate() {
        let j = 2u32.pow(rn.0 as u32);
        let n = rn.1;
        for i in n.1..=n.2 {
            lookup[i as usize] |= j;
        }
        for i in n.3..=n.4 {
            lookup[i as usize] |= j;
        }
    }

    let mut sol_a = 0u16;
    let mut results = vec![u32::MAX; rules.len()];
    for ticket in others.iter() {
        let choices: Vec<u32> = ticket.iter().map(|x| lookup[*x as usize]).collect();
        if choices.iter().find(|x| **x == 0) == None {
            for x in choices.iter().enumerate() {
                results[x.0] &= x.1;
            }
        } else {
            sol_a = ticket
                .iter()
                .filter(|x| lookup[**x as usize] == 0)
                .fold(sol_a, |acc, x| acc + x);
        }
    }

    let sol_b = clean_up(&results)
        .iter()
        .map(|x| x.trailing_zeros())
        .enumerate()
        .fold(1u64, |acc, x| {
            if rules[x.1 as usize].0.starts_with("departure") {
                let h = ticket[x.0];
                println!("{} -> {}", x.0, h);
                acc * h as u64
            } else {
                acc
            }
        });

    (sol_a, sol_b)
}

fn clean_up(results: &[u32]) -> Vec<u32> {
    let mut current: Vec<u32> = Vec::new();
    current.extend_from_slice(results);

    let parts: (Vec<u32>, Vec<u32>) = results
        .iter()
        .map(|x| *x)
        .partition(|x| x.count_ones() == 1);
    if parts.1.len() == 0 {
        current
    } else {
        for y in parts.0 {
            current = current
                .iter()
                .map(|x| if *x == y { *x } else { *x & !y })
                .collect();
        }
        clean_up(&current)
    }
}

fn solution_a(input: &str) -> u16 {
    solution(input).0
}

fn solution_b(input: &str) -> u64 {
    solution(input).1
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
    fn test_solution_b() {
        let input = content().unwrap();

        assert_eq!(solution_b(&input), 2843534243843u64);
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

    #[test]
    fn test_b() {
        let input = "class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9";

        assert_eq!(solution_b(&input), 1);
    }
}
