use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::vec;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let w = input.trim().find('\n').unwrap();
    let data = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .map(|x| x.to_digit(10).unwrap())
        .collect_vec();
    let h = data.len() / w;
    let coord = |i: usize| ((i % w) as i32, (i / w) as i32);
    let trail = |x: i32, y: i32, level: u32| {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x < w && y < h {
            if data.get(x + y * w) == Some(&level) {
                Some((x, y))
            } else {
                None
            }
        } else {
            None
        }
    };

    let start = data
        .iter()
        .enumerate()
        .filter(|x| *x.1 == 0)
        .map(|(i, _)| (i, i))
        .collect_vec();
    let rtv = (1..=9)
        .fold(start, |acc, n| {
            let mut rtv = vec![];

            acc.iter().for_each(|p| {
                let (x, y) = coord(p.0);
                [
                    trail(x + 1, y, n),
                    trail(x - 1, y, n),
                    trail(x, y + 1, n),
                    trail(x, y - 1, n),
                ]
                .iter()
                .filter(|x| x.is_some())
                .for_each(|v| {
                    if let Some((x, y)) = v {
                        rtv.push((x + y * w, p.1));
                    }
                });
            });
            rtv
        })
        .into_iter()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, HashSet<usize>>, x: (usize, usize)| {
                if !acc.contains_key(&x.1) {
                    acc.insert(x.1, HashSet::new());
                }
                if let Some(v) = acc.get_mut(&x.1) {
                    v.insert(x.0);
                }
                acc
            },
        );
    Some(rtv.values().map(|x| x.len()).sum())
}

fn solution_b(input: &str) -> Option<usize> {
    let w = input.trim().find('\n').unwrap();
    let data = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .map(|x| x.to_digit(10).unwrap_or(10))
        .collect_vec();
    let h = data.len() / w;
    let coord = |i: usize| ((i % w) as i32, (i / w) as i32);
    let trail = |x: i32, y: i32, level: u32| {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x < w && y < h {
            let p = x + y * w;
            if data.get(p) == Some(&level) {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    };

    let start = data
        .iter()
        .enumerate()
        .filter(|x| *x.1 == 0)
        .map(|(i, _)| vec![i])
        .collect_vec();
    let rtv = (1..=9)
        .fold(start, |acc, n| {
            let mut rtv = vec![];

            acc.iter().for_each(|path| {
                let (x, y) = coord(*path.last().unwrap());
                [
                    trail(x + 1, y, n),
                    trail(x - 1, y, n),
                    trail(x, y + 1, n),
                    trail(x, y - 1, n),
                ]
                .iter()
                .filter(|x| x.is_some())
                .for_each(|v| {
                    if let Some(p) = v {
                        let mut np = path.clone();
                        np.push(*p);
                        rtv.push(np);
                    }
                });
            });
            rtv
        })
        .into_iter()
        .fold(vec![], |mut acc, x| {
            let v = x
                .iter()
                .enumerate()
                .map(|(i, v)| (i + 1) * v)
                .sum::<usize>();
            acc.push(v);
            acc
        });
    Some(rtv.len())
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
        assert_eq!(solution_a(&data), Some(36));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(81));
    }

    #[test]
    fn test_simple_b1() {
        let data = ".....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....";
        assert_eq!(solution_b(data), Some(3));
    }

    #[test]
    fn test_simple_b2() {
        let data = "..90..9
                    ...1.98
                    ...2..7
                    6543456
                    765.987
                    876....
                    987....";
        assert_eq!(solution_b(data), Some(13));
    }

    #[test]
    fn test_simple_b3() {
        let data = "012345
                    123456
                    234567
                    345678
                    4.6789
                    56789.";
        assert_eq!(solution_b(data), Some(227));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(587));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1340));
    }
}
