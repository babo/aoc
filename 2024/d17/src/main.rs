use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn sub_1(a: u64) -> u64 {
    let b = (a % 8) ^ 7;
    let c = a / 2u64.pow(b as u32);
    ((b ^ c) ^ 7) % 8
}

fn sub_2(a: u64) -> u64 {
    let b = (a % 8) ^ 7;
    let c = a / 2u64.pow(b as u32);
    ((b ^ c) ^ 7) % 8
}

fn part_1() -> HashMap<u8, u64> {
    let mut rtv = HashMap::new();

    for res in 0..8 {
        for a in 0u64.. {
            if sub_1(a) == res {
                rtv.insert(res as u8, a);
                break;
            }
        }
    }

    rtv
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Instruction {
    Adv = 0,
    Bxl = 1,
    Bst,
    Jzn,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jzn),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => Err(()),
        }
    }
}

struct Handheld {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
}

impl Handheld {
    fn new(input: &str) -> Self {
        let abc: (u64, u64, u64) = input
            .lines()
            .filter(|x| x.contains("Register"))
            .map(|x| x.split_whitespace().last().unwrap().parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let program = input
            .lines()
            .filter(|x| x.contains("Program"))
            .flat_map(|x| {
                let instr: Vec<u8> = x
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .split(",")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();
                instr
            })
            .collect_vec();

        Handheld {
            reg_a: abc.0,
            reg_b: abc.1,
            reg_c: abc.2,
            program,
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut rtv = vec![];
        let mut ip = 0usize;
        let mut reg_a = self.reg_a;
        let mut reg_b = self.reg_b;
        let mut reg_c = self.reg_c;

        println!("Program: {:?}", self.program);
        println!("Initial: A: {} B: {} C: {}", reg_a, reg_b, reg_c);

        while ip < self.program.len() {
            let opcode = self.program[ip];
            let operand = self.program[ip + 1];
            let combo = match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => (operand % 8) as u64, // panic!("Invalid combo operand")
            };
            println!("      Opcode: {} Operand: {}", opcode, operand);

            let combo_str = match operand {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "reg_a",
                5 => "reg_b",
                6 => "reg_c",
                _ => "illegal", // panic!("Invalid combo operand")
            };

            ip += 2;
            match Instruction::try_from(opcode) {
                Ok(Instruction::Adv) => {
                    println!(
                        "Adv: store reg_a / 2pow({}) in reg_a {} / {} = {}",
                        combo_str,
                        reg_a,
                        2u64.pow(combo as u32),
                        reg_a / 2u64.pow(combo as u32)
                    );
                    reg_a /= 2u64.pow(combo as u32)
                }
                Ok(Instruction::Bxl) => {
                    println!(
                        "Bxl:reg_b ^= {} which is {} ^ {} => {}",
                        operand,
                        reg_b,
                        operand,
                        reg_b ^ operand as u64
                    );
                    reg_b ^= operand as u64;
                }
                Ok(Instruction::Bst) => {
                    println!(
                        "Bst: store {} % 8 in reg b which is {} % 8 = {}",
                        combo_str,
                        combo,
                        combo % 8
                    );
                    reg_b = combo % 8;
                }
                Ok(Instruction::Jzn) => {
                    println!("Jzn: {} {} skip to {}", reg_a, operand, operand);
                    if reg_a != 0 {
                        ip = operand as usize;
                    }
                }
                Ok(Instruction::Bxc) => {
                    println!(
                        "Bxc: reg_b xor reg_c stored in reg_b {} ^ {} = {}",
                        reg_b,
                        reg_c,
                        reg_b ^ reg_c
                    );
                    reg_b ^= reg_c;
                }
                Ok(Instruction::Out) => {
                    println!("Out: {} % 8 {} % 8 = {}", combo_str, combo, combo % 8);
                    rtv.push(combo % 8);
                }
                Ok(Instruction::Bdv) => {
                    println!(
                        "Bdv: store reg_a ({}) / 2pow({}) in reg_b {}",
                        reg_a,
                        combo_str,
                        reg_a / 2u64.pow(combo as u32)
                    );
                    reg_b = reg_a / 2u64.pow(combo as u32);
                }
                Ok(Instruction::Cdv) => {
                    println!(
                        "Cdv: store reg_a / 2pow({}) in reg_c {} / 2pow({}) = {}",
                        combo_str,
                        reg_a,
                        combo,
                        reg_a / 2u64.pow(combo as u32)
                    );
                    reg_c = reg_a / 2u64.pow(combo as u32);
                }
                Err(_) => {
                    panic!("Invalid instruction");
                }
            }
        }
        self.reg_a = reg_a;
        self.reg_b = reg_b;
        self.reg_c = reg_c;

        println!("Final: A: {} B: {} C: {}\n", reg_a, reg_b, reg_c);
        rtv
    }
}

fn solution_a(input: &str) -> String {
    let mut hh = Handheld::new(input);
    let out = hh.run();
    out.iter().map(|x| x.to_string()).join(",")
}

fn solve_b(program: Vec<u8>) -> Option<u64> {
    let nums = program.iter().enumerate().map(|(i, x)| (i as u32, *x)).collect_vec();
    for a in 0u64.. {
        if nums.iter().all(|(i, x)| {
            *x as u64 == sub_2(a / 8u64.pow(*i))
        }) {
            return Some(a);
        }
    }
    None
}

fn solve_c(program: Vec<u8>) -> Option<u64> {
    let mut cache = HashMap::new();
    for i in 0u8..8 {
        let mut a10 = vec![];

        for a in 0.. {
            if sub_1(a) == i as u64 {
                a10.push(a);
                if a10.len() == 10 {
                    break;
                }
            }
        }
        cache.insert(i, a10);
    }
    println!("{:?}", cache);

    let nums = program.iter().enumerate().map(|(i, x)| (i as u32, *x)).collect_vec();
    let n = program.len();
    println!("Length: {}", n);

    Some(program.iter().fold(0, |acc, x| {
        let a = cache.get(x).unwrap()[1];
        acc * 8 + a
    }))
}

fn solution_b(input: &str) -> Option<usize> {
    let mut hh = Handheld::new(input);
    let data = hh.program.clone().iter().map(|x| *x).take(10).collect::<Vec<_>>();

    if let Some(a) = solve_c(data) {
        hh.reg_a = a as u64;
        let out = hh.run();
        println!("Found a: {} {}", a, u64::MAX);
        println!("Program: {:?}", hh.program);
        println!("Output: {:?}", out);
        if out.len() == hh.program.len()
            && out
                .iter()
                .zip(hh.program.iter())
                .all(|(a, b)| *a == *b as u64)
        {
            return Some(a as usize);
        }
    }

    None
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
    fn test_mini_a1() {
        let data = "Register A: 0\nRegister B: 0\nRegister C: 9\nProgram: 2,6";
        let mut hh = Handheld::new(data);
        hh.run();

        assert_eq!(hh.reg_b, 1);
    }

    #[test]
    fn test_mini_a2() {
        let data = "Register A: 10\nRegister B: 0\nRegister C: 9\nProgram: 5,0,5,1,5,4";
        let mut hh = Handheld::new(data);
        let out = hh.run();

        assert_eq!(out, [0, 1, 2]);
    }

    #[test]
    fn test_mini_a3() {
        let data = "Register A: 2024\nRegister B: 0\nRegister C: 9\nProgram: 0,1,5,4,3,0";
        let mut hh = Handheld::new(data);
        let out = hh.run();

        assert_eq!(hh.reg_a, 0);
        assert_eq!(out, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_mini_a4() {
        let data = "Register A: 2024\nRegister B: 29\nRegister C: 9\nProgram: 1,7";
        let mut hh = Handheld::new(data);
        hh.run();

        assert_eq!(hh.reg_b, 26);
    }

    #[test]
    fn test_mini_a5() {
        let data = "Register A: 0\nRegister B: 2024\nRegister C: 43690\nProgram: 4,0";
        let mut hh = Handheld::new(data);
        hh.run();

        assert_eq!(hh.reg_b, 44354);
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_simple_b() {
        let data = "Register A: 2024
                    Register B: 0
                    Register C: 0

                    Program: 0,3,5,4,3,0";
        assert_eq!(solution_b(&data), Some(117440));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let out = solution_a(&c);
        assert_eq!(out, "2,1,0,1,7,2,5,0,3");
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
