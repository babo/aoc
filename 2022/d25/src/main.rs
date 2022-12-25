use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Snafu {
    fiver: String,
    decimal: usize,
}

impl Snafu {
    fn from(orig: &str) -> Self {
        let num = orig.chars().rev().fold((0usize, 0usize, 1), |prev, c| {
            let d: usize = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => 1,
                '=' => 2,
                _ => unreachable!("Snafu"),
            };
            if c.is_ascii_digit() {
                (prev.0 + prev.2 * d, prev.1, prev.2 * 5)
            } else {
                (prev.0, prev.1 + prev.2 * d, prev.2 * 5)
            }
        });
        assert!(num.0 >= num.1);
        let decimal = num.0 - num.1;
        Snafu {
            fiver: orig.to_string(),
            decimal,
        }
    }

    fn to(num: usize) -> Self {
        let mut res = Vec::new();
        let mut num = num;

        loop {
            let (carry, d) = match num % 5 {
                0 => (0, '0'),
                1 => (0, '1'),
                2 => (0, '2'),
                3 => (1, '='),
                4 => (1, '-'),
                _ => unreachable!("No way!"),
            };
            res.push(d);
            num /= 5;
            num += carry;
            if num == 0 {
                break;
            }
        }
        let fiver: String = res.iter().rev().collect();

        Snafu { fiver, decimal: num }
    }
}

fn solution_a(input: &str) -> Option<String> {
    let s: usize = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| Snafu::from(x).decimal)
        .sum();
    println!("{s}");
    Some(Snafu::to(s).fiver)
}

fn solution_b(_input: &str) -> Option<usize> {
    None
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
    use itertools::Itertools;

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
        assert_eq!(solution_a(&data).map(|x| x.eq("2=-1=0")), Some(true));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c).map(|x| x == "2=2-1-010==-0-1-=--2"), Some(true));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), None);
    }

    #[test]
    fn test_snafu_from() {
        assert_eq!(Snafu::from("1=-0-2").decimal, 1747);
        assert_eq!(Snafu::from("1=-1=").decimal, 353);
    }

    #[test]
    fn test_snafu() {
        read_to_string("./samples.txt")
            .unwrap()
            .lines()
            .for_each(|line| {
                let p = line
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .collect_vec();
                let d = usize::from_str_radix(p[0], 10).unwrap();
                assert_eq!(Snafu::from(p[1]).decimal, d);
                assert_eq!(Snafu::to(d).fiver, p[1]);
            });
        assert_eq!(Snafu::to(2).fiver, "2");
        assert_eq!(Snafu::to(1257).fiver, "20012");
        assert_eq!(Snafu::to(3).fiver, "1=");
        assert_eq!(Snafu::to(353).fiver, "1=-1=");
        assert_eq!(Snafu::to(1747).fiver, "1=-0-2");
    }
}
