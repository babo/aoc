use itertools::Itertools;
use std::fs::read_to_string;
use std::iter;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_a(data: &str, d: char, s: (usize, usize), w: usize, h: usize) -> usize {
    let mut d = d;
    let (mut x, mut y) = s;
    let mut n = 0;
    let at = |a, b| data.chars().nth(b * w + a).unwrap();

    println!("{d}");
    while n < 60000 {
        println!("{n} {x} {y}: {} => {d}", at(x, y));

        match at(x, y) {
            '-' => match d {
                'E' => x += 1,
                'W' => x -= 1,
                _ => break,
            },
            '|' => match d {
                'S' => y += 1,
                'N' => y -= 1,
                _ => break,
            },
            'L' => match d {
                'S' => {
                    x += 1;
                    d = 'E';
                }
                'W' => {
                    y -= 1;
                    d = 'N';
                }
                _ => break,
            },
            'J' => match d {
                'S' => {
                    x -= 1;
                    d = 'W';
                }
                'E' => {
                    y -= 1;
                    d = 'N';
                }
                _ => break,
            },
            '7' => match d {
                'E' => {
                    y += 1;
                    d = 'S';
                }
                'N' => {
                    x -= 1;
                    d = 'W';
                }
                _ => break,
            },
            'F' => match d {
                'W' => {
                    y += 1;
                    d = 'S';
                }
                'N' => {
                    x += 1;
                    d = 'E';
                }
                _ => break,
            },
            '.' => break,
            'S' => {
                if n != 0 {
                    break;
                }
                match d {
                    'E' => {
                        if (x + 1) == w {
                            break;
                        } else {
                            x += 1;
                            match at(x, y) {
                                '|' => break,
                                'L' => break,
                                'F' => break,
                                _ => (),
                            }
                        }
                    }
                    'W' => {
                        if x == 0 {
                            break;
                        } else {
                            x -= 1;
                            match at(x, y) {
                                '|' => break,
                                'J' => break,
                                '7' => break,
                                _ => (),
                            }
                        }
                    }
                    'N' => {
                        if y == 0 {
                            break;
                        } else {
                            y -= 1;
                            match at(x, y) {
                                '-' => break,
                                'L' => break,
                                'J' => break,
                                _ => (),
                            }
                        }
                    }
                    'S' => {
                        if (y + 1) == h {
                            break;
                        } else {
                            y += 1;
                            match at(x, y) {
                                '-' => break,
                                '7' => break,
                                'F' => break,
                                _ => (),
                            }
                        }
                    }
                    _ => unimplemented!("What?"),
                }
            }
            _ => unimplemented!("Invalid character"),
        }
        n += 1;
    }
    if at(x, y) == 'S' {
        n / 2
    } else {
        0
    }
}

