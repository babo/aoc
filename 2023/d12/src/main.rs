use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use regex::bytes::Regex;
use std::time::Instant;
use std::{collections::HashMap, fs::read_to_string};

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn validate(springs: &[char], groups: &[u16]) -> bool {
    let c = springs.iter().fold_while((None, 0), |accu, x| {
        if accu.0.is_none() {
            Continue((if *x == '.' { None } else { Some(1) }, accu.1))
        } else if *x == '#' {
            Continue((accu.0.map(|i| i + 1), accu.1))
        } else {
            if groups.get(accu.1).map(|x| *x != accu.0.unwrap()) == Some(true) {
                Done((None, accu.1))
            } else {
                Continue((None, accu.1 + 1))
            }
        }
    });
    let is_done = c.is_done();
    let c = c.into_inner();
    !is_done
        && if c.1 + 1 == groups.len() {
            c.0 == groups.last().copied()
        } else if c.1 == groups.len() {
            c.0.is_none()
        } else {
            false
        }
}

fn combinations(springs: &str, groups: &[u16]) -> usize {
    let unknowns = springs
        .chars()
        .enumerate()
        .filter(|x| x.1 == '?')
        .map(|x| x.0)
        .collect_vec();

    let mut springs = springs.chars().collect_vec();
    let n = unknowns.len() as u32;
    (0..2usize.pow(n))
        .map(|x| {
            for i in 0..n {
                if let Some(c) = springs.get_mut(unknowns[i as usize]) {
                    *c = if (x >> i) & 1 == 1 { '#' } else { '.' };
                }
            }
            validate(&springs, groups)
        })
        .filter(|x| *x)
        .count()
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
    let springs = line
        .trim()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .chars()
        .fold((false, "".to_string()), |accu, x| {
            if x == '.' && accu.0 {
                accu
            } else {
                let mut s = accu.1;
                s.push(x);
                (x == '.', s)
            }
        })
        .1;

    combinations(&springs, &groups)
}

struct Buckets {
    cache: HashMap<usize, Option<usize>>,
}

impl Buckets {
    fn new() -> Self {
        let cache = HashMap::new();
        Buckets { cache }
    }

    fn front_first(&mut self, pattern: &str, groups: &[u16]) -> Option<usize> {
        let has_next = groups.len() > 1;
        let min_needed = groups.iter().map(|x| *x as usize).sum::<usize>()
            + if has_next { groups.len() - 1 } else { 0 };

        let mut placement: Vec<Vec<(usize, usize)>> = std::iter::repeat(Vec::new()).take(groups.len()).collect_vec();

        for i in 0..groups.len() {
            let start = if i == 0 { vec![0] } else {
                let len = groups[i-1] as usize + 2;
                placement[i-1].iter().map(|x| x.0 + len).collect_vec()
            };
            let number = groups[i];
            pattern
                .chars()
                .enumerate()
                .skip(*start.iter().min().unwrap())
                .fold_while((None, None, number, false), |accu, (ln, ch)| {
                    match ch {
                        '.' => (),
                        '#' => (),
                        '?' => (),
                        _ => unimplemented!("What a car!")
                    };
                    accu
                });
        }
        None
    }

