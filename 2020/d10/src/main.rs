use std::fs::read_to_string;
use std::io::{self};

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn combinations(n: &[u32]) -> u128 {
    let m = n.len();

    if m > 3 {
        let cont = m > 4;
        if n[3] == n[0] + 3 {
            if cont {
                return combinations(&n[1..]) + combinations(&n[2..]) + combinations(&n[3..]);
            }
            return 4;
        }
    }

    if m > 2 {
        let cont = m > 3;
        if n[2] == n[0] + 2 {
            if cont {
                return combinations(&n[1..]) + combinations(&n[2..]);
            }
            return 2;
        }
        if n[1] == n[0] + 3 {
            if cont {
                return combinations(&n[1..]);
            }
            return 1;
        }
        return combinations(&n[2..]);
    }

    1
}

fn solution_a() -> u32 {
    let mut all: Vec<u32> = content()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    all.sort();
    all.push(all.last().unwrap() + 3);
    let rtv = all
        .iter()
        .fold((0u32, 0u32, 0u32), |acc, x| -> (u32, u32, u32) {
            match x - acc.0 {
                1 => (*x, acc.1 + 1, acc.2),
                3 => (*x, acc.1, acc.2 + 1),
                _ => panic!("What a difference: {}", x - acc.0),
            }
        });
    rtv.1 * rtv.2
}

fn solution_b() -> u128 {
    let mut all: Vec<u32> = content()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    all.sort();
    all.push(all.last().unwrap() + 3);
    combinations(&all)
}

fn main() {
    let a = solution_a();
    let b = solution_b();

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_file_reading() -> Result<(), io::Error> {
        assert_ne!(content()?.len(), 0);
        Ok(())
    }

    #[test]
    fn test_conversion() -> Result<(), io::Error> {
        let c = content().unwrap();
        let a = c
            .split_whitespace()
            .map(|x| x.parse::<u32>().ok())
            .filter(|x| x.is_none());
        assert_eq!(a.count(), 0);
        Ok(())
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(solution_a(), 1856);
    }

    #[test]
    fn test_combinations_a1() {
        let a = vec![0, 1, 2, 3];

        assert_eq!(combinations(&a), 4);
    }
    #[test]
    fn test_combinations_a2() {
        let b = vec![0, 1, 4, 5];

        assert_eq!(combinations(&b), 1);
    }
    #[test]
    fn test_combinations_a3() {
        let c = vec![0, 3, 4, 5];

        assert_eq!(combinations(&c), 2);
    }
    #[test]
    fn test_combinations_a4() {
        let d = vec![0, 3, 6];

        assert_eq!(combinations(&d), 1);
    }
    #[test]
    fn test_combinations_a5() {
        let a = vec![0, 1, 2, 5];

        assert_eq!(combinations(&a), 2);
    }

    #[test]
    fn test_combinations_b() {
        let c = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];

        assert_eq!(combinations(&c), 8);
    }

    #[test]
    fn test_combinations_c() {
        let d = vec![
            0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35,
            38, 39, 42, 45, 46, 47, 48, 49, 52,
        ];

        assert_eq!(combinations(&d), 19208);
    }
}
