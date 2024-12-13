use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn spend_a(claw_a: (usize, usize), claw_b: (usize, usize), prize: (usize, usize)) -> usize {
    (0..100)
        .map(|a| {
            let (x, y) = (claw_a.0 * a, claw_a.1 * a);
            if x > prize.0 || y > prize.1 {
                return usize::MAX;
            }
            let b = (prize.0 - x) / claw_b.0;
            if (prize.0 - x) % claw_b.0 != 0 {
                return usize::MAX;
            }
            if y + b * claw_b.1 != prize.1 {
                return usize::MAX;
            }

            println!("{} {}", a, b);
            a * 3 + b
        })
        .min()
        .map(|x| if x == usize::MAX { 0 } else { x })
        .unwrap()
}

fn spend_b((ax, ay): (usize, usize), (bx, by): (usize, usize), (px, py): (usize, usize)) -> usize {
    let (ax, ay) = (ax as i64, ay as i64);
    let (bx, by) = (bx as i64, by as i64);
    let (px, py) = (px as i64, py as i64);
    //assert!(ax * py > ay * px);
    let dd = ax * py - ay * px;
    //assert!(ax * by > ay * bx);
    let ds = ax * by - ay * bx;
    if dd % ds != 0 {
        return 0;
    }
    let m = dd / ds;
    let dd = px - m * bx;
    let ds = ax;
    if dd % ds != 0 {
        return 0;
    }
    let n = dd / ds;
    (3 * n + m) as usize
}

fn read_input(input: &str) -> Vec<((usize, usize), (usize, usize), (usize, usize))> {
    let (machines, _): (
        _,
        (
            Option<(usize, usize)>,
            Option<(usize, usize)>,
            Option<(usize, usize)>,
        ),
    ) = input
        .lines()
        .fold((vec![], (None, None, None)), |(mut acc, claw), line| {
            let parts = line.trim().split(": ").collect_vec();
            if parts.len() != 2 {
                return (acc, claw);
            }
            let port: Vec<usize> = parts[1]
                .split_ascii_whitespace()
                .map(|x| {
                    x.chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect();
            let xy = (*port.get(0).unwrap(), *port.get(1).unwrap());
            let kind = parts[0];
            if kind == "Button A" {
                (acc, (Some(xy), None, None))
            } else if kind == "Button B" {
                (acc, (claw.0, Some(xy), None))
            } else if kind == "Prize" {
                acc.push((claw.0.unwrap(), claw.1.unwrap(), xy));
                (acc, (None, None, None))
            } else {
                unreachable!("Unexpected input")
            }
        });
    machines
}

fn solution_a(input: &str) -> Option<usize> {
    let machines = read_input(input);

    Some(
        machines
            .iter()
            .map(|x| spend_b(x.0, x.1, x.2))
            .sum::<usize>(),
    )
}

fn solution_b(input: &str) -> Option<usize> {
    let d = 10000000000000;
    let machines = read_input(input)
        .iter()
        .map(|x| (x.0, x.1, (x.2 .0 + d, x.2 .1 + d)))
        .collect_vec();

    Some(
        machines
            .iter()
            .map(|x| spend_b(x.0, x.1, x.2))
            .sum::<usize>(),
    )
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
        assert_eq!(solution_a(&data), Some(480));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(875318608908));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(37297));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(83197086729371));
    }
}
