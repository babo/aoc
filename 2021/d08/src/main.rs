use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let data: Vec<&str> = input
        .lines()
        .map(|line| {
            line.split('|')
                .map(|x| x.split_whitespace())
                .last()
                .unwrap()
        })
        .flatten()
        .collect();

    let count = data.iter().fold(0usize, |acc, x| {
        let unique = match x.trim().len() {
            2 => 1,
            4 => 1,
            3 => 1,
            7 => 1,
            _ => 0,
        };
        if unique == 1 {
            acc + 1
        } else {
            acc
        }
    });
    Some(count)
}

fn deduct(input: &str) -> Vec<String> {
    let numbers: Vec<String> = input
        .trim()
        .split(' ')
        .map(|w| {
            let mut word: Vec<char> = w.clone().chars().collect();
            word.sort();
            String::from_iter(word)
        })
        .collect();

    let mut unknown: Vec<usize> = Vec::new();
    let mut solution = [0usize; 10];
    for x in numbers.iter().enumerate() {
        match x.1.len() {
            2 => solution[1] = x.0,
            3 => solution[7] = x.0,
            4 => solution[4] = x.0,
            7 => solution[8] = x.0,
            _ => unknown.push(x.0),
        }
    }

    // find 9, first join 4 and 7
    let mut n47 = numbers[solution[4]].clone();
    for c in numbers[solution[7]].chars() {
        if !n47.contains(c) {
            n47.push(c);
            n47 = n47.chars().sorted().collect::<String>();
            break;
        }
    }
    let n9 = unknown
        .iter()
        .enumerate()
        .find(|x| {
            numbers
                .get(*x.1)
                .iter()
                .find(|w| {
                    if w.len() == 6 {
                        if n47.chars().map(|c| w.contains(c)).all(|x| x) {
                            return true;
                        }
                    }
                    false
                })
                .map_or(false, |_| true)
        })
        .unwrap();
    let n9pos = n9.0;
    solution[9] = *n9.1;
    unknown.remove(n9pos);

    let seg2 = "abcdefg"
        .chars()
        .find(|c| numbers.get(solution[9]).map(|w| !w.contains(*c)) == Some(true))
        .unwrap();
    // find 2
    let n2 = unknown
        .iter()
        .enumerate()
        .find(|x| {
            numbers
                .get(*x.1)
                .iter()
                .find(|w| {
                    if w.len() == 5 {
                        w.contains(seg2)
                    } else {
                        false
                    }
                })
                .map_or(false, |_| true)
        })
        .unwrap();
    let n2pos = n2.0;
    solution[2] = *n2.1;
    unknown.remove(n2pos);

    // find 3
    let n1 = numbers[solution[1]].clone();
    let n3 = unknown
        .iter()
        .enumerate()
        .find(|x| {
            numbers
                .get(*x.1)
                .iter()
                .find(|w| {
                    if w.len() == 5 {
                        if n1.chars().map(|c| w.contains(c)).all(|x| x) {
                            return true;
                        }
                    }
                    false
                })
                .map_or(false, |_| true)
        })
        .unwrap();
    let n3pos = n3.0;
    solution[3] = *n3.1;
    unknown.remove(n3pos);

    // find 5
    let n5 = unknown
        .iter()
        .enumerate()
        .find(|x| {
            numbers
                .get(*x.1)
                .iter()
                .find(|w| w.len() == 5)
                .map_or(false, |_| true)
        })
        .unwrap();
    let n5pos = n5.0;
    solution[5] = *n5.1;
    unknown.remove(n5pos);

    // find 0
    let n0 = unknown
        .iter()
        .enumerate()
        .find(|x| {
            numbers
                .get(*x.1)
                .iter()
                .find(|w| {
                    if w.len() == 6 {
                        if n1.chars().map(|c| w.contains(c)).all(|x| x) {
                            return true;
                        }
                    }
                    false
                })
                .map_or(false, |_| true)
        })
        .unwrap();
    let n0pos = n0.0;
    solution[0] = *n0.1;
    unknown.remove(n0pos);

    solution[6] = *unknown.first().unwrap();
    solution
        .iter()
        .map(|x| numbers[*x].clone())
        .collect::<Vec<String>>()
}

fn decode(input: &str, code: &Vec<String>) -> usize {
    input
        .trim()
        .split(' ')
        .map(|w| {
            let word: String = w.clone().chars().sorted().collect();
            code.iter()
                .enumerate()
                .find(|x| word.eq(x.1))
                .map(|x| x.0)
                .unwrap()
        })
        .fold(0, |acc, x| acc * 10 + x)
}

fn solution_b(input: &str) -> Option<usize> {
    let data: Vec<usize> = input
        .lines()
        .map(|line| {
            let lr: Vec<&str> = line.split("|").collect();
            let code = deduct(lr[0]);
            decode(lr[1], &code)
        })
        .collect();

    Some(data.iter().sum())
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
        assert_eq!(solution_a(&data), Some(26));
    }

    #[test]
    fn test_line() {
        let line =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(solution_b(&line), Some(5353));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(61229));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(412));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(978171));
    }
}
