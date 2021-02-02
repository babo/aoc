use regex::RegexSet;
use std::fs::read_to_string;

#[macro_use]
extern crate lazy_static;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn partition_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .partition(|&x| match x.chars().next() {
            Some(n) => n.is_numeric(),
            _ => false,
        })
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Rule {
    Term(char),
    A1(usize),
    A2(usize, usize),
    A3(usize, usize, usize),
    A4(usize, usize, usize, usize),
    O2(usize, usize),
    O4(usize, usize, usize, usize),
    L3(usize, usize, usize),
    L5(usize, usize, usize, usize, usize),
}

struct RulesWithSize {
    rules: Vec<Rule>,
}

impl RulesWithSize {
    pub fn from_str(input: &str) -> Self {
        let parts = partition_input(input);
        let mut raw: Vec<(usize, Rule)> = parts.0.iter().map(|&x| to_rule(x)).collect();
        raw.sort_by_key(|x| x.0);
        let max_index = raw.iter().map(|x| x.0).max().unwrap();
        let mut ready = vec![Rule::Term('x'); max_index + 1];
        raw.iter().for_each(|x| ready[x.0] = x.1);
        RulesWithSize { rules: ready }
    }

    pub fn is_a_match(&self, pos: usize, input: Option<&str>) -> Option<String> {
        match self.rules[pos] {
            Rule::Term(c) => input.and_then(|x| x.get(..1)).and_then(|h| {
                if h == String::from(c) {
                    input
                        .and_then(|x| x.get(1..))
                        .and_then(|x| Some(String::from(x)))
                } else {
                    None
                }
            }),
            Rule::A1(a1) => self.is_a_match(a1, input),
            Rule::A2(a1, a2) => match self.rules[a1] {
                Rule::L3(_, _, _) => {
                    input
                        .and_then(|x| self.is_a_match(a1, Some(x)))
                        .and_then(|x| {
                            let line = String::from(input.unwrap());
                            let mut n = line.len() - x.len();

                            loop {
                                match self.is_a_match(a1, line.get(0..n)).and_then(|rest| {
                                    let l = n - rest.len();
                                    self.is_a_match(a2, line.get(l..))
                                }) {
                                    Some(rest) => {
                                        if rest.is_empty() {
                                            return Some(rest);
                                        }
                                    }
                                    None => (),
                                };

                                if n > 2 {
                                    n -= 1;
                                } else {
                                    break None;
                                }
                            }
                        })
                }
                _ => input
                    .and_then(|x| self.is_a_match(a1, Some(x)))
                    .and_then(|x| self.is_a_match(a2, Some(&x))),
            },
            Rule::A3(a1, a2, a3) => input
                .and_then(|x| self.is_a_match(a1, Some(x)))
                .and_then(|x| self.is_a_match(a2, Some(&x)))
                .and_then(|x| self.is_a_match(a3, Some(&x))),
            Rule::A4(a1, a2, a3, a4) => input
                .and_then(|x| self.is_a_match(a1, Some(x)))
                .and_then(|x| self.is_a_match(a2, Some(&x)))
                .and_then(|x| self.is_a_match(a3, Some(&x)))
                .and_then(|x| self.is_a_match(a4, Some(&x))),
            Rule::O2(a1, a2) => input
                .and(self.is_a_match(a1, input))
                .or(input.and(self.is_a_match(a2, input))),
            Rule::O4(a1, a2, a3, a4) => input
                .and(self.is_a_match(a1, input))
                .and_then(|x| self.is_a_match(a2, Some(&x)))
                .or(input
                    .and(self.is_a_match(a3, input))
                    .and_then(|x| self.is_a_match(a4, Some(&x)))),
            Rule::L3(a1, _a2, _a3) => input.and_then(|input| {
                let mut w = String::from(input);

                loop {
                    match self.is_a_match(a1, Some(&w)) {
                        Some(data) => w = data,
                        None => break,
                    }
                }
                if w.len() == input.len() {
                    None
                } else {
                    Some(w)
                }
            }),
            Rule::L5(a1, _a2, _a3, _a4, a5) => input.and_then(|input| {
                let mut w = String::from(input);
                let mut n = 0;

                loop {
                    match self.is_a_match(a1, Some(&w)) {
                        Some(data) => {
                            n += 1;
                            w = data;
                        }
                        None => break,
                    }
                }
                if n == 0 {
                    return None;
                }
                for _ in 0..n {
                    match self.is_a_match(a5, Some(&w)) {
                        Some(data) => {
                            w = data;
                        }
                        None => {
                            return None;
                        }
                    }
                }
                Some(w)
            }),
        }
    }
}

