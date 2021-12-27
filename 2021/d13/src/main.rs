use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::FromIterator;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Fold {
    H(usize),
    V(usize),
}

fn solution_a(input: &str) -> Option<usize> {
    let folds: Vec<Fold> = input
        .lines()
        .skip_while(|line| !line.starts_with("fold along "))
        .map(|line| {
            line.find('=')
                .map(|p| {
                    line.get(p + 1..)
                        .map(|n| {
                            let amount = usize::from_str_radix(n, 10).ok().unwrap();
                            line.get(p - 1..p)
                                .map(|c| {
                                    if c.eq("x") {
                                        Fold::V(amount)
                                    } else {
                                        Fold::H(amount)
                                    }
                                })
                                .unwrap()
                        })
                        .unwrap()
                })
                .unwrap()
        })
        .collect();

    let marks: Vec<(usize, usize)> = input
        .lines()
        .map_while(|line| {
            let data: Vec<Option<usize>> = line
                .split(',')
                .map(|d| usize::from_str_radix(d, 10).ok())
                .collect();
            if data.len() == 2 {
                Some((data.get(0).unwrap().unwrap(), data.get(1).unwrap().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let visible: HashSet<(usize, usize)> =
        folds.iter().take(1).fold(HashSet::new(), |mut acc, f| {
            match f {
                Fold::H(yy) => marks.iter().for_each(|xy| {
                    if xy.1 < *yy {
                        acc.insert(*xy);
                    } else {
                        acc.insert((xy.0, yy - (xy.1 - yy)));
                    }
                }),
                Fold::V(xx) => marks.iter().for_each(|xy| {
                    if xy.0 < *xx {
                        acc.insert(*xy);
                    } else {
                        acc.insert((*xx - (xy.0 - *xx), xy.1));
                    }
                }),
            }
            acc
        });

    Some(visible.len())
}

fn solution_b(input: &str) -> Option<usize> {
    let folds: Vec<Fold> = input
        .lines()
        .skip_while(|line| !line.starts_with("fold along "))
        .map(|line| {
            line.find('=')
                .map(|p| {
                    line.get(p + 1..)
                        .map(|n| {
                            let amount = usize::from_str_radix(n, 10).ok().unwrap();
                            line.get(p - 1..p)
                                .map(|c| {
                                    if c.eq("x") {
                                        Fold::V(amount)
                                    } else {
                                        Fold::H(amount)
                                    }
                                })
                                .unwrap()
                        })
                        .unwrap()
                })
                .unwrap()
        })
        .collect();

    let marks: Vec<(usize, usize)> = input
        .lines()
        .map_while(|line| {
            let data: Vec<Option<usize>> = line
                .split(',')
                .map(|d| usize::from_str_radix(d, 10).ok())
                .collect();
            if data.len() == 2 {
                Some((data.get(0).unwrap().unwrap(), data.get(1).unwrap().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let visible: Vec<(usize, usize)> = folds.iter().fold(marks, |acc, fold_along| {
        let f = |xy: &(usize, usize)| -> (usize, usize) {
            match fold_along {
                Fold::H(yy) => {
                    if xy.1 < *yy {
                        *xy
                    } else {
                        (xy.0, yy - (xy.1 - yy))
                    }
                }
                Fold::V(xx) => {
                    if xy.0 < *xx {
                        *xy
                    } else {
                        (*xx - (xy.0 - *xx), xy.1)
                    }
                }
            }
        };

        let visible: HashSet<(usize, usize)> = acc.iter().fold(HashSet::new(), |mut acc, xy| {
            acc.insert(f(xy));
            acc
        });
        visible
            .iter()
            .map(|xy| xy.clone())
            .sorted()
            .collect::<Vec<(usize, usize)>>()
    });

    let xx: (usize, usize) = visible
        .iter()
        .fold(None, |acc: Option<(usize, usize)>, xy| {
            if acc.is_none() {
                Some((xy.0, xy.0))
            } else {
                acc.map(|acc| (acc.0.min(xy.0), acc.0.max(xy.0)))
            }
        })
        .unwrap();
    let yy: (usize, usize) = visible
        .iter()
        .fold(None, |acc: Option<(usize, usize)>, xy| {
            if acc.is_none() {
                Some((xy.1, xy.1))
            } else {
                acc.map(|acc| (acc.0.min(xy.1), acc.1.max(xy.1)))
            }
        })
        .unwrap();
    let w = 2 + xx.1 - xx.0;
    let h = 1 + yy.1 - yy.0;
    let mut pr: Vec<char> = std::iter::repeat(' ').take(w * h).collect();
    for y in 0..h {
        pr.get_mut(y * w + w - 1).map(|p| *p = '\n');
    }
    visible.iter().for_each(|xy| {
        pr.get_mut(xy.1 * (w + 0) + xy.0).map(|p| *p = 'X');
    });
    println!("{}", String::from_iter(pr.iter()));

    Some(visible.len())
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
        assert_eq!(solution_a(&data), Some(17));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(16));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(770));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(102));
    }
}
