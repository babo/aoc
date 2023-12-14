use itertools::Itertools;
use std::fs::read_to_string;
use std::time::Instant;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_a(data: &str) -> usize {
    let w = data.lines().next().map(|x| x.len()).unwrap();

    data.lines()
        .enumerate()
        .fold(
            (0, std::iter::repeat(0).take(w).collect_vec()),
            |(mut weight, mut bumps), (n, line)| {
                for (col, c) in line.chars().enumerate() {
                    match c {
                        '#' => {
                            if let Some(b) = bumps.get_mut(col) {
                                *b = n + 1;
                            }
                        }
                        'O' => {
                            weight += w - bumps[col];
                            if let Some(b) = bumps.get_mut(col) {
                                *b += 1;
                            }
                        }
                        '.' => (),
                        _ => unimplemented!("Unexpected character"),
                    }
                }
                (weight, bumps)
            },
        )
        .0
}

fn solve_b(data: &str, cycles: usize) -> usize {
    let w = data.lines().next().map(|x| x.len()).unwrap();
    let mut data = data.chars().filter(|x| !x.is_whitespace()).collect_vec();

    let now = Instant::now();
    for cycle in 0..cycles {
        for direction in 0..4 {
            let mut bumps = std::iter::repeat(if direction < 2 { 0 } else { w - 1 })
                .take(w)
                .collect_vec();
            match direction {
                0 => {
                    for r in 0..w {
                        for c in 0..w {
                            match data[(r * w) + c] {
                                '#' => {
                                    if let Some(b) = bumps.get_mut(c) {
                                        *b = r + 1;
                                    }
                                }
                                'O' => {
                                    if let Some(b) = bumps.get_mut(c) {
                                        if *b != r {
                                            if let Some(x) = data.get_mut((*b * w) + c) {
                                                assert_eq!(*x, '.', "at {r} {c} {}", *b);
                                                *x = 'O';
                                            }
                                            if let Some(x) = data.get_mut((r * w) + c) {
                                                *x = '.';
                                            }
                                        }
                                        *b += 1;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                1 => {
                    for c in 0..w {
                        for r in 0..w {
                            match data[(r * w) + c] {
                                '#' => {
                                    if let Some(b) = bumps.get_mut(r) {
                                        *b = c + 1;
                                    }
                                }
                                'O' => {
                                    if let Some(b) = bumps.get_mut(r) {
                                        if *b != c {
                                            if let Some(x) = data.get_mut((r * w) + *b) {
                                                assert_eq!(*x, '.', "at {r} {c} {}", *b);
                                                *x = 'O';
                                            }
                                            if let Some(x) = data.get_mut((r * w) + c) {
                                                *x = '.';
                                            }
                                            *b += 1;
                                        } else {
                                            *b = c + 1;
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                2 => {
                    for r in (0..w).rev() {
                        for c in 0..w {
                            match data[(r * w) + c] {
                                '#' => {
                                    if let Some(b) = bumps.get_mut(c) {
                                        *b = r.max(1) - 1;
                                    }
                                }
                                'O' => {
                                    if let Some(b) = bumps.get_mut(c) {
                                        if *b != r {
                                            if let Some(x) = data.get_mut((*b * w) + c) {
                                                assert_eq!(*x, '.', "at {r} {c} {}", *b);
                                                *x = 'O';
                                            }
                                            if let Some(x) = data.get_mut((r * w) + c) {
                                                *x = '.';
                                            }
                                        }
                                        *b = (*b).max(1) - 1;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                3 => {
                    for c in (0..w).rev() {
                        for r in 0..w {
                            match data[(r * w) + c] {
                                '#' => {
                                    if let Some(b) = bumps.get_mut(r) {
                                        *b = c.max(1) - 1;
                                    }
                                }
                                'O' => {
                                    if let Some(b) = bumps.get_mut(r) {
                                        if *b != c {
                                            if let Some(x) = data.get_mut((r * w) + *b) {
                                                assert_eq!(*x, '.', "at {r} {c} {}", *b);
                                                *x = 'O';
                                            }
                                            if let Some(x) = data.get_mut((r * w) + c) {
                                                *x = '.';
                                            }
                                            *b = (*b).max(1) - 1;
                                        } else {
                                            *b = c.max(1) - 1;
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        /*
                for r in 0..w {
                    println!(
                        "{}",
                        data.get((r * w)..(r + 1) * w).unwrap().iter().join("")
                    )
                }
                println!();
        */
        let weight = (0..w)
        .map(|r| {
            (0..w)
                .map(|c| if data[(r * w) + c] == 'O' { w - r } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();

        println!("{weight}");
        /*
        if cycle % 100000 == 0 && cycle > 1 {
            let elapsed_time = now.elapsed();
            println!("{cycle} elapsed: {}s {weight}", elapsed_time.as_secs());
        }
        */
    }
    (0..w)
        .map(|r| {
            (0..w)
                .map(|c| if data[(r * w) + c] == 'O' { w - r } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn solution_a(input: &str) -> Option<usize> {
    let clean = input.lines().map(|line| line.trim()).join("\n");
    Some(solve_a(&clean))
}

fn solution_b(input: &str) -> Option<usize> {
    let clean = input.lines().map(|line| line.trim()).join("\n");
    Some(solve_b(&clean, 1000000000))
}

fn main() {
    let c = content().unwrap();

    // let a = solution_a(&c);
    // let b = solution_b(&c);

    // println!("Step A: {:?}", a);
    // println!("Step B: {:?}", b);

    println!("Count: {}", c.chars().filter(|x| *x == 'O').count());
    let b = solve_b(&c, 1000000);
    println!("Step B: {}", b);
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
        assert_eq!(solution_a(&data), Some(136));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(64));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(109638));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(102657));
    }
}
