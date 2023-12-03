use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_a(full: &[bool], lnumber: usize, line: &str) -> usize {
    let w = line.len();

    println!();
    line.chars()
        .enumerate()
        .fold((0, 0, false), |acc, x| {
            if x.1.is_numeric() {
                let n = acc.1 * 10 + x.1.to_digit(10).unwrap() as usize;
                let c = acc.2
                    || (acc.1 == 0 && x.0 > 0 && full.get(lnumber * w + x.0 - 1) == Some(&true))
                    || (x.0 + 1 < w && full.get(lnumber * w + x.0 + 1) == Some(&true))
                    || (lnumber > 0 && full.get((lnumber - 1) * w + x.0) == Some(&true))
                    || (lnumber > 0
                        && x.0 + 1 < w
                        && full.get((lnumber - 1) * w + x.0 + 1) == Some(&true))
                    || (lnumber > 0
                        && x.0 > 0
                        && full.get((lnumber - 1) * w + x.0 - 1) == Some(&true))
                    || full.get((lnumber + 1) * w + x.0) == Some(&true)
                    || (x.0 > 0 && full.get((lnumber + 1) * w + x.0 - 1) == Some(&true))
                    || (x.0 + 1 < w && full.get((lnumber + 1) * w + x.0 + 1) == Some(&true));
                if x.0 + 1 == w && c {
                    println!("{} => {}", lnumber, n);
                    (acc.0 + n, 0, false)
                } else {
                    (acc.0, n, c)
                }
            } else if acc.2 {
                println!("{} => {}", lnumber, acc.1);
                (acc.0 + acc.1, 0, false)
            } else {
                (acc.0, 0, false)
            }
        })
        .0
}

fn solve_b(full: &[bool], lnumber: usize, line: &str) -> Vec<(usize, usize)> {
    let w = line.len();

    let mut fc: Vec<(usize, usize)> = Vec::new();
    let mut found = None;

    line.chars().enumerate().fold((0, false), |acc, x| {
        let mut f = |n: usize| {
            let r = full.get(n) == Some(&true);
            if r {
                found = Some(n);
            } else {
                found = None;
            }
            r
        };

        if x.1.is_numeric() {
            let n = acc.0 * 10 + x.1.to_digit(10).unwrap() as usize;
            let c = acc.1
                || (acc.0 == 0 && x.0 > 0 && f(lnumber * w + x.0 - 1))
                || (x.0 + 1 < w && f(lnumber * w + x.0 + 1))
                || (lnumber > 0 && f((lnumber - 1) * w + x.0))
                || (lnumber > 0 && x.0 + 1 < w && f((lnumber - 1) * w + x.0 + 1))
                || (lnumber > 0 && x.0 > 0 && f((lnumber - 1) * w + x.0 - 1))
                || f((lnumber + 1) * w + x.0)
                || (x.0 > 0 && f((lnumber + 1) * w + x.0 - 1))
                || (x.0 + 1 < w && f((lnumber + 1) * w + x.0 + 1));
            if x.0 + 1 == w && c {
                found.map(|x| fc.push((x, n)));
                (0, false)
            } else {
                (n, c)
            }
        } else if acc.1 {
            found.map(|x| fc.push((x, acc.0)));
            (0, false)
        } else {
            (0, false)
        }
    });
    fc
}

fn solution_a(input: &str) -> Option<usize> {
    let full = input
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(|x| x != '.' && !x.is_ascii_digit())
        .collect::<Vec<bool>>();
    Some(
        input
            .lines()
            .enumerate()
            .map(|x| solve_a(&full, x.0, x.1))
            .sum::<usize>(),
    )
}

fn solution_b(input: &str) -> Option<usize> {
    let full = input
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(|x| x == '*')
        .collect::<Vec<bool>>();
    let mut found: Vec<(usize, usize)> = Vec::new();
    input
        .lines()
        .enumerate()
        .for_each(|x| found.extend(solve_b(&full, x.0, x.1)));

    let mut res = 0usize;
    found.iter().enumerate().for_each(|x| {
        if let Some((_, n)) = found.iter().skip(x.0 + 1).find(|after| after.0 == x.1 .0) {
            res += x.1 .1 * n
        }
        println!("{} {}", x.1 .0, x.1 .1);
    });
    Some(res)
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
        assert_eq!(solution_a(&data), Some(4361));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(467835));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(527144));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(81463996));
    }
}
