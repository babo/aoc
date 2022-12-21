use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Formula {
    Add,
    Sub,
    Div,
    Mul,
}

struct MonkeyMath<'a> {
    wait: HashMap<&'a str, (&'a str, i8)>,
    ready: HashMap<&'a str, i64>,
    forms: HashMap<&'a str, (Formula, Option<i64>, Option<i64>)>,
}

impl<'a> MonkeyMath<'a> {
    fn new() -> Self {
        MonkeyMath {
            wait: HashMap::new(),
            ready: HashMap::new(),
            forms: HashMap::new(),
        }
    }

    fn eval(&self, name: &'a str) -> Option<i64> {
        match self.forms[name] {
            (Formula::Add, Some(a), Some(b)) => Some(a + b),
            (Formula::Sub, Some(a), Some(b)) => Some(a - b),
            (Formula::Mul, Some(a), Some(b)) => Some(a * b),
            (Formula::Div, Some(a), Some(b)) => Some(a / b),
            _ => None,
        }
    }

    fn chain(&mut self, name: &'a str, value: i64) {
        let mut name = name;
        let mut val = value;
        loop {
            self.ready.insert(name, val);
            let is_wait = self.wait.get(name).map(|x| *x);
            match is_wait
                .map(|(key, param)| {
                    let (op, p1, p2) = self.forms.get(key).unwrap();
                    self.forms.insert(
                        key,
                        if param == 1 {
                            (*op, Some(val), *p2)
                        } else {
                            (*op, *p1, Some(val))
                        },
                    );
                    self.eval(key).map(|val| (key, val))
                })
                .flatten()
            {
                None => break,
                Some(nv) => {
                    name = nv.0;
                    val = nv.1;
                }
            }
        }
    }

    fn process(&mut self, line: &'a str) -> Option<i64> {
        let name = line.get(0..4).unwrap();
        if line.chars().nth(6).map(|c| c.is_ascii_digit()) == Some(true) {
            let val = i64::from_str_radix(line.get(6..).unwrap(), 10).unwrap();
            self.chain(&name, val);
        } else {
            let p = line.split(' ').collect_vec();
            let a = self.ready.get(&p[1]).map(|x| *x);
            let b = self.ready.get(&p[3]).map(|x| *x);
            assert!(!p[1].eq(p[3]));
            let form = match p[2] {
                "+" => (Formula::Add, a, b),
                "-" => (Formula::Sub, a, b),
                "/" => (Formula::Div, a, b),
                "*" => (Formula::Mul, a, b),
                _ => unimplemented!("What?"),
            };
            self.forms.insert(name, form);
            match form {
                (_, None, None) => {
                    self.wait.insert(p[1], (name, 1));
                    self.wait.insert(p[3], (name, 2));
                }
                (_, None, _) => {
                    self.wait.insert(p[1], (name, 1));
                }
                (_, _, None) => {
                    self.wait.insert(p[3], (name, 2));
                }
                _ => {
                    self.wait.insert(p[1], (name, 1));
                    self.wait.insert(p[3], (name, 2));
                    self.eval(name).map(|val| self.chain(&name, val));
                }
            };
        }
        self.ready.get("root").map(|v| *v)
    }

    fn revert(&self) -> Option<i64> {
        let mut calc = Vec::new();
        let mut key = "humn";
        loop {
            match self
                .wait
                .get(key)
                .map(|(n, p)| {
                    println!("Get {key} {n} {p}");
                    let f = self.forms.get(n).unwrap();
                    if n.eq(&"root") {
                        println!("Found root {p}");
                        if *p == 1 {
                            f.2
                        } else {
                            f.1
                        }
                    } else {
                        println!("Depends on {n} {p}");
                        if *p == 1 {
                            calc.push((f.0, None, f.2));
                        } else {
                            calc.push((f.0, f.1, None))
                        }

                        key = n;
                        None
                    }
                })
                .flatten()
            {
                None => continue,
                Some(n) => {
                    let val = calc.iter().rev().fold(Some(n), |prev, exp| {
                        let prev = prev.unwrap();
                        println!("{:?} {}", exp, prev);
                        match exp {
                            (Formula::Add, None, Some(a)) => Some(prev - a),
                            (Formula::Add, Some(a), None) => Some(prev - a),
                            (Formula::Mul, None, Some(a)) => Some(prev / a),
                            (Formula::Mul, Some(a), None) => Some(prev / a),
                            (Formula::Sub, None, Some(a)) => Some(a + prev),
                            (Formula::Sub, Some(a), None) => Some(a - prev),
                            (Formula::Div, None, Some(a)) => Some(a * prev),
                            (Formula::Div, Some(a), None) => Some(a / prev),

                            _ => unreachable!("No way!"),
                        }
                    });
                    println!("Result: {:?}", val);
                    return val;
                }
            }
        }
    }
}

fn solution_a(input: &str) -> Option<i64> {
    let mut mm = MonkeyMath::new();

    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .for_each(|line| {
            mm.process(line);
        });

    mm.ready.get(&"root").map(|v| *v)
}

fn solution_b(input: &str) -> Option<i64> {
    let mut mm = MonkeyMath::new();

    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .for_each(|line| {
            mm.process(line);
        });

    let r = mm.forms.get("root");
    println!("{:?}", r);
    mm.revert()
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
        assert_eq!(solution_a(&data), Some(152));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(301));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(10037517593724));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(3272260914328));
    }
}
