use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Fwd(u32),
    Left,
    Right,
}

struct Maze {
    rows: usize,
    cols: usize,
    tile: usize,
    board: Vec<u8>,
    instruction: Vec<Step>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let dlines = input
            .lines()
            .filter(|x| x.contains('.'))
            .map(|x| x.trim_end())
            .collect_vec();
        let cols = dlines.iter().map(|x| x.len()).max().unwrap();
        let rows = dlines.len();
        let mut board = Vec::new();
        dlines.iter().for_each(|line| {
            line.bytes().for_each(|c| board.push(c));
            (0..(cols - line.len())).for_each(|_| board.push(b' '));
        });

        let code = input.lines().find(|x| x.contains('L')).unwrap();
        let mut instruction = Vec::new();
        code.chars()
            .fold(None, |prev, curr| match curr {
                'L' => {
                    prev.map(|n| instruction.push(Step::Fwd(n)));
                    instruction.push(Step::Left);
                    None
                }
                'R' => {
                    prev.map(|n| instruction.push(Step::Fwd(n)));
                    instruction.push(Step::Right);
                    None
                }
                n if n.is_ascii_alphanumeric() => match prev {
                    None => n.to_digit(10),
                    Some(p) => n.to_digit(10).map(|nd| nd + p * 10),
                },
                _ => prev,
            })
            .map(|n| instruction.push(Step::Fwd(n)));

