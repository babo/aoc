use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn get_parts(input: &str) -> Vec<(usize, usize, usize, usize)> {
    input
        .lines()
        .filter(|x| x.starts_with('{'))
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit() || *c == ',')
                .join("")
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    CondAccept((char, bool, usize)),
    CondReject((char, bool, usize)),
    CondJump((char, bool, usize, String)),
    Accept,
    Reject,
    Jump(String),
}

impl Instruction {
    fn new(x: &str) -> Self {
        if x.contains(':') {
            let part = x.chars().next().unwrap();
            let is_less = x.chars().any(|c| c == '<');
            let val = x
                .chars()
                .filter(|c| c.is_ascii_digit())
                .join("")
                .parse::<usize>()
                .unwrap();
            let last = x.chars().skip_while(|c| *c != ':').skip(1).join("");

            if last == "A" {
                Instruction::CondAccept((part, is_less, val))
            } else if last == "R" {
                Instruction::CondReject((part, is_less, val))
            } else {
                Instruction::CondJump((part, is_less, val, last))
            }
        } else if x == "R" {
            Instruction::Reject
        } else if x == "A" {
            Instruction::Accept
        } else {
            Instruction::Jump(x.to_string())
        }
    }
}

type ValRange = (
    (usize, usize),
    (usize, usize),
    (usize, usize),
    (usize, usize),
);

fn total(v: &ValRange) -> usize {
    (v.0 .1 + 1 - v.0 .0) * (v.1 .1 + 1 - v.1 .0) * (v.2 .1 + 1 - v.2 .0) * (v.3 .1 + 1 - v.3 .0)
}

fn replace(r: &ValRange, c: char, v: (usize, usize)) -> ValRange {
    match c {
        'x' => (v, r.1, r.2, r.3),
        'm' => (r.0, v, r.2, r.3),
        'a' => (r.0, r.1, v, r.3),
        's' => (r.0, r.1, r.2, v),
        _ => unimplemented!("What a xmas!"),
    }
}

fn combo(values: ValRange, name: &str, workflows: &HashMap<String, Vec<Instruction>>) -> usize {
    let instructions = workflows.get(name).unwrap();

    instructions
        .iter()
        .fold((values, 0), |(val, accu), instr| match instr {
            Instruction::Accept => (val, accu + total(&val)),
            Instruction::Reject => (val, accu),
            Instruction::Jump(w) => {
                let inc = combo(val, w, workflows);
                (val, accu + inc)
            }
            Instruction::CondJump(cond) => {
                let v = match cond.0 {
                    'x' => val.0,
                    'm' => val.1,
                    'a' => val.2,
                    's' => val.3,
                    _ => unimplemented!("What a xmas!"),
                };
                if cond.1 {
                    if v.0 >= cond.2 {
                        (val, accu)
                    } else {
                        let a = (v.0, cond.2 - 1);
                        assert!(a.0 < a.1);
                        let inc = combo(replace(&val, cond.0, a), &cond.3, workflows);
                        let n = (cond.2, v.1);
                        assert!(n.0 < n.1);
                        (replace(&val, cond.0, n), accu + inc)
                    }
                } else if v.1 <= cond.2 {
                    (val, accu)
                } else {
                    let a = (cond.2 + 1, v.1);
                    assert!(a.0 < a.1);
                    let inc = combo(replace(&val, cond.0, a), &cond.3, workflows);
                    let n = (v.0, cond.2);
                    assert!(n.0 < n.1);
                    (replace(&val, cond.0, n), accu + inc)
                }
            }
            Instruction::CondAccept(cond) => {
                let v = match cond.0 {
                    'x' => val.0,
                    'm' => val.1,
                    'a' => val.2,
                    's' => val.3,
                    _ => unimplemented!("What a xmas!"),
                };
                if cond.1 {
                    if v.0 >= cond.2 {
                        (val, accu)
                    } else {
                        let a = (v.0, cond.2 - 1);
                        assert!(a.0 < a.1);
                        let inc = total(&replace(&val, cond.0, a));
                        let n = (cond.2, v.1);
                        assert!(n.0 < n.1);
                        (replace(&val, cond.0, n), accu + inc)
                    }
                } else if v.1 <= cond.2 {
                    (val, accu)
                } else {
                    let a = (cond.2 + 1, v.1);
                    assert!(a.0 < a.1);
                    let inc = total(&replace(&val, cond.0, a));
                    let n = (v.0, cond.2);
                    assert!(n.0 < n.1);
                    (replace(&val, cond.0, n), accu + inc)
                }
            }
            Instruction::CondReject(cond) => {
                let v = match cond.0 {
                    'x' => val.0,
                    'm' => val.1,
                    'a' => val.2,
                    's' => val.3,
                    _ => unimplemented!("What a xmas!"),
                };
                if cond.1 {
                    if v.0 >= cond.2 {
                        (val, accu)
                    } else {
                        let n = (cond.2, v.1);
                        assert!(n.0 < n.1);
                        (replace(&val, cond.0, n), accu)
                    }
                } else if v.1 <= cond.2 {
                    (val, accu)
                } else {
                    let n = (v.0, cond.2);
                    assert!(n.0 < n.1);
                    (replace(&val, cond.0, n), accu)
                }
            }
        })
        .1
}

