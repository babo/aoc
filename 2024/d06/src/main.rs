use core::net;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solution_a(input: &str) -> Option<usize> {
    let w = input
        .trim()
        .chars()
        .find_position(|x| x.is_ascii_whitespace())
        .unwrap()
        .0;
    let h = input.lines().count();
    let to_coord = |p: usize| (p % w, p / w);
    let data = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect_vec();
    assert_eq!(w * h, data.len());

    let at = |x: usize, y: usize| data[y * w + x];
    let mut guard = to_coord(data.iter().position(|x| *x == '^').unwrap());
    let mut been = HashSet::new();
    let mut dir = Direction::Up;

    loop {
        been.insert(guard);

        match dir {
            Direction::Up => {
                if guard.1 == 0 {
                    break;
                }
                if at(guard.0, guard.1 - 1) == '#' {
                    dir = Direction::Right;
                } else {
                    guard.1 -= 1;
                }
            }
            Direction::Down => {
                if guard.1 == h - 1 {
                    break;
                }
                if at(guard.0, guard.1 + 1) == '#' {
                    dir = Direction::Left;
                } else {
                    guard.1 += 1;
                }
            }
            Direction::Right => {
                if guard.0 == w - 1 {
                    break;
                }
                if at(guard.0 + 1, guard.1) == '#' {
                    dir = Direction::Down;
                } else {
                    guard.0 += 1;
                }
            }
            Direction::Left => {
                if guard.0 == 0 {
                    break;
                }
                if at(guard.0 - 1, guard.1) == '#' {
                    dir = Direction::Up;
                } else {
                    guard.0 -= 1;
                }
            }
        }
    }

    Some(been.len())
}

fn nightwatch(data: &Vec<char>, wh: (usize, usize), obs: (usize, usize)) -> usize {
    let (w, h) = wh;
    let at = |p: (usize, usize)| data[p.1 * w + p.0];
    let to_coord = |p: usize| (p % w, p / w);

    let mut guard = to_coord(data.iter().position(|x| *x == '^').unwrap());
    let mut been = HashSet::new();
    let mut dir = Direction::Up;

    loop {
        if !been.insert((guard, dir.clone())) {
            return 1;
        }

        match dir {
            Direction::Up => {
                if guard.1 == 0 {
                    break;
                }
                let np = (guard.0, guard.1 - 1);
                if at(np) == '#' || np == obs {
                    dir = Direction::Right;
                } else {
                    guard = np;
                }
            }
            Direction::Down => {
                if guard.1 == h - 1 {
                    break;
                }
                let np = (guard.0, guard.1 + 1);
                if at(np) == '#' || np == obs {
                    dir = Direction::Left;
                } else {
                    guard = np;
                }
            }
            Direction::Right => {
                if guard.0 == w - 1 {
                    break;
                }
                let np = (guard.0 + 1, guard.1);
                if at(np) == '#' || np == obs {
                    dir = Direction::Down;
                } else {
                    guard = np;
                }
            }
            Direction::Left => {
                if guard.0 == 0 {
                    break;
                }
                let np = (guard.0 - 1, guard.1);
                if at(np) == '#' || np == obs {
                    dir = Direction::Up;
                } else {
                    guard = np;
                }
            }
        }
    }

    0
}

fn solution_b(input: &str) -> Option<usize> {
    let w = input
        .trim()
        .chars()
        .find_position(|x| x.is_ascii_whitespace())
        .unwrap()
        .0;
    let h = input.lines().count();
    let data = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect_vec();
    let at = |x: usize, y: usize| data[y * w + x];
    let mut count = 0;

    for x in 0..w {
        for y in 0..h {
            if at(x, y) == '.' {
                count += nightwatch(&data, (w, h), (x, y));
            }
        }
    }

    Some(count)
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
        assert_eq!(solution_a(&data), Some(41));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(6));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(4515));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1309));
    }
}
