use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<u32> {
    let d: Vec<u32> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let w = input
        .chars()
        .enumerate()
        .find(|c| c.1.is_whitespace())
        .unwrap()
        .0;
    let h = d.len() / w;
    let p = |x: usize, y: usize| d.get(y * w + x);
    let up = |x: usize, y: usize| if y > 0 { p(x, y - 1) } else { None };
    let down = |x: usize, y: usize| if y + 1 < h { p(x, y + 1) } else { None };
    let left = |x: usize, y: usize| if x > 0 { p(x - 1, y) } else { None };
    let right = |x: usize, y: usize| if x + 1 < w { p(x + 1, y) } else { None };

    let mut level = 0u32;
    for y in 0..h {
        for x in 0..w {
            let v = p(x, y).unwrap();
            if left(x, y).map_or(true, |w| w > v)
                && right(x, y).map_or(true, |w| w > v)
                && up(x, y).map_or(true, |w| w > v)
                && down(x, y).map_or(true, |w| w > v)
            {
                level += v + 1;
            }
        }
    }
    Some(level)
}

fn solution_b(input: &str) -> Option<usize> {
    let d: Vec<u32> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let w = input
        .chars()
        .enumerate()
        .find(|c| c.1.is_whitespace())
        .unwrap()
        .0;
    let h = d.len() / w;
    let p = |x: usize, y: usize| d.get(y * w + x);
    let up = |x: usize, y: usize| if y > 0 { p(x, y - 1) } else { None };
    let down = |x: usize, y: usize| if y + 1 < h { p(x, y + 1) } else { None };
    let left = |x: usize, y: usize| if x > 0 { p(x - 1, y) } else { None };
    let right = |x: usize, y: usize| if x + 1 < w { p(x + 1, y) } else { None };

    let mut basins: Vec<usize> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let v = p(x, y).unwrap();
            if left(x, y).map_or(true, |w| w > v)
                && right(x, y).map_or(true, |w| w > v)
                && up(x, y).map_or(true, |w| w > v)
                && down(x, y).map_or(true, |w| w > v)
            {
                let mut rto: HashSet<(usize, usize)> = HashSet::new();
                let mut seen: HashSet<(usize, usize)> = HashSet::new();

                rto.insert((x, y));
                while !rto.is_empty() {
                    let k = *rto.iter().next().unwrap();
                    rto.remove(&k);
                    seen.insert(k);
                    up(k.0, k.1).map(|h| {
                        let kk = (k.0, k.1 - 1);
                        if *h < 9 && !seen.contains(&kk) {
                            rto.insert(kk);
                        }
                    });
                    down(k.0, k.1).map(|h| {
                        let kk = (k.0, k.1 + 1);
                        if *h < 9 && !seen.contains(&kk) {
                            rto.insert(kk);
                        }
                    });
                    left(k.0, k.1).map(|h| {
                        let kk = (k.0 - 1, k.1);
                        if *h < 9 && !seen.contains(&kk) {
                            rto.insert(kk);
                        }
                    });
                    right(k.0, k.1).map(|h| {
                        let kk = (k.0 + 1, k.1);
                        if *h < 9 && !seen.contains(&kk) {
                            rto.insert(kk);
                        }
                    });
                }
                basins.push(seen.len());
            }
        }
    }

    Some(
        basins
            .iter()
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, x| acc * x),
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
        assert_eq!(solution_a(&data), Some(15));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(1134));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(512));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1600104));
    }
}
