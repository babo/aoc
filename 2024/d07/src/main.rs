use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn find_ops_a(tv: usize, nums: &Vec<usize>) -> usize {
    let all = nums.iter().fold(vec![], |acc, x| {
        if acc.len() == 0 {
            vec![*x]
        } else {
            let mut res = vec![];
            for a in acc.iter() {
                if a <= &tv {
                    res.push(a + x);
                    res.push(a * x);
                }
            }
            res
        }
    });
    all.iter().find(|x| **x == tv).map(|x| *x).unwrap_or(0)
}

fn find_ops_b(tv: usize, nums: &Vec<usize>) -> usize {
    let all = nums.iter().fold(vec![], |acc, x| {
        if acc.len() == 0 {
            vec![*x]
        } else {
            let mut res = vec![];
            for a in acc.iter() {
                if a <= &tv {
                    res.push(a + x);
                    res.push(a * x);
                    res.push(a * 10usize.pow(x.ilog10() + 1) + x);
                }
            }
            res
        }
    });
    all.iter().find(|x| **x == tv).map(|x| *x).unwrap_or(0)
}

fn solve_a(line: &str) -> usize {
    let parts: Option<(usize, Vec<usize>)> =
        line.trim().split(":").collect_tuple().map(|(res, parts)| {
            (
                res.trim().parse::<usize>().unwrap(),
                parts
                    .trim()
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        });
    parts.map(|(tv, nums)| find_ops_a(tv, &nums)).unwrap_or(0)
}

fn solve_b(line: &str) -> usize {
    let parts: Option<(usize, Vec<usize>)> =
        line.trim().split(":").collect_tuple().map(|(res, parts)| {
            (
                res.trim().parse::<usize>().unwrap(),
                parts
                    .trim()
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        });
    parts.map(|(tv, nums)| find_ops_b(tv, &nums)).unwrap_or(0)
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<usize>())
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
        assert_eq!(solution_a(&data), Some(3749));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(11387));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1298103531759));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(140575048428831));
    }
}
