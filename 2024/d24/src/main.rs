use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::write;

// https://www.101computing.net/binary-additions-using-logic-gates/

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

type GateFn = fn(bool, bool) -> bool;

fn op(gate: &Gate) -> GateFn {
    match gate {
        Gate::And => |a, b| a && b,
        Gate::Or => |a, b| a || b,
        Gate::Xor => |a, b| a ^ b,
    }
}

type GateVal = (
    Gate,
    String,
    String,
    String,
    Option<bool>,
    Option<bool>,
    Option<bool>,
);

struct Device {
    input: String,
    ops: Vec<GateVal>,
    vals: HashMap<String, bool>,
}

impl Device {
    fn new(input: &str) -> Self {
        let vals: HashMap<String, bool> = input.trim().lines().take_while(|x| !x.is_empty()).fold(
            HashMap::new(),
            |mut acc, line| {
                let kv = line.trim().split(": ").collect_vec();
                acc.insert(kv[0].to_string(), kv[1] == "1");
                acc
            },
        );

        let ops: Vec<GateVal> = input
            .trim()
            .lines()
            .skip(vals.len() + 1)
            .map(|line| {
                let parts = line.split_ascii_whitespace().collect_vec();

                let gate = match parts[1] {
                    "AND" => Gate::And,
                    "OR" => Gate::Or,
                    "XOR" => Gate::Xor,
                    _ => unimplemented!(),
                };

                let val: GateVal = (
                    gate,
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                    vals.get(parts[0]).map(|x| *x),
                    vals.get(parts[2]).map(|x| *x),
                    None,
                );
                val
            })
            .collect_vec();
        Self {
            input: input.to_string(),
            ops,
            vals,
        }
    }

    fn nb(&self) -> usize {
        self.vals.len() / 2
    }

    fn eval(&self, xy: Option<(usize, usize)>) -> usize {
        let mut vals = self.vals.clone();
        let mut ops = self.ops.clone();

        if let Some(xy) = xy {
            vals.clear();
            (0..self.nb()).for_each(|i| {
                let m = 1 << i;
                vals.insert(format!("x{:02}", i), (xy.0 & m) != 0);
                vals.insert(format!("y{:02}", i), (xy.1 & m) != 0);
            });
        }

        while ops.iter().any(|x| x.6.is_none()) {
            ops.iter_mut().for_each(|v| {
                let c = vals
                    .get(v.1.as_str())
                    .map(|a| vals.get(v.2.as_str()).map(|b| op(&v.0)(*a, *b)))
                    .flatten();
                if let Some(c) = c {
                    vals.insert(v.3.clone(), c);
                    v.6 = Some(c);
                }
            });
        }
        vals.keys()
            .filter(|x| x.starts_with("z"))
            .sorted()
            .fold(0, |acc, k| {
                if *vals.get(k).unwrap() {
                    let s = k
                        .chars()
                        .skip(1)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    acc | (1 << s)
                } else {
                    acc
                }
            })
    }
}

fn solution_a(input: &str) -> usize {
    Device::new(input).eval(None)
}

