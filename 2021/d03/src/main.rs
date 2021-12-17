use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn core_a(input: &str) -> (usize, usize) {
    let mut count: [i64; 16] = [0; 16];
    let n = input.lines().nth(0).map(|x| x.trim().len()).unwrap();
    input.lines().for_each(|line| {
        let x = u64::from_str_radix(line.trim(), 2).unwrap();
        for i in 0..n {
            count[i] += if (x & (1 << i)) != 0 { 1 } else { -1 };
        }
    });
    let gamma = count
        .iter()
        .take(n)
        .fold((0usize, 0u8), |acc, x| {
            (
                if *x >= 0 {
                    acc.0 + (1usize << acc.1)
                } else {
                    acc.0
                },
                acc.1 + 1,
            )
        })
        .0;
    let mask = (1usize << n) - 1;
    let epsilon = gamma ^ mask;

    println!("{:016b} {:016b} {:016b}", gamma, epsilon, mask);
    (gamma, epsilon)
}
fn solution_a(input: &str) -> Option<usize> {
    let (gamma, epsilon) = core_a(input);
    Some(gamma * epsilon)
}

fn count_bits(input: &Vec<usize>, n: usize, most: bool) -> Option<usize> {
    let mut mask = 0usize;
    let mut bits = 0usize;

    for ii in 1..=n {
        let bit = 1usize << (n - ii);

        let count = input.iter().fold(0i64, |acc, x| {
            if (x & bits) ^ mask == 0 {
                if x & bit != 0 {
                    acc + 1
                } else {
                    acc - 1
                }
            } else {
                acc
            }
        });
        bits |= bit;
        if (most && count >= 0) || (!most && count < 0) {
            mask |= bit;
        }
        if ii == 1 {
            continue;
        }

        let f = |x: &usize| (x & bits) ^ mask == 0;
        let first = input.iter().position(f);
        if first.is_none() {
            break;
        }
        let first = first.unwrap();
        let second = input.iter().skip(first + 1).position(f);
        if second.is_none() {
            let val = input.iter().nth(first).unwrap();
            return Some(*val);
        }
    }
    None
}

fn solution_b(input: &str) -> Option<usize> {
    let nn = input.lines().nth(0).map(|x| x.trim().len()).unwrap();
    let mask = (1u16 << (nn + 1)) - 1;
    let nums: Vec<usize> = input
        .lines()
        .map(|line| (u16::from_str_radix(line.trim(), 2).unwrap() & mask) as usize)
        .collect();
    nums.iter().for_each(|x| println!("{:02} {:016b}", x, x));

    count_bits(&nums, nn, true)
        .and_then(|oxygen| count_bits(&nums, nn, false).and_then(|co2| Some(oxygen * co2)))
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
    fn test_simple_a() {
        let data = "00100
                    11110
                    10110
                    10111
                    10101
                    01111
                    00111
                    11100
                    10000
                    11001
                    00010
                    01010";
        assert_eq!(solution_a(&data), Some(198));
    }

    #[test]
    fn test_simple_b() {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        assert_eq!(solution_b(&input), Some(230));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(738234));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(3969126));
    }
}