fn eval_part(
    part: (usize, usize, usize, usize),
    workflows: &HashMap<String, Vec<Instruction>>,
) -> bool {
    let mut rule = "in".to_string();
    loop {
        let instructions = workflows.get(&rule).unwrap();
        let actual = instructions
            .iter()
            .find(|x| match x {
                Instruction::CondJump(cond) => {
                    let v = match cond.0 {
                        'x' => part.0,
                        'm' => part.1,
                        'a' => part.2,
                        's' => part.3,
                        _ => unimplemented!("What a xmas!"),
                    };
                    (cond.1 && v < cond.2) || (!cond.1 && v > cond.2)
                }
                Instruction::CondAccept(cond) => {
                    let v = match cond.0 {
                        'x' => part.0,
                        'm' => part.1,
                        'a' => part.2,
                        's' => part.3,
                        _ => unimplemented!("What a xmas!"),
                    };
                    (cond.1 && v < cond.2) || (!cond.1 && v > cond.2)
                }
                Instruction::CondReject(cond) => {
                    let v = match cond.0 {
                        'x' => part.0,
                        'm' => part.1,
                        'a' => part.2,
                        's' => part.3,
                        _ => unimplemented!("What a xmas!"),
                    };
                    (cond.1 && v < cond.2) || (!cond.1 && v > cond.2)
                }
                _ => true,
            })
            .unwrap();
        match actual {
            Instruction::Accept => return true,
            Instruction::Reject => return false,
            Instruction::Jump(w) => rule = w.clone(),
            Instruction::CondAccept(_) => return true,
            Instruction::CondReject(_) => return false,
            Instruction::CondJump(w) => rule = w.3.clone(),
        }
    }
}

fn get_workflows(input: &str) -> HashMap<String, Vec<Instruction>> {
    input
        .lines()
        .take_while(|x| !x.is_empty())
        .fold(HashMap::new(), |mut dict, line| {
            let name = line.chars().take_while(|c| *c != '{').join("");
            let desc = line
                .chars()
                .skip_while(|c| *c != '{')
                .skip(1)
                .take_while(|c| *c != '}')
                .join("");
            let instr = desc.split(',').map(Instruction::new).collect_vec();
            dict.insert(name, instr);
            dict
        })
}

fn ratings(part: &(usize, usize, usize, usize)) -> usize {
    part.0 + part.1 + part.2 + part.3
}

fn clean(input: &str) -> String {
    input.lines().map(|x| x.trim()).join("\n")
}

fn solution_a(input: &str) -> Option<usize> {
    let input = clean(input);
    let parts = get_parts(&input);
    let workflows = get_workflows(&input);

    Some(
        parts
            .iter()
            .filter(|part| eval_part(**part, &workflows))
            .map(ratings)
            .sum::<usize>(),
    )
}

fn solution_b(input: &str) -> Option<usize> {
    let input = clean(input);
    let workflows = get_workflows(&input);

    let initial = ((1, 4000), (1, 4000), (1, 4000), (1, 4000));
    Some(combo(initial, "in", &workflows))
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
        assert_eq!(solution_a(&data), Some(19114));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(167409079868000));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(425811));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(131796824371749));
    }
}
