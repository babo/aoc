use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn validate(springs: &[char], groups: &[u16]) -> bool {
    let c = springs.iter().fold((true, None, 0), |accu, x| {
        if accu.0 {
            if accu.1.is_none() {
                (true, if *x == '.' { None } else { Some(1) }, accu.2)
            } else if *x == '#' {
                (true, accu.1.map(|i| i + 1), accu.2)
            } else {
                (
                    groups.get(accu.2).map(|x| *x == accu.1.unwrap()) == Some(true),
                    None,
                    accu.2 + 1,
                )
            }
        } else {
            accu
        }
    });
    c.0 && {
        if c.2 + 1 == groups.len() {
            let e = *groups.last().unwrap();
            c.1 == Some(e)
        } else {
            c.2 == groups.len() && c.1.is_none()
        }
    }
}

fn solve_a(line: &str) -> usize {
    let groups: Vec<u16> = line
        .trim()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect_vec();
    let mut springs = line
        .trim()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .chars()
        .collect_vec();
    let unknowns = springs
        .iter()
        .enumerate()
        .filter(|x| *x.1 == '?')
        .map(|x| x.0)
        .collect_vec();
    let n = unknowns.len() as u32;
    (0..2usize.pow(n))
        .map(|x| {
            for i in 0..n {
                springs
                    .get_mut(unknowns[i as usize])
                    .map(|c| *c = if (x >> i) & 1 == 1 { '#' } else { '.' });
            }
            validate(&springs, &groups)
        })
        .filter(|x| *x)
        .count()
}

fn solve_b(line: &str) -> usize {
    let a = line.trim().split_ascii_whitespace().next().unwrap();
    let b = line.trim().split_ascii_whitespace().last().unwrap();
    let mut a = std::iter::repeat(a).take(5).join("?");
    let b = std::iter::repeat(b).take(5).join(",");
    a.push(' ');
    a.push_str(&b);
    solve_a(&a)
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
    fn test_validate() {
        let data = simple().unwrap();

        data.lines()
            .zip([1, 4, 1, 1, 4, 10])
            .for_each(|x| assert_eq!(solution_a(x.0), Some(x.1)));
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(21));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(525152));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(7344));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