fn to_rule(input: &str) -> (usize, Rule) {
    lazy_static! {
        static ref RE_RULES: RegexSet = RegexSet::new(&[
            r#"^(\d+): "([[:alpha:]])"$"#,
            r"^(\d+):( \d+)+ \|( \d+)+$",
            r"^(\d+):( \d+)+$",
        ])
        .unwrap();
    }

    let which: Vec<usize> = RE_RULES.matches(input).into_iter().collect();
    assert_eq!(which.len(), 1);
    let col = input.find(':').unwrap();
    let id = input.get(0..col).unwrap().parse::<usize>().unwrap();

    if which[0] == 0 {
        let c = input.chars().nth(input.find('"').unwrap() + 1).unwrap();
        (id, Rule::Term(c))
    } else {
        let n: Vec<usize> = input
            .get(col + 1..)
            .unwrap()
            .split(' ')
            .map(|x| x.trim())
            .filter(|&x| x != "" && x != "|")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        match which.first() {
            Some(1) => match n.len() {
                5 => (id, Rule::L5(n[0], n[1], n[2], n[3], n[4])),
                4 => (id, Rule::O4(n[0], n[1], n[2], n[3])),
                3 => (id, Rule::L3(n[0], n[1], n[2])),
                2 => (id, Rule::O2(n[0], n[1])),
                _ => panic!("Invalid args"),
            },
            Some(2) => match n.len() {
                1 => (id, Rule::A1(n[0])),
                2 => (id, Rule::A2(n[0], n[1])),
                3 => (id, Rule::A3(n[0], n[1], n[2])),
                4 => (id, Rule::A4(n[0], n[1], n[2], n[3])),
                w @ _ => panic!("Should never have {}", w),
            },
            _ => panic!("Should never reach here"),
        }
    }
}

fn solution_a(input: &str) -> usize {
    let parts = partition_input(input);
    let rules = RulesWithSize::from_str(input);

    parts
        .1
        .iter()
        .filter(|x| {
            rules
                .is_a_match(0, Some(x))
                .and_then(|x| Some(x.is_empty()))
                .or(Some(false))
                .unwrap()
        })
        .count()
}

fn solution_b(input: &str) -> usize {
    let parts = partition_input(input);
    let mut rules = RulesWithSize::from_str(input);
    rules.rules[8] = Rule::L3(42, 42, 8);
    rules.rules[11] = Rule::L5(42, 31, 42, 11, 31);

    parts
        .1
        .iter()
        .filter(|x| !x.is_empty())
        .filter(|x| {
            rules
                .is_a_match(0, Some(x))
                .and_then(|x| Some(x.is_empty()))
                .or(Some(false))
                .unwrap()
        })
        .count()
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

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_split() {
        let input = content().unwrap();
        let parts = partition_input(&input);
        assert_eq!(parts.0.len(), 136);
    }

    #[test]
    fn test_rules() {
        let input = content().unwrap();
        let parts = partition_input(&input);
        let c: Vec<(usize, Rule)> = parts.0.iter().map(|&x| to_rule(x)).collect();
        assert_eq!(c.len(), 136);
    }

    #[test]
    fn test_sample_a() {
        let input = r#"
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        aaaabb
        aaabab
        abbabb
        abbbab
        aabaab
        aabbbb
        abaaab
        ababbb"#;

        assert_eq!(solution_a(input), 8);
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(solution_a(&content().unwrap()), 113);
    }

    #[test]
    fn test_solution_b() {
        assert_eq!(solution_b(&content().unwrap()), 253);
    }

    #[test]
    fn test_changed_ruleset() {
        let input = r#"
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        "#;

        assert_eq!(solution_a(input), 3);
        assert_eq!(solution_b(input), 12);
    }

    #[test]
    fn test_simple_ruleset() {
        let input = r#"
            0: 8 11
            1: "a"
            10: 23 14 | 28 1
            11: 42 31 | 42 11 31
            12: 24 14 | 19 1
            13: 14 3 | 1 12
            14: "b"
            15: 1 | 14
            16: 15 1 | 14 14
            17: 14 2 | 1 7
            18: 15 15
            19: 14 1 | 14 14
            2: 1 24 | 14 4
            20: 14 14 | 1 15
            21: 14 1 | 1 14
            22: 14 14
            23: 25 1 | 22 14
            24: 14 1
            25: 1 1 | 1 14
            26: 14 22 | 1 20
            27: 1 6 | 14 18
            28: 16 1
            3: 5 14 | 16 1
            31: 14 17 | 1 13
            4: 1 1
            42: 9 14 | 10 1
            5: 1 14 | 15 1
            6: 14 14 | 1 14
            7: 14 5 | 1 21
            8: 42 | 42 8
            9: 14 27 | 1 26

            ababaaaaaabaaab
        "#;

        assert_eq!(solution_a(input), 1);
    }
}
