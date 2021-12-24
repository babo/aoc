use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn neighbors(pos: usize, w: usize) -> Vec<usize> {
    let mut rtv: Vec<usize> = Vec::new();
    let x = pos % w;
    let y = pos / w;

    if x > 0 {
        rtv.push(pos - 1);
    }
    if x + 1 < w {
        rtv.push(pos + 1);
    }
    if y > 0 {
        rtv.push(pos - w);
        if x > 0 {
            rtv.push(pos - w - 1);
        }
        if x + 1 < w {
            rtv.push(pos - w + 1);
        }
    }
    if y + 1 < w {
        rtv.push(pos + w);
        if x > 0 {
            rtv.push(pos + w - 1);
        }
        if x + 1 < w {
            rtv.push(pos + w + 1);
        }
    }
    rtv
}

fn solution_a(input: &str) -> Option<usize> {
    let w = input.lines().next().map(|x| x.len()).unwrap();
    let mut data: Vec<u32> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut flashes = 0usize;
    for _ in 0..100 {
        let mut done: HashSet<usize> = HashSet::new();
        let mut flashed: Vec<usize> = Vec::new();
        for x in data.iter_mut().enumerate() {
            if *x.1 < 9 {
                *x.1 += 1;
            } else {
                *x.1 = 0;
                done.insert(x.0);
                flashed.push(x.0);
            }
        }

        while !flashed.is_empty() {
            flashed.pop().map(|p| {
                for nb in neighbors(p, w) {
                    if !done.contains(&nb) {
                        data.get_mut(nb).map(|w| {
                            if *w < 9 {
                                *w += 1;
                            } else {
                                *w = 0;
                                done.insert(nb);
                                flashed.push(nb);
                            }
                        });
                    }
                }
            });
        }

        flashes += done.len();
    }
    Some(flashes)
}

fn solution_b(input: &str) -> Option<usize> {
    let w = input.lines().next().map(|x| x.len()).unwrap();
    let mut data: Vec<u32> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut steps = 0usize;
    for _ in 0..1000 {
        let mut done: HashSet<usize> = HashSet::new();
        let mut flashed: Vec<usize> = Vec::new();
        for x in data.iter_mut().enumerate() {
            if *x.1 < 9 {
                *x.1 += 1;
            } else {
                *x.1 = 0;
                done.insert(x.0);
                flashed.push(x.0);
            }
        }

        while !flashed.is_empty() {
            flashed.pop().map(|p| {
                for nb in neighbors(p, w) {
                    if !done.contains(&nb) {
                        data.get_mut(nb).map(|w| {
                            if *w < 9 {
                                *w += 1;
                            } else {
                                *w = 0;
                                done.insert(nb);
                                flashed.push(nb);
                            }
                        });
                    }
                }
            });
        }

        steps += 1;
        if done.len() == w * w {
            break;
        }
    }
    Some(steps)
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
        assert_eq!(solution_a(&data), Some(1656));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(195));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1599));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(418));
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(neighbors(12, 5).len(), 8);
        assert_eq!(neighbors(0, 5).len(), 3);
        assert_eq!(neighbors(2, 5).len(), 5);
        assert_eq!(neighbors(4, 5).len(), 3);
        assert_eq!(neighbors(5, 5).len(), 5);
        assert_eq!(neighbors(9, 5).len(), 5);
        assert_eq!(neighbors(20, 5).len(), 3);
        assert_eq!(neighbors(22, 5).len(), 5);
        assert_eq!(neighbors(24, 5).len(), 3);
    }
}
