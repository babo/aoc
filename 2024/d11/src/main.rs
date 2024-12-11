use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

// Rules:
// * If the stone is engraved with the number `0`, it is replaced by a stone engraved with the number `1`.
// * If the stone is engraved with a number that has an *even* number of digits, it is replaced by *two stones*.
//      The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
//      (The new numbers don't keep extra leading zeroes: `1000` would become stones `10` and `0`.)
// * If none of the other rules apply, the stone is replaced by a new stone; the old stone's number *multiplied by 2024* is engraved on the new stone.

fn rules(x: usize) -> Vec<usize> {
    if x == 0 {
        return vec![1];
    }

    let nd = x.ilog10() + 1;
    if nd % 2 == 0 {
        let d = 10_usize.pow(nd / 2);
        vec![x / d, x % d]
    } else {
        vec![x * 2024]
    }
}

fn solution(input: &str, blinks: u8) -> usize {
    let input = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let mut memo: HashMap<usize, Vec<usize>> = HashMap::new();
    let take_5 = |x: usize| {
        (0..5).fold(vec![x], |acc, _| {
            acc.iter().flat_map(|x| rules(*x)).collect_vec()
        })
    };

    let mut batch = input.clone();
    while !batch.is_empty() {
        let mut unique: HashSet<usize> = HashSet::new();
        batch.iter().for_each(|x| {
            if !memo.contains_key(x) {
                let v = take_5(*x);
                unique.extend(v.iter().filter(|x| !memo.contains_key(x)));
                memo.insert(*x, v);
            }
        });
        batch = unique.into_iter().collect_vec();
    }
    println!("Memo size: {}", memo.len());
    let size_5: HashMap<usize, usize> = memo.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.insert(*k, v.len());
        acc
    });
    let deep = (1..blinks / 5).fold(size_5, |prev, _| {
        memo.iter().fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(*k, v.iter().map(|x| prev[x]).sum());
            acc
        })
    });
    input.iter().map(|x| deep[x]).sum()
}

fn main() {
    let c = content().unwrap();

    let a = solution(&c, 25);
    let b = solution(&c, 75);

    println!("Step A: {}", a);
    println!("Step B: {}", b);
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
        assert_eq!(solution(&data, 25), 55312);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution(&c, 25), 175006);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution(&c, 75), 207961583799296);
    }

    #[test]
    fn test_rules() {
        assert_eq!(rules(0), vec![1]);
        assert_eq!(rules(1), vec![2024]);
        assert_eq!(rules(10), vec![1, 0]);
        assert_eq!(rules(100), vec![202400]);
        assert_eq!(rules(1000), vec![10, 0]);
    }
}
