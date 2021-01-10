use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn mask(input: & str) -> Option<(u64, u64)> {
    lazy_static! {
        static ref RE_MASK: Regex =
            Regex::new(r"^\s*mask = ([X01]+)\s*$").unwrap();
    }
    let m = RE_MASK.captures(input);
    if m.is_some() {
        let cap = m.unwrap();
        let mask = cap.get(1).unwrap().as_str();
        let m1: String = mask.chars().map(|x| if x == 'X' { '1' } else { x }).collect();
        let m2: String = mask.chars().map(|x| if x == 'X' { '0' } else { x }).collect();
        let mask_and = u64::from_str_radix(&m1, 2).ok().unwrap();
        let mask_or = u64::from_str_radix(&m2, 2).ok().unwrap();
        Some((mask_or, mask_and))
    } else {
        None
    }
}

fn float_mask(input: & str) -> (u64, Vec<u64>) {
    lazy_static! {
        static ref RE_MASK: Regex =
            Regex::new(r"^\s*mask = ([X01]+)\s*$").unwrap();
    }
    let m = RE_MASK.captures(input);
    if m.is_none() {
        panic!("Unknown line: {}", input);
    }
    let mask = m.unwrap().get(1).unwrap().as_str();
    let count = mask.chars().filter(|x| *x == 'X').count();
    let xmask: String = mask.chars().map(|x| if x == 'X' { '0' } else { '1' }).collect();
    let xmask = u64::from_str_radix(&xmask, 2).ok().unwrap();
    let maxi = 2u64.pow(count as u32);
    let mut rtv:Vec<u64> = Vec::new();

    for number in 0 .. maxi {
        let mut cm = String::from(mask);
        for i in 0 .. count {
            let r = if (maxi >> (1 + i)) & number != 0 { "1" } else { "0" };
            cm = cm.replacen('X', r, 1);
        }
        rtv.push(u64::from_str_radix(&cm, 2).ok().unwrap())
    }
    (xmask, rtv)
}

fn mem(input: & str) -> Option<(u64, u64)> {
    lazy_static! {
        static ref RE_MEM: Regex =
            Regex::new(r"^\s*mem\[(\d+)\] = (\d+)\s*$").unwrap();
    }
    let m = RE_MEM.captures(input);
    if m.is_some() {
        let cap = m.unwrap();
        let n1 = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let n2 = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
        Some((n1, n2))
    } else {
        None
    }
}

fn process_a(input: &str) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut mask_and = 0xffffffffffffu64;
    let mut mask_or = 0u64;

    for line in input.lines() {
        match mask(line) {
            Some((mo, ma)) => {
                mask_or = mo;
                mask_and = ma;
            },
            None => {
                match mem(line) {
                    Some((pos, num)) => {
                        let val = (num | mask_or) & mask_and;
                        memory.insert(pos, val);
                    },
                    None => panic!("Unknown line: {}", line)
                }
            }
        }
    }

    memory.values().sum()
}

fn process_b(input: &str) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut masks: Vec<u64> = Vec::new();
    let mut xmask = 1u64 << 36 - 1;

    for line in input.lines() {
        match mem(line) {
            Some((addr, val)) => {
                for m in masks.iter() {
                    memory.insert(addr & xmask | m, val);
                }
            },
            None => {
                let fm = float_mask(line);
                xmask = fm.0;
                masks = fm.1;
            }
        }
    }

    memory.values().sum()
}

fn solution_a(input: &str) -> u64 {
    process_a(input)
}

fn solution_b(input: &str) -> u64 {
    process_b(input)
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
    fn test_content() {
        let input = "mask = X100110110X011000101000101XX11001X11
        mem[5201] = 1838761";

        assert_ne!(process_a(&input), 0);
    }

    #[test]
    fn test_full_content() {
        let input = content().unwrap();
        assert_ne!(process_a(&input), 0);
    }

    #[test]
    fn test_radix() {
        let m = u64::from_str_radix("1011", 2).ok().unwrap();
        assert_eq!(m, 11);
    }

    #[test]
    fn test_mask() {
        let input = "   mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        assert_eq!(mask(&input), Some((
            0b000000000000000000000000000001000000u64,
            0b111111111111111111111111111111111101u64
        )));
    }

    #[test]
    fn test_mem() {
        let input = "   mem[5] = 11";
        assert_eq!(mem(&input), Some((5, 11)));
    }

    #[test]
    fn test_and_or() {
        let input = "   mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                        mem[5] = 11";
        assert_eq!(process_a(&input), 73);
    }

    #[test]
    fn test_no_effect() {
        let input = "   mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                        mem[6] = 101";
        assert_eq!(process_a(&input), 101);
    }

    #[test]
    fn test_or() {
        let input = "   mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                        mem[7] = 0";
        assert_eq!(process_a(&input), 64);
    }

    #[test]
    fn test_sum() {
        let input = "   mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                        mem[5] = 11
                        mem[6] = 101
                        mem[7] = 0";
        assert_eq!(process_a(&input), 73+101+64);
    }

    #[test]
    fn test_overwrite() {
        let input = "   mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                        mem[5] = 11
                        mem[5] = 101
                        mem[5] = 0";
        assert_eq!(process_a(&input), 64);
    }

    #[test]
    fn test_float_1() {
        let input = "   mask = 000000000000000000000000000000X1001X";
        assert_eq!(float_mask(input).1.len(), 4);
    }

    #[test]
    fn test_float_2() {
        let input = "   mask = 000000000000000000000000000000001XX";
        let mut out = float_mask(input);
        out.1.sort();
        assert_eq!(out.1, [4, 5, 6, 7]);
    }

    #[test]
    fn test_b() {
        let input = "   mask = 000000000000000000000000000000X1001X
                        mem[42] = 100
                        mask = 00000000000000000000000000000000X0XX
                        mem[26] = 1";
        assert_eq!(process_b(input), 208);
    }

    #[test]
    fn test_b1() {
        let input = "   mask = 000000000000000000000000000000000XXX
                        mem[7] = 1";
        assert_eq!(process_b(input), 8);
    }

    #[test]
    fn test_b2() {
        let input = "   mask = 00000000000000000000000000000000000X
                        mem[0] = 1";
        assert_eq!(process_b(input), 2);
    }


    #[test]
    fn test_solution_a() {
        let input = content().unwrap();
        assert_eq!(solution_a(&input), 6559449933360u64);
    }

    #[test]
    fn test_solution_b() {
        let input = content().unwrap();
        assert_eq!(solution_b(&input), 3369767240513u64);
    }
}
