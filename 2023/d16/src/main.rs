use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn visitor(input: &str, frm: (usize, usize, usize)) -> usize {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let w = input.lines().next().map(|x| x.len()).unwrap();
    let h = input.lines().count();
    let puzzle = input.chars().filter(|x| !x.is_whitespace()).collect_vec();
    let mut data = puzzle.iter().map(|_| '.').collect_vec();
    let pos = |(x, y, _): (usize, usize, usize)| (y * w + x);
    let mut start: Vec<(usize, usize, usize)> = vec![frm];

    loop {
        println!("{}", start.len());
        let coord = start.pop();
        if coord.is_none() {
            break;
        }
        let mut coord = coord.unwrap();
        let mut mirrored = HashSet::new();

        loop {
            // println!("{} {}", coord.0, coord.1);
            let nd = match puzzle[pos(coord)] {
                '.' => coord.2,
                '-' => match coord.2 {
                    0 => 0,
                    1 => {
                        if data[pos(coord)] == '.' {
                            start.push(coord);
                            2
                        } else {
                            0
                        }
                    }
                    2 => 2,
                    3 => {
                        if data[pos(coord)] == '.' {
                            start.push(coord);
                            0
                        } else {
                            2
                        }
                    }
                    _ => unimplemented!("What a direction!"),
                },
                '|' => match coord.2 {
                    0 => {
                        if data[pos(coord)] == '.' {
                            start.push(coord);
                            1
                        } else {
                            3
                        }
                    }
                    1 => 1,
                    2 => {
                        if data[pos(coord)] == '.' {
                            start.push(coord);
                            3
                        } else {
                            1
                        }
                    }
                    3 => 3,
                    _ => unimplemented!("What a direction!"),
                },
                '/' => match coord.2 {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => unimplemented!("What a direction!"),
                },
                '\\' => match coord.2 {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    3 => 2,
                    _ => unimplemented!("What a direction!"),
                },
                _ => unimplemented!("What a char!"),
            };

            if (puzzle[pos(coord)] == '/' || puzzle[pos(coord)] == '\\') && !mirrored.insert(coord)
            {
                println!("Mirrored: {:?}", coord);
                break;
            }
            if let Some(c) = data.get_mut(pos(coord)) {
                if *c == '.' {
                    *c = '#';
                }
            }

            if let Some(nc) = match nd {
                0 => {
                    if coord.0 + 1 < w {
                        Some((coord.0 + 1, coord.1, nd))
                    } else {
                        None
                    }
                }
                1 => {
                    if coord.1 + 1 < h {
                        Some((coord.0, coord.1 + 1, nd))
                    } else {
                        None
                    }
                }
                2 => {
                    if coord.0 > 0 {
                        Some((coord.0 - 1, coord.1, nd))
                    } else {
                        None
                    }
                }
                3 => {
                    if coord.1 > 0 {
                        Some((coord.0, coord.1 - 1, nd))
                    } else {
                        None
                    }
                }
                _ => unimplemented!("What a direction!"),
            } {
                coord = nc;
            } else {
                break;
            }
        }
    }

    data.iter().filter(|x| **x == '#').count()
}

fn solution_a(input: &str) -> Option<usize> {
   Some(visitor(input, (0, 0, 0)))
}

fn solution_b(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let w = input.lines().next().map(|x| x.len()).unwrap();
    let h = input.lines().count();
    let m = [
    (0..w).map(|x| visitor(&input, (x, 0, 1))).max(),
    (0..w).map(|x| visitor(&input, (x, h-1, 3))).max(),
    (0..h).map(|y| visitor(&input, (0, y, 0))).max(),
    (0..h).map(|y| visitor(&input, (w-1, y, 2))).max()];

    Some(m.iter().fold(0, |accu, x| accu.max(x.unwrap())))
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
        assert_eq!(solution_a(&data), Some(46));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(51));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(7798));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(8026));
    }
}