fn solve_b(data: &str, d: char, s: (usize, usize), w: usize, h: usize) -> usize {
    let mut boundary: Vec<bool> = Vec::from_iter(iter::repeat(false).take(data.len()));
    let mut d = d;
    let (mut x, mut y) = s;
    let mut n = 0;
    let pos = |a, b| b * w + a;
    let at = |a, b| data.chars().nth(pos(a, b)).unwrap();

    println!("{d}");
    while n < 60000 {
        boundary.get_mut(pos(x, y)).map(|c| *c = true);

        match at(x, y) {
            '-' => match d {
                'E' => x += 1,
                'W' => x -= 1,
                _ => break,
            },
            '|' => match d {
                'S' => y += 1,
                'N' => y -= 1,
                _ => break,
            },
            'L' => match d {
                'S' => {
                    x += 1;
                    d = 'E';
                }
                'W' => {
                    y -= 1;
                    d = 'N';
                }
                _ => break,
            },
            'J' => match d {
                'S' => {
                    x -= 1;
                    d = 'W';
                }
                'E' => {
                    y -= 1;
                    d = 'N';
                }
                _ => break,
            },
            '7' => match d {
                'E' => {
                    y += 1;
                    d = 'S';
                }
                'N' => {
                    x -= 1;
                    d = 'W';
                }
                _ => break,
            },
            'F' => match d {
                'W' => {
                    y += 1;
                    d = 'S';
                }
                'N' => {
                    x += 1;
                    d = 'E';
                }
                _ => break,
            },
            '.' => break,
            'S' => {
                if n != 0 {
                    break;
                }
                match d {
                    'E' => {
                        if (x + 1) == w {
                            break;
                        } else {
                            x += 1;
                            match at(x, y) {
                                '|' => break,
                                'L' => break,
                                'F' => break,
                                _ => (),
                            }
                        }
                    }
                    'W' => {
                        if x == 0 {
                            break;
                        } else {
                            x -= 1;
                            match at(x, y) {
                                '|' => break,
                                'J' => break,
                                '7' => break,
                                _ => (),
                            }
                        }
                    }
                    'N' => {
                        if y == 0 {
                            break;
                        } else {
                            y -= 1;
                            match at(x, y) {
                                '-' => break,
                                'L' => break,
                                'J' => break,
                                _ => (),
                            }
                        }
                    }
                    'S' => {
                        if (y + 1) == h {
                            break;
                        } else {
                            y += 1;
                            match at(x, y) {
                                '-' => break,
                                '7' => break,
                                'F' => break,
                                _ => (),
                            }
                        }
                    }
                    _ => unimplemented!("What?"),
                }
            }
            _ => unimplemented!("Invalid character"),
        }
        n += 1;
    }
    if at(x, y) == 'S' {
        n = 0;
        for j in 0..h {
            let mut out = true;
            let mut corner: Option<char> = None;
            for i in 0..w {
                print!(
                    "{}",
                    if boundary.get(pos(i, j)) == Some(&true) {
                        at(i, j)
                    } else if !out && at(i, j) == '.' {
                        'X'
                    } else {
                        '.'
                    }
                );
                if boundary.get(pos(i, j)) == Some(&true) {
                    match at(i, j) {
                        '|' => {
                            out = !out;
                            corner = None;
                        }
                        'L' => {
                            corner = Some('L');
                        }
                        'J' => {
                            if corner == Some('F') {
                                out = !out;
                            }
                            corner = Some('J');
                        }
                        '7' => {
                            if corner == Some('L') {
                                out = !out;
                            }
                            corner = Some('7');
                        }
                        'F' => {
                            corner = Some('F');
                        }
                        _ => (),
                    }
                } else if !out {
                    n += 1;
                };
            }
            println!();
        }
        n
    } else {
        0
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let h = input.lines().count();
    let w = input.lines().next().map(|x| x.len()).unwrap();
    println!("{input}");
    let data = input.chars().filter(|x| !x.is_whitespace()).join("");
    let s = data.chars().position(|x| x == 'S').unwrap();
    let x = s % w;
    let y = s / w;

    "NWSE"
        .chars()
        .map(|d| solve_a(&data, d, (x, y), w, h))
        .max()
}

fn solution_b(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let h = input.lines().count();
    let w = input.lines().next().map(|x| x.len()).unwrap();
    println!("{input}");
    let data = input.chars().filter(|x| !x.is_whitespace()).join("");
    let s = data.chars().position(|x| x == 'S').unwrap();
    let x = s % w;
    let y = s / w;

    "NWSE"
        .chars()
        .map(|d| solve_b(&data, d, (x, y), w, h))
        .find(|x| *x != 0)
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

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_simple_a() {
        let data = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        assert_eq!(solution_a(&data), Some(4));
    }

    #[test]
    fn test_simple_b() {
        let data = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";
        assert_eq!(solution_a(&data), Some(8));
    }

    #[test]
    fn test_simple_c() {
        let data = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        assert_eq!(solution_b(&data), Some(4));
    }

    #[test]
    fn test_simple_d() {
        let data = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";
        assert_eq!(solution_b(&data), Some(8));
    }

    #[test]
    fn test_simple_e() {
        let data = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solution_b(&data), Some(10));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(6838));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(451));
    }
}
