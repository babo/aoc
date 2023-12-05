use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| (x, x))
        .collect_vec();

    let ready = input.lines().skip(2).fold(seeds, |accu, line| {
        let line = line.trim();
        if line.is_empty() {
            println!("{:?}", accu);
            return accu.iter().map(|(_, x)| (*x, *x)).collect_vec();
        }
        if line.ends_with(" map:") {
            println!("{line}");
            return accu;
        }
        let (dest, from, width) = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        accu.iter()
            .map(|x| {
                let (a, b) = *x;
                if a >= from && a < from + width {
                    (a, dest + a - from)
                } else {
                    (a, b)
                }
            })
            .collect_vec()
    });
    ready.iter().map(|x| x.1).min()
}

fn solution_b(input: &str) -> Option<usize> {
    let seeds: & mut Vec<(usize, usize)> = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
        .fold((None, &mut Vec::new()), |(prev, accu), (i, x)| {
            if i % 2 == 0 {
                (Some(x), accu)
            } else {
                accu.push((prev.unwrap(), x));
                (None, accu)
            }
        })
        .1;

    type Inner = (
        (usize, usize),
        Option<(usize, usize)>,
        Option<(usize, usize)>,
    );

    let ready = input.lines().skip(2).fold(
        (seeds, None),
        |(accum, mapper): (&mut Vec<(usize, usize)>, Option<Vec<Inner>>), line: &str| {
            let line = line.trim();
            if line.is_empty() {
                println!("{:?}", accum);
                let mapper = mapper.unwrap();
                let after =
                    accum
                        .iter()
                        .enumerate()
                        .fold(&mut Vec::new(), |accu, (i, x)| {
                            let rules = mapper.get(i).unwrap();
                            accu.push(rules.0);
                            rules.1.map(|x| accu.push(x));
                            rules.2.map(|x| accu.push(x));
                            accu
                        });

                return (accum, None);
            }
            if line.ends_with(" map:") {
                println!("{line}");
                let mut initial: Vec<Inner> = Vec::new();
                accum.iter().for_each(|x| initial.push((*x, None, None)));

                return (accum, Some(initial));
            }
            /*
            let mapper = mapper.unwrap();
            let (dest, from, width) = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            accu.iter().enumerate().for_each(|(i, x)| {
                let (a, b) = *x;
                let to = from + width;
                if b < from || a > to {
                    ()
                } else if from <= a && to >= b {
                    let alma: Inner = ((dest + a - from, dest + b - from), None, None);
                    if let Some(val) = mapper.get_mut(i) {
                        *val = alma;
                    }
                }
            });
            */
            (accum, mapper)
        },
    );
    Some(0)
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
        assert_eq!(solution_a(&data), Some(35));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(46));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(535088217));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(51399228));
    }
}