    fn place(&mut self, pattern: &str, groups: &[u16]) -> Option<usize> {
        println!("place {pattern} {} {:?}", pattern.len(), groups);

        let pattern = pattern.chars().skip_while(|x| *x == '.').join("");
        let has_next = groups.len() > 1;
        let min_needed = groups.iter().map(|x| *x as usize).sum::<usize>()
            + if has_next { groups.len() - 1 } else { 0 };
        if min_needed == 0 {
            println!("End");
            return if pattern.chars().filter(|x| *x == '#').count() == 0 {
                Some(1)
            } else {
                None
            };
        }
        if pattern.len() < min_needed {
            println!("Not enough {min_needed}");
            return None;
        }

        let key = pattern.len();
        if self.cache.contains_key(&key) {
            return self.cache.get(&key).copied().unwrap();
        }

        let number = *groups.first().unwrap();
        let result =
            pattern
                .chars()
                .enumerate()
                .fold((None, None, number, false), |accu, (ln, ch)| {
                    if accu.0.is_some() {
                        accu
                    } else {
                        match ch {
                            '.' => {
                                if accu.3 {
                                    (Some(accu.1), None, 0, false)
                                } else if accu.1.is_some() {
                                    (Some(None), None, 0, false)
                                } else {
                                    accu
                                }
                            }
                            '#' => {
                                if accu.3 {
                                    (Some(None), None, 0, false)
                                } else if accu.1.is_some() {
                                    if accu.2 > 1 {
                                        (None, accu.1, accu.2 - 1, false)
                                    } else {
                                        if has_next {
                                            (None, accu.1, 0, true)
                                        } else {
                                            (Some(accu.1), None, 0, false)
                                        }
                                    }
                                } else {
                                    (None, Some(ln), number - 1, number == 1)
                                }
                            }
                            '?' => {
                                if accu.3 {
                                    (Some(accu.1), None, 0, false)
                                } else if accu.1.is_some() {
                                    if accu.2 > 1 {
                                        (None, accu.1, accu.2 - 1, false)
                                    } else {
                                        if has_next {
                                            (None, accu.1, 0, true)
                                        } else {
                                            (Some(accu.1), None, 0, false)
                                        }
                                    }
                                } else {
                                    (None, Some(ln), number - 1, number == 1)
                                }
                            }
                            _ => unimplemented!("what a char!"),
                        }
                    }
                });
        let retval = if let Some(Some(frm)) = result.0 {
            let next = frm + number as usize + if has_next { 1 } else { 0 };
            let position = pattern.len() - frm;
            let below = self.place(&pattern[next..], &groups[1..]);
            let wildcard = if pattern.chars().nth(frm) == Some('?') {
                println!("Check wildcard");
                self.place(&pattern[frm + 1..], groups)
            } else {
                None
            };
            println!("Got: {position} Below: {:?} Wildcard: {:?}", below, wildcard);
            let total = below.map_or(0, |x| x) + wildcard.map_or(0, |x| x);
            let total = if total != 0 { Some(total) } else { None };

            total
        } else if result.1.is_some() && result.2 == 0 {
            println!("Found end of pattern");
            Some(1)
        } else {
            let reduced = pattern.chars().skip_while(|x| *x == '.').join("");
            if reduced.chars().next() == Some('?') {
                println!("Failed on wildcard, skip it: {reduced}");
                self.place(&reduced[1..], groups)
            } else {
                println!("Doesn't match");
                None
            }
        };
        self.cache.insert(pattern.len(), retval);
        retval
    }
}

fn combo(pattern: &str, groups: &[u16]) -> usize {
    let mut buckets = Buckets::new();
    buckets.place(pattern, groups).unwrap()
}

fn solve_b(line: &str) -> usize {
    let mut groups: Vec<u16> = line
        .trim()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect_vec();
    let springs = line.trim().split_ascii_whitespace().next().unwrap();
    let r = combinations(springs, &groups);

    let mut extra = springs.to_string();
    extra.push('?');
    extra.push_str(springs);
    groups.append(&mut groups.iter().copied().collect_vec());
    let b = combinations(&extra, &groups);
    let n = b * (b / r) * (b / r) * (b / r);
    println!("{line}");
    println!("{r} {b} {n}");
    n
}

fn solve_c(line: &str) -> usize {
    let a = line.trim().split_ascii_whitespace().next().unwrap();
    let b = line.trim().split_ascii_whitespace().last().unwrap();
    let a = std::iter::repeat(a).take(5).join("?");
    let b = std::iter::repeat(b)
        .take(5)
        .join(",")
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect_vec();

    combo(&a, &b)
}

