use std::fs::read_to_string;
use std::iter::FromIterator;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<u32> {
    Some(count_versions(input))
}

fn solution_b(input: &str) -> Option<u64> {
    let bits = to_bits(&input);
    Some(evaluate(&bits, 0).2)
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
}

fn to_bits(input: &str) -> Vec<char> {
    let rtv: Vec<char> = input
        .trim()
        .chars()
        .map(|x| {
            let v = x.to_digit(16).unwrap() as u8;
            format!("{:04b}", v).chars().collect::<Vec<char>>()
        })
        .flatten()
        .collect();

    rtv
}

fn to_num(iter: &[char]) -> u32 {
    u32::from_str_radix(&String::from_iter(iter), 2)
        .ok()
        .unwrap()
}

fn evaluate(bits: &Vec<char>, from: usize) -> (u32, usize, u64) {
    let mut version = to_num(&bits[from..from + 3]);
    let kind = to_num(&bits[from + 3..from + 6]);
    if kind == 4 {
        let mut last = from + 6;
        let mut l = 0u64;
        loop {
            let x = to_num(&bits[last..last + 5]) as u64;
            l = (l << 4) | (x & 0x0f);
            last += 5;
            if x & 0x10 == 0 {
                break;
            }
        }
        return (version, last, l);
    }

    let mut last;
    let mut results: Vec<u64> = Vec::new();
    let length_type = to_num(&bits[from + 6..from + 7]);
    if length_type == 0 {
        last = from + 22;
        let len = to_num(&bits[from + 8..from + 22]) as usize;
        while last < from + 22 + len {
            let result = evaluate(bits, last);
            version += result.0;
            last = result.1;
            results.push(result.2);
        }
    } else {
        last = from + 18;
        let len = to_num(&bits[from + 8..from + 18]) as usize;
        for _ in 0..len {
            let result = evaluate(bits, last);
            version += result.0;
            last = result.1;
            results.push(result.2);
        }
    }

    let result = match kind {
        0 => results.iter().sum(),
        1 => results.iter().product(),
        2 => *results.iter().min().unwrap(),
        3 => *results.iter().max().unwrap(),
        5 => {
            if results.get(0).unwrap() > results.get(1).unwrap() {
                1
            } else {
                0
            }
        }

        6 => {
            if results.get(0).unwrap() < results.get(1).unwrap() {
                1
            } else {
                0
            }
        }
        7 => {
            if results.get(0).unwrap() == results.get(1).unwrap() {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknown operator kind: {}", kind)
    };
    println!("operator {}", kind);
    println!("{:?}", results);
    println!("=> {}", result);

    (version, last, result)
}

fn count_versions(data: &str) -> u32 {
    let bits = to_bits(&data);
    return evaluate(&bits, 0).0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_operator() {
        let data = "EE00D40C823060";
        let v = count_versions(data);
        assert_eq!(v, 14);

        let data = "38006F45291200";
        let v = count_versions(data);
        assert_eq!(v, 9);
    }

    #[test]
    fn test_simple_a() {
        let data = "8A004A801A8002F478";
        assert_eq!(solution_a(&data), Some(16));

        let data = "620080001611562C8802118E34";
        assert_eq!(solution_a(&data), Some(12));

        let data = "C0015000016115A2E0802F182340";
        assert_eq!(solution_a(&data), Some(23));

        let data = "A0016C880162017C3686B18A3D4780";
        assert_eq!(solution_a(&data), Some(31));
    }

    #[test]
    fn test_simple_b() {
        let data = "C200B40A82";
        assert_eq!(solution_b(&data), Some(3));

        let data = "04005AC33890";
        assert_eq!(solution_b(&data), Some(54));

        let data = "880086C3E88112";
        assert_eq!(solution_b(&data), Some(7));

        let data = "CE00C43D881120";
        assert_eq!(solution_b(&data), Some(9));

        let data = "D8005AC2A8F0";
        assert_eq!(solution_b(&data), Some(1));

        let data = "F600BC2D8F";
        assert_eq!(solution_b(&data), Some(0));

        let data = "9C005AC2F8F0";
        assert_eq!(solution_b(&data), Some(0));

        let data = "9C0141080250320F1802104A08";
        assert_eq!(solution_b(&data), Some(1));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(953));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(246225449979));
    }
}
