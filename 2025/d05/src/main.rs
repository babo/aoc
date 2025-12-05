use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let mut res: (Vec<(usize, usize)>, Vec<usize>) =
        input
            .lines()
            .fold((Vec::new(), Vec::new()), |(mut rs, mut ns), line| {
                if line.contains('-') {
                    let lh: (usize, usize) = line
                        .trim()
                        .split('-')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    rs.push(lh);
                } else {
                    let _ = line.parse::<usize>().and_then(|n| {
                        ns.push(n);
                        Ok(())
                    });
                }
                (rs, ns)
            });
    res.0.sort_by_key(|x| x.0);

    let rm = res.0.len();
    Some(
        res.1
            .iter()
            .map(|n| {
                for i in 0..rm {
                    let lh = res.0.get(i).unwrap();
                    if *n >= lh.0 && *n <= lh.1 {
                        return 1;
                    }
                }
                0
            })
            .sum(),
    )
}

fn solution_b(input: &str) -> Option<usize> {
    let mut res: Vec<(usize, usize)> =
        input
            .lines()
            .take_while(|l| l.contains('-'))
            .fold(Vec::new(), |mut acc, line| {
                let lh: (usize, usize) = line
                    .trim()
                    .split('-')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                acc.push(lh);
                acc
            });
    res.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    let mut rtv = 0usize;
    let (mut l, mut h) = *res.first().unwrap();
    res.iter().skip(1).for_each(|x| {
        if h < x.0 {
            rtv += h - l + 1;
            (l, h) = *x;
        } else if h <= x.1 {
            h = x.1
        }
    });
    rtv += 1 + h - l;

    Some(rtv)
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
        assert_eq!(solution_a(&data), Some(3));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(14));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(744));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(347468726696961));
    }
}
