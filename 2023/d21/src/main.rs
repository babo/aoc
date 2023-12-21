use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_b(_line: &str) -> usize {
    0
}

fn solution_a(input: &str, steps: usize) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let w = input.lines().next().map(|x| x.len()).unwrap();
    let input = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect_vec();
    let h = input.len() / w;
    let at = |(x, y): &(usize, usize)| (x + y * w);
    let s = input.iter().position(|x| *x == 'S').unwrap();
    let s = (s % w, s / w);

    let hs: HashSet<(usize, usize)> = (0..steps).fold(
        HashSet::from_iter(std::iter::repeat(s).take(1)),
        |current, _step| {
            HashSet::from_iter(
                current
                    .iter()
                    .map(|(x, y)| {
                        vec![
                            (*x as i64 - 1, *y as i64),
                            (*x as i64 + 1, *y as i64),
                            (*x as i64, *y as i64 + 1),
                            (*x as i64, *y as i64 - 1),
                        ]
                    })
                    .flatten()
                    .filter(|xy| xy.0 >= 0 && xy.0 < w as i64 && xy.1 >= 0 && xy.1 < h as i64)
                    .map(|xy| (xy.0 as usize, xy.1 as usize))
                    .filter(|xy| input[at(xy)] != '#'),
            )
        },
    );
    hs.iter().for_each(|xy| assert_ne!(input[at(xy)], '#'));
    Some(hs.len())
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<usize>())
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c, 64);
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
        assert_eq!(solution_a(&data, 6), Some(16));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c, 64), Some(3699));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