fn solve_d(line: &str) -> usize {
    let a = line.trim().split_ascii_whitespace().next().unwrap();
    let b = line.trim().split_ascii_whitespace().last().unwrap();
    let b = b
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect_vec();

    combo(&a, &b)
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    let now = Instant::now();
    Some(
        input
            .lines()
            .enumerate()
            .map(|x| {
                if x.0 % 10 == 0 && x.0 > 0 {
                    let elapsed_time = now.elapsed();
                    println!("round: {} elapsed: {}s", x.0, elapsed_time.as_secs());
                }
                solve_b(x.1.trim())
            })
            .sum::<usize>(),
    )
}

fn solution_c(input: &str) -> Option<usize> {
    let now = Instant::now();
    Some(
        input
            .lines()
            .enumerate()
            .map(|x| {
                if x.0 % 50 == 0 && x.0 > 0 {
                    let elapsed_time = now.elapsed();
                    println!("round: {} elapsed: {}s", x.0, elapsed_time.as_secs());
                }
                solve_c(x.1.trim())
            })
            .sum::<usize>(),
    )
}

fn solution_d(input: &str) -> Option<usize> {
    let now = Instant::now();
    Some(
        input
            .lines()
            .enumerate()
            .map(|x| {
                if x.0 % 50 == 0 && x.0 > 0 {
                    let elapsed_time = now.elapsed();
                    println!("round: {} elapsed: {}s", x.0, elapsed_time.as_secs());
                }
                solve_d(x.1.trim())
            })
            .sum::<usize>(),
    )
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let d = solution_c(&c);
    let b = solution_b(&c);

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
    println!("Step C: {:?}", d);
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
    fn test_single_rule() {
        let data = "??##????#???.?.????# 7,1,2,2";

        assert_eq!(solve_a(data), 5);
    }

    #[test]
    fn test_validate_a() {
        let data = simple().unwrap();

        data.lines()
            .zip([1, 4, 1, 1, 4, 10])
            .for_each(|x| assert_eq!(solution_a(x.0), Some(x.1)));
    }

    #[test]
    fn test_validate_b() {
        let data = simple().unwrap();

        data.lines()
            .zip([1, 16384, 1, 16, 2500, 506250])
            .for_each(|x| assert_eq!(solution_b(x.0), Some(x.1)));
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
    fn test_simple_c() {
        let data = simple().unwrap();
        assert_eq!(solution_c(&data), Some(525152));
    }

    #[test]
    fn test_single() {
        let groups = [1];
        let mut buckets = Buckets::new();

        assert_eq!(buckets.place("?", &groups), Some(1));
        assert_eq!(buckets.place("#", &groups), Some(1));
    }

    #[test]
    fn test_combo() {
        let data = "?.?.?.??????#.#";
        let a = [1, 1, 1, 2, 1];
        let mut buckets = Buckets::new();
        let prev = combinations(&data, &a);

        assert_eq!(buckets.place(data, &a), Some(prev));
    }

    #[test]
    fn test_length() {
        let c = content().unwrap();
        let d: usize = c
            .lines()
            .map(|line| {
                let nums: Vec<usize> = line
                    .trim()
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec();
                let r = nums.iter().sum::<usize>() + nums.len() - 1;
                let l = line
                    .trim()
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .chars()
                    .fold((false, "".to_string()), |accu, x| {
                        if x == '.' && accu.0 {
                            accu
                        } else {
                            let mut s = accu.1;
                            s.push(x);
                            (x == '.', s)
                        }
                    })
                    .1;

                if l.len() > r {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>();
        assert_eq!(d, 0);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(7344));
    }

    #[test]
    fn test_lib() {
        let r = "...".to_string().replace("..", ".");
        assert_eq!(r, ".");
    }

    #[test]
    fn test_solution_d() {
        let c = content().unwrap();
        assert_eq!(solution_d(&c), Some(7344));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let result = solution_c(&c);
        // Higher than 709232705819
        assert_eq!(result.map(|x| x > 709232705819), Some(true));
        assert_eq!(result, Some(3000));
    }
}