        Maze {
            board,
            instruction,
            rows,
            cols,
            tile: cols / 4,
        }
    }

    fn at(&self, row: usize, col: usize) -> u8 {
        self.board[col + row * self.cols]
    }

    fn forward(&self, row: usize, col: usize, steps: usize, heading: u8) -> (usize, usize) {
        let start = if steps != 0 { 1 } else { 0 };
        let mut nr = row as i32;
        let mut nc = col as i32;
        match heading {
            0 => {
                for c in start..self.cols as i32 {
                    let cc = (nc + c) % self.cols as i32;
                    if self.at(row, cc as usize) != b' ' {
                        nc = cc;
                        break;
                    }
                }
            }
            1 => {
                for r in start..self.rows as i32 {
                    let rr = (nr + r) % self.rows as i32;
                    if self.at(rr as usize, col) != b' ' {
                        nr = rr;
                        break;
                    }
                }
            }
            2 => {
                for c in start..self.cols as i32 {
                    let cc = (self.cols as i32 + nc - c) % self.cols as i32;
                    if self.at(row, cc as usize) != b' ' {
                        nc = cc;
                        break;
                    }
                }
            }
            3 => {
                for r in start..self.rows as i32 {
                    let rr = (self.rows as i32 + nr - r) % self.rows as i32;
                    if self.at(rr as usize, col) != b' ' {
                        nr = rr;
                        break;
                    }
                }
            }
            d => unreachable!("What a direction! {d}"),
        }
        let (nr, nc) = (nr as usize, nc as usize);
        match self.at(nr, nc) {
            b'#' => (row, col),
            b'.' => {
                if steps > 0 {
                    self.forward(nr, nc, steps - 1, heading)
                } else {
                    (nr, nc)
                }
            }
            c => unreachable!("No way! {nr} {nc} {steps} {heading} |{}|", c as char),
        }
    }

    fn around(&self, row: usize, col: usize, steps: usize, heading: u8) -> (usize, usize, u8) {
        let start = if steps != 0 { 1 } else { 0 };
        let mut nr = row;
        let mut nc = col;
        let mut nh = heading;
        let face = if row < self.tile {
            1
        } else if row < 2 * self.tile {
            2 + col / self.tile
        } else {
            3 + col / self.tile
        };
        match heading {
            0 => {
                let cc = col + start;
                match face {
                    1 => {
                        if cc >= 3 * self.tile {
                            nr = 3 * self.tile - row;
                            nc = 3 * self.tile;
                            nh = 2;
                        } else {
                            nc = cc;
                        }
                    }
                    4 => {
                        if cc >= 3 * self.tile {
                            nr = 2 * self.tile;
                            nc = 5 * self.tile - row - 1;
                            nh = 1
                        } else {
                            nc = cc;
                        }
                    }
                    6 => {
                        if cc >= 4 * self.tile {
                            nr = 3 * self.tile - row;
                            nc = 3 * self.tile;
                            nh = 2;
                        } else {
                            nc = cc;
                        }
                    }
                    _ => nc = cc,
                }
            }
            1 => {
                let rr = row + start;
                match face {
                    1 => nr = rr,
                    4 => nr = rr,
                    2 => {
                        if rr >= 2 * self.tile {
                            nr = 3 * self.tile;
                            nc = 3 * self.tile - col;
                            nh = 3;
                        } else {
                            nr = rr;
                        }
                    }
                    3 => {
                        if rr >= 2 * self.tile {
                            nc = 2 * self.tile;
                            nr = 1 * self.tile - col;
                            nh = 1;
                        } else {
                            nr = rr;
                        }
                    }
                    5 => {
                        if rr >= 3 * self.tile {
                            nr = 2 * self.tile - 1;
                            nc = self.tile - (col - 2 * self.tile) - 1;
                            nh = 3;
                        } else {
                            nr = rr;
                        }
                    }
                    6 => {
                        if rr >= 3 * self.tile {
                            nc = 0;
                            nr = 2 * self.tile - (col - 3 * self.tile);
                            nh = 1;
                        } else {
                            nr = rr;
                        }
                    }
                    _ => unreachable!("Opps"),
                }
            }
            2 => match face {
                1 => {
                    let cc = col - start;
                    if cc <= 2 * self.tile {
                        nr = self.tile;
                        nc = self.tile + row;
                        nh = 1;
                    } else {
                        nc = cc;
                    }
                }
                2 => {
                    if col == 0 {
                        nr = 3 * self.tile;
                        nc = 4 * self.tile - (row - self.tile);
                        nh = 3;
                    } else {
                        nc -= start;
                    }
                }
                5 => {
                    if col <= 2 * self.tile {
                        nr = 2 * self.tile;
                        nc = self.tile + 3 * self.tile - row;
                        nh = 3;
                    } else {
                        nc -= start;
                    }
                }
                _ => nc -= start,
            },
            3 => match face {
                1 => {
                    if row == 0 {
                        nr = self.tile;
                        nc = self.tile - (col - 2 * self.tile);
                        nh = 2;
                    } else {
                        nr -= start;
                    }
                }
                2 => {
                    if row <= self.tile {
                        nr = 0;
                        nc = 3 * self.tile - col;
                        nh = 1;
                    } else {
                        nr -= start;
                    }
                }
                3 => {
                    println!("Y {row} {}", self.tile);
                    if row <= self.tile {
                        nc = 2 * self.tile;
                        nr = col - self.tile;
                        nh = 0;
                    } else {
                        nr -= start;
                    }
                }
                6 => {
                    if row <= 2 * self.tile {
                        nc = 3 * self.tile;
                        nr = 2 * self.tile - (col - 3 * self.tile);
                        nh = 2;
                    } else {
                        nr -= start;
                    }
                }
                _ => nr -= start,
            },
            d => unreachable!("What a direction! {d}"),
        }
        let (nr, nc) = (nr as usize, nc as usize);
        println!(
            "{:2}->{:2} {:2}->{:2} steps: {:2} heading: {heading} {nh} face: {face} {}",
            row, nr,
            col, nc,
            steps,
            self.at(nr, nc) as char
        );

        match self.at(nr, nc) {
            b'#' => (row, col, heading),
            b'.' => {
                if steps > 1 {
                    self.around(nr, nc, steps - 1, nh)
                } else {
                    (nr, nc, nh)
                }
            }
            c => unreachable!("No way! {nr} {nc} {steps} {nh} |{}|", c as char),
        }
    }

    fn walk(&self) -> (usize, usize, u8) {
        let (r, c) = self.forward(0, 0, 0, 0);
        self.instruction.iter().fold((r, c, 0), |prev, next| {
            let (mut r, mut c, mut heading) = prev;
            match next {
                Step::Left => heading = if heading == 0 { 3 } else { heading - 1 },
                Step::Right => heading = (heading + 1) % 4,
                Step::Fwd(steps) => (r, c) = self.forward(r, c, *steps as usize, heading),
            };
            (r, c, heading)
        })
    }

    fn cube(&self) -> (usize, usize, u8) {
        let (r, c) = (0, self.tile * 2);
        self.instruction.iter().fold((r, c, 0), |prev, next| {
            let (mut r, mut c, mut heading) = prev;
            match next {
                Step::Left => heading = if heading == 0 { 3 } else { heading - 1 },
                Step::Right => heading = (heading + 1) % 4,
                Step::Fwd(steps) => (r, c, heading) = self.around(r, c, *steps as usize, heading),
            };
            println!();
            (r, c, heading)
        })
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let m = Maze::new(input);
    let (r, c, f) = m.walk();
    Some((r + 1) * 1000 + 4 * (c + 1) + f as usize)
}

fn solution_b(input: &str) -> Option<usize> {
    let m = Maze::new(input);
    let (r, c, f) = m.cube();
    Some((r + 1) * 1000 + 4 * (c + 1) + f as usize)
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
        assert_eq!(solution_a(&data), Some(6032));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(5031));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(27492));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
