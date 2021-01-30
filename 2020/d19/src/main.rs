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
}

struct RulesWithSize {
    rules: Vec<Rule>,
    width: Vec<usize>,
}

impl RulesWithSize {
    pub fn from_str(input: &str) -> Self {
        let parts = partition_input(input);
        let mut raw: Vec<(usize, Rule)> = parts.0.iter().map(|&x| to_rule(x)).collect();
        raw.sort_by_key(|x| x.0);
        let sorted: Vec<Rule> = raw.iter().map(|x| x.1).collect();
        Self::new(&sorted)
    }

    pub fn new(data: &[Rule]) -> Self {
        let mut width = Self::cs2(0, data);
        width.sort_by_key(|x| x.0);
        width.dedup();
        RulesWithSize {
            rules: data.to_vec(),
            width: width.iter().map(|x| x.1).collect(),
        }
    }

    fn cs2(pos: usize, rules: &[Rule]) -> Vec<(usize, usize)> {
        let mut rtv = Vec::new();
        let w = match rules[pos] {
            Rule::Term(_) => 1,
            Rule::A1(a1) => {
                let w1 = Self::cs2(a1, rules);
                let w = w1.last().unwrap().1;
                rtv.extend(w1);

                w
            }
            Rule::A2(a1, a2) => {
                let w1 = Self::cs2(a1, rules);
                let w2 = Self::cs2(a2, rules);

                let w = w1.last().unwrap().1 + w2.last().unwrap().1;
                rtv.extend(w1);
                rtv.extend(w2);
                w
            }
            Rule::A3(a1, a2, a3) => {
                let w1 = Self::cs2(a1, rules);
                let w2 = Self::cs2(a2, rules);
                let w3 = Self::cs2(a3, rules);

                let w = w1.last().unwrap().1 + w2.last().unwrap().1 + w3.last().unwrap().1;
                rtv.extend(w1);
                rtv.extend(w2);
                rtv.extend(w3);
                w
            }
            Rule::A4(a1, a2, a3, a4) => {
                let w1 = Self::cs2(a1, rules);
                let w2 = Self::cs2(a2, rules);
                let w3 = Self::cs2(a3, rules);
                let w4 = Self::cs2(a4, rules);

                let w = w1.last().unwrap().1
                    + w2.last().unwrap().1
                    + w3.last().unwrap().1
                    + w4.last().unwrap().1;
                rtv.extend(w1);
                rtv.extend(w2);
                rtv.extend(w3);
                rtv.extend(w4);
                w
            }
            Rule::O2(a1, a2) => {
                let w1 = Self::cs2(a1, rules);
                let w2 = Self::cs2(a2, rules);

                let l = w1.last().unwrap().1;
                let r = w2.last().unwrap().1;
                if l != r {
                    panic!("Expected to be equal: {} {}", l, r);
                }
                rtv.extend(w1);
                rtv.extend(w2);
                l
            }
            Rule::O4(a1, a2, a3, a4) => {
                let w1 = Self::cs2(a1, rules);
                let w2 = Self::cs2(a2, rules);
                let w3 = Self::cs2(a3, rules);
                let w4 = Self::cs2(a4, rules);
                let l = w1.last().unwrap().1 + w2.last().unwrap().1;
                let r = w3.last().unwrap().1 + w4.last().unwrap().1;

                if l != r {
                    panic!("Expected to be equal: {} {}", l, r);
                }
                rtv.extend(w1);
                rtv.extend(w2);
                rtv.extend(w3);
                rtv.extend(w4);
                l
            }
        };
        rtv.push((pos, w));
        rtv
    }

    pub fn is_a_match(&self, pos: usize, input: &str) -> bool {
        match self.rules[pos] {
            Rule::Term(c) => input == String::from(c),
            Rule::A1(a1) => self.is_a_match(a1, input),
            Rule::A2(a1, a2) => {
                let w1 = self.width[a1];
                let w2 = self.width[a2];
                input.len() == w1 + w2
                    && self.is_a_match(a1, input.get(..w1).unwrap())
                    && self.is_a_match(a2, input.get(w1..w1 + w2).unwrap())
            }
            Rule::A3(a1, a2, a3) => {
                let w1 = self.width[a1];
                let w2 = self.width[a2];
                let w3 = self.width[a3];

                input.len() == w1 + w2 + w3
                    && self.is_a_match(a1, input.get(..w1).unwrap())
                    && self.is_a_match(a2, input.get(w1..w1 + w2).unwrap())
                    && self.is_a_match(a3, input.get(w1 + w2..w1 + w2 + w3).unwrap())
            }
            Rule::A4(a1, a2, a3, a4) => {
                let w1 = self.width[a1];
                let w2 = self.width[a2];
                let w3 = self.width[a3];
                let w4 = self.width[a4];
                input.len() == w1 + w2 + w3 + w4
                    && self.is_a_match(a1, input.get(..w1).unwrap())
                    && self.is_a_match(a2, input.get(w1..w1 + w2).unwrap())
                    && self.is_a_match(a2, input.get(w1 + w2..w1 + w2 + w3).unwrap())
                    && self.is_a_match(a2, input.get(w1 + w2 + w3..w1 + w2 + w3 + w4).unwrap())
            }
            Rule::O2(a1, a2) => {
                let w1 = self.width[a1];
                input.len() == w1 && (self.is_a_match(a1, input) || self.is_a_match(a2, input))
            }
            Rule::O4(a1, a2, a3, a4) => {
                let w1 = self.width[a1];
                let w2 = self.width[a2];
                input.len() == w1 + w2
                    && ((self.is_a_match(a1, input.get(..w1).unwrap())
                        && self.is_a_match(a2, input.get(w1..w1 + w2).unwrap()))
                        || (self.is_a_match(a3, input.get(..w1).unwrap())
                            && self.is_a_match(a4, input.get(w1..w1 + w2).unwrap())))
            }
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
            Some(1) => {
                if n.len() == 4 {
                    (id, Rule::O4(n[0], n[1], n[2], n[3]))
                } else {
                    (id, Rule::O2(n[0], n[1]))
                }
            }
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

    parts.1.iter().filter(|x| rules.is_a_match(0, x)).count()
}

fn solution_b(_input: &str) -> u64 {
    0
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {:?}", a);
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
    fn test_width() {
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
        let rules = RulesWithSize::from_str(input);
        assert_eq!(rules.width[0], 6);
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
}