fn solution_b(input: &str) -> String {
    let mut mixed = Vec::new();
    let mut device = Device::new(input);
    let failed = (0..device.nb())
        .filter(|i| {
            let val = 1 << i;
            device.eval(Some((val, 0))) != val || device.eval(Some((0, val))) != val
        })
        .collect_vec();
    println!("Failed: {:?}", failed);

    let mut carry = None;
    let bits = device.nb();
    for i in 0..bits {
        let (kx, ky, kz) = (
            format!("x{:02}", i),
            format!("y{:02}", i),
            format!("z{:02}", i),
        );
        if i == 0 {
            let xor = device.ops.iter().find(|x| {
                x.0 == Gate::Xor
                    && x.3 == kz
                    && ((x.1 == ky && x.2 == kx) || (x.1 == kx && x.2 == ky))
            });
            let and = device.ops.iter().find(|x| {
                x.0 == Gate::And && ((x.1 == ky && x.2 == kx) || (x.1 == kx && x.2 == ky))
            });
            assert!(xor.is_some());
            assert!(and.is_some());
            carry = Some(and.unwrap().3.clone());
        } else {
            println!("{}", kz);
            let ha1_xor = device.ops.iter().find(|x| {
                x.0 == Gate::Xor && ((x.1 == ky && x.2 == kx) || (x.1 == kx && x.2 == ky))
            });
            let ha1_and = device.ops.iter().find(|x| {
                x.0 == Gate::And && ((x.1 == ky && x.2 == kx) || (x.1 == kx && x.2 == ky))
            });
            assert!(ha1_xor.is_some());
            assert!(ha1_and.is_some());
            let ha1_sum = ha1_xor.unwrap().3.clone();
            let ha1_carry = ha1_and.unwrap().3.clone();
            let cin = carry.clone().unwrap();
            let ha2_xor = device.ops.iter().find(|x| {
                x.0 == Gate::Xor
                    && x.3 == kz
                    && ((x.1 == ha1_sum && x.2 == cin) || (x.1 == cin && x.2 == ha1_sum))
            });
            if ha2_xor.is_none() {
                let mixed_xor = device.ops.iter().find(|x| {
                    x.0 == Gate::Xor
                        && ((x.1 == ha1_sum && x.2 == cin) || (x.1 == cin && x.2 == ha1_sum))
                });
                println!("XOR {} {}", ha1_sum, cin);
                if mixed_xor.is_none() {
                    let mixed_xor = device
                        .ops
                        .iter()
                        .find(|x| x.0 == Gate::Xor && x.3 == kz && (x.1 == cin || x.2 == cin));
                    let name = if mixed_xor.unwrap().1 == cin {
                        mixed_xor.unwrap().2.clone()
                    } else {
                        mixed_xor.unwrap().1.clone()
                    };
                    assert!(mixed_xor.is_some());
                    println!("Mixed: {} {}", ha1_sum, name);
                    let modified = device
                        .input
                        .replace(format!("-> {}", ha1_sum.as_str()).as_str(), "-> 777")
                        .replace(format!("-> {}", name.as_str()).as_str(), "-> 444")
                        .replace("444", &ha1_sum)
                        .replace("777", &name);
                    write("modified", modified).unwrap();
                    return "Failed".to_string();
                }
                let name = mixed_xor.unwrap().3.clone();
                mixed.push(name.clone());
                mixed.push(kz.clone());
                println!("Mixed: {} {}", kz, name);
                let modified = device
                    .input
                    .replace(format!("-> {}", kz.as_str()).as_str(), "-> 777")
                    .replace(format!("-> {}", name.as_str()).as_str(), "-> 444")
                    .replace("444", &kz)
                    .replace("777", &name);
                write("modified", modified).unwrap();
                return "Failed".to_string();
            }
            let ha2_and = device.ops.iter().find(|x| {
                x.0 == Gate::And
                    && ((x.1 == ha1_sum && x.2 == cin) || (x.1 == cin && x.2 == ha1_sum))
            });
            assert!(ha2_xor.is_some());
            assert!(ha2_and.is_some());
            let ha2_carry = ha2_and.unwrap().3.clone();
            println!("{} {}", ha1_carry, ha2_carry);
            let ha2_or = device.ops.iter().find(|x| {
                x.0 == Gate::Or
                    && ((x.1 == ha1_carry && x.2 == ha2_carry)
                        || (x.1 == ha2_carry && x.2 == ha1_carry))
            });
            assert!(ha2_or.is_some());
            carry = Some(ha2_or.unwrap().3.clone());
        }
    }

    mixed.iter().sorted().join(",")
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
        assert_eq!(solution_a(&data), 2024);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), "z00,z01,z02,z05");
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 48806532300520);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), "ddn,kqh,nhs,nnf,wrc,z09,z20,z34");
    }
}
