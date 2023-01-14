use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Fwd(usize),
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
        let tile = cols.max(rows) / 4;
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
                    None => n.to_digit(10).map(|nd| nd as usize),
                    Some(p) => n.to_digit(10).map(|nd| nd as usize + p * 10),
                },
                _ => prev,
            })
            .map(|n| instruction.push(Step::Fwd(n)));

        Maze {
            board,
            instruction,
            rows,
            cols,
            tile,
        }
    }

    fn normalize(&self) -> Self {
        if self.rows < self.cols {
            Maze {
                rows: self.rows,
                cols: self.cols,
                tile: self.tile,
                board: self.board.clone(),
                instruction: self.instruction.clone(),
            }
        } else {
            let (cols, rows, tile) = (self.rows, self.cols, self.tile);
            let mut board = vec![b' '; tile * tile * 12];
            // 1
            for c in tile..2 * tile {
                for r in 0..tile {
                    board[r * cols + c + tile] = self.board[r * self.cols + c];
                }
            }
            // 6
            for c in 2 * tile..3 * tile {
                for r in 0..tile {
                    let rr = tile - 1 - r + 2 * tile;
                    let cc = 3 * tile - 1 - c + 3 * tile;
                    board[rr * cols + cc] = self.board[r * self.cols + c];
                }
            }
            // 2, 3
            for c in 0..tile {
                for r in 2 * tile..4 * tile {
                    let rr = tile + c;
                    let cc = 4 * tile - 1 - r;
                    board[rr * cols + cc] = self.board[r * self.cols + c];
                }
            }
            // 4, 5
            for c in tile..2 * tile {
                for r in tile..3 * tile {
                    let rr = r;
                    let cc = tile + c;
                    board[rr * cols + cc] = self.board[r * self.cols + c];
                }
            }
            Maze {
                cols,
                rows,
                board,
                tile,
                instruction: self.instruction.clone(),
            }
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

    fn face(&self, row: usize, col: usize) -> usize {
        if self.cols > self.rows {
            if row < self.tile {
                1
            } else if row < 2 * self.tile {
                2 + col / self.tile
            } else {
                3 + col / self.tile
            }
        } else {
            if row < self.tile && col <= self.tile && col < 2 * self.tile {
                1
            } else if row < self.tile && col <= 2 * self.tile {
                6
            } else if row > 3 * self.tile && col < self.tile {
                2
            } else if row < 2 * self.tile && col <= 2 * self.tile {
                4
            } else if row < 3 * self.tile && col < self.tile {
                3
            } else if row < 3 * self.tile && col < 2 * self.tile {
                5
            } else {
                unreachable!("Hello");
            }
        }
    }

    fn step(&self, row: usize, col: usize, heading: u8) -> (usize, usize, u8) {
        let mut nr = row;
        let mut nc = col;
        let mut nh = heading;
        let face = self.face(row, col);
        match heading {
            0 => match face {
                1 => {
                    if col >= 3 * self.tile - 1 {
                        nr = 3 * self.tile - row - 1;
                        nc = 4 * self.tile - 1;
                        nh = 2;
                    } else {
                        nc += 1;
                    }
                }
                4 => {
                    if col >= 3 * self.tile - 1 {
                        nr = 2 * self.tile;
                        nc = 5 * self.tile - row - 1;
                        nh = 1
                    } else {
                        nc += 1;
                    }
                }
                6 => {
                    if col >= 4 * self.tile - 1 {
                        nr = 3 * self.tile - row - 1;
                        nc = 3 * self.tile - 1;
                        nh = 2;
                    } else {
                        nc += 1;
                    }
                }
                _ => nc += 1,
            },
            1 => match face {
                2 => {
                    if row >= 2 * self.tile - 1 {
                        nr = 3 * self.tile - 1;
                        nc = 3 * self.tile - col - 1;
                        nh = 3;
                    } else {
                        nr += 1;
                    }
                }
                3 => {
                    if row >= 2 * self.tile - 1 {
                        nc = 2 * self.tile;
                        nr = 4 * self.tile - col - 1;
                        nh = 0;
                    } else {
                        nr += 1;
                    }
                }
                5 => {
                    if row >= 3 * self.tile - 1 {
                        nr = 2 * self.tile - 1;
                        nc = 3 * self.tile - col - 1;
                        nh = 3;
                    } else {
                        nr += 1;
                    }
                }
                6 => {
                    if row >= 3 * self.tile - 1 {
                        nc = 0;
                        nr = 5 * self.tile - col - 1;
                        nh = 0;
                    } else {
                        nr += 1;
                    }
                }
                _ => nr += 1,
            },
            2 => match face {
                1 => {
                    if col <= 2 * self.tile {
                        nr = self.tile;
                        nc = self.tile + row;
                        nh = 1;
                    } else {
                        nc -= 1;
                    }
                }
                2 => {
                    if col == 0 {
                        nr = 3 * self.tile - 1;
                        nc = 5 * self.tile - row - 1;
                        nh = 3;
                    } else {
                        nc -= 1;
                    }
                }
                5 => {
                    if col <= 2 * self.tile {
                        nr = 2 * self.tile - 1;
                        nc = 4 * self.tile - row - 1;
                        nh = 3;
                    } else {
                        nc -= 1;
                    }
                }
                _ => nc -= 1,
            },
            3 => match face {
                1 => {
                    if row == 0 {
                        nr = self.tile;
                        nc = 3 * self.tile - col - 1;
                        nh = 1;
                    } else {
                        nr -= 1;
                    }
                }
                2 => {
                    if row <= self.tile {
                        nr = 0;
                        nc = 3 * self.tile - col - 1;
                        nh = 1;
                    } else {
                        nr -= 1;
                    }
                }
                3 => {
                    if row <= self.tile {
                        nc = 2 * self.tile;
                        nr = col - self.tile;
                        nh = 0;
                    } else {
                        nr -= 1;
                    }
                }
                6 => {
                    if row <= 2 * self.tile {
                        nc = 3 * self.tile - 1;
                        nr = 5 * self.tile - col - 1;
                        nh = 2;
                    } else {
                        nr -= 1;
                    }
                }
                _ => nr -= 1,
            },
            d => unreachable!("What a direction! {d}"),
        }
        (nr, nc, nh)
    }

    fn around(&self, row: usize, col: usize, steps: usize, heading: u8) -> (usize, usize, u8) {
        let (nr, nc, nh) = self.step(row, col, heading);

        match self.at(nr, nc) {
            b'#' => (row, col, heading),
            b'.' => {
                if steps > 0 {
                    println!("{nr} {nc} {heading}");
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
            let (r, c, heading) = prev;
            match next {
                Step::Right => (r, c, (heading + 1) % 4),
                Step::Left => (r, c, if heading == 0 { 3 } else { heading - 1 }),
                Step::Fwd(steps) => {
                    let (r, c) = self.forward(r, c, *steps, heading);
                    (r, c, heading)
                }
            }
        })
    }

    fn cube(&self) -> (usize, usize, u8) {
        let (r, c) = (0, self.tile * 2);
        self.instruction.iter().fold((r, c, 0), |prev, next| {
            let (r, c, heading) = prev;
            println!("{r} {c} {heading}");

            match next {
                Step::Right => (r, c, (heading + 1) % 4),
                Step::Left => (r, c, if heading == 0 { 3 } else { heading - 1 }),
                Step::Fwd(steps) => self.around(r, c, *steps - 1, heading),
            }
        })
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let m = Maze::new(input);
    let (r, c, f) = m.walk();
    Some((r + 1) * 1000 + 4 * (c + 1) + f as usize)
}

fn solution_b(input: &str) -> Option<usize> {
    let o = Maze::new(input);
    let m = o.normalize();

    let (r, c, f) = m.cube();
    println!("{r} {c} {f}");
    if o.rows < o.cols {
        Some((r + 1) * 1000 + 4 * (c + 1) + f as usize)
    } else {
        println!("{r} {c} {f} {}", m.tile);
        let (rr, cc, ff) = (r, c - m.tile, f as usize);
        Some((rr + 1) * 1000 + 4 * (cc + 1) + ff)
    }
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

    fn setup_mini() -> Maze {
        let board = (0..(12 * 16))
            .map(|i| {
                let r = i / 16;
                let c = i % 16;
                match (r / 4, c / 4) {
                    (0, 2) => b'.',
                    (1, 0) => b'.',
                    (1, 1) => b'.',
                    (1, 2) => b'.',
                    (2, 2) => b'.',
                    (2, 3) => b'.',
                    _ => b' ',
                }
            })
            .collect_vec();
        let instruction = vec![Step::Fwd(1); 1];
        Maze {
            rows: 12,
            cols: 16,
            tile: 4,
            board,
            instruction,
        }
    }

    #[test]
    fn test_steps() {
        let m = setup_mini();

        assert_eq!(m.step(0, 11, 0), (11, 15, 2));
        assert_eq!(m.step(3, 11, 0), (8, 15, 2));
        assert_eq!(m.step(4, 11, 0), (8, 15, 1));
        assert_eq!(m.step(7, 11, 0), (8, 12, 1));
        assert_eq!(m.step(8, 15, 0), (3, 11, 2));
        assert_eq!(m.step(11, 15, 0), (0, 11, 2));

        assert_eq!(m.step(7, 0, 1), (11, 11, 3));
        assert_eq!(m.step(7, 3, 1), (11, 8, 3));
        assert_eq!(m.step(7, 4, 1), (11, 8, 0));
        assert_eq!(m.step(7, 7, 1), (8, 8, 0));
        assert_eq!(m.step(11, 8, 1), (7, 3, 3));
        assert_eq!(m.step(11, 11, 1), (7, 0, 3));
        assert_eq!(m.step(11, 12, 1), (7, 0, 0));
        assert_eq!(m.step(11, 15, 1), (4, 0, 0));

        assert_eq!(m.step(0, 8, 2), (4, 4, 1));
        assert_eq!(m.step(3, 8, 2), (4, 7, 1));
        assert_eq!(m.step(4, 0, 2), (11, 15, 3));
        assert_eq!(m.step(7, 0, 2), (11, 12, 3));
        assert_eq!(m.step(8, 8, 2), (7, 7, 3));
        assert_eq!(m.step(11, 8, 2), (7, 4, 3));

        assert_eq!(m.step(4, 0, 3), (0, 11, 1));
        assert_eq!(m.step(4, 3, 3), (0, 8, 1));
        assert_eq!(m.step(4, 4, 3), (0, 8, 0));
        assert_eq!(m.step(4, 7, 3), (3, 8, 0));
        assert_eq!(m.step(0, 8, 3), (4, 3, 1));
        assert_eq!(m.step(0, 11, 3), (4, 0, 1));
        assert_eq!(m.step(8, 12, 3), (7, 11, 2));
        assert_eq!(m.step(8, 15, 3), (4, 11, 2));
    }

    #[test]
    fn test_faces() {
        let m = setup_mini();

        assert_eq!(m.face(8, 15), 6);
        assert_eq!(m.face(0, 11), 1);
        assert_eq!(m.face(0, 8), 1);
        assert_eq!(m.face(3, 8), 1);
        assert_eq!(m.face(3, 11), 1);

        assert_eq!(m.face(4, 0), 2);
        assert_eq!(m.face(4, 3), 2);
        assert_eq!(m.face(7, 0), 2);
        assert_eq!(m.face(7, 3), 2);

        assert_eq!(m.face(4, 4), 3);
        assert_eq!(m.face(4, 7), 3);
        assert_eq!(m.face(7, 4), 3);
        assert_eq!(m.face(7, 7), 3);

        assert_eq!(m.face(4, 8), 4);
        assert_eq!(m.face(4, 11), 4);
        assert_eq!(m.face(7, 8), 4);
        assert_eq!(m.face(7, 11), 4);

        assert_eq!(m.face(8, 8), 5);
        assert_eq!(m.face(8, 11), 5);
        assert_eq!(m.face(11, 8), 5);
        assert_eq!(m.face(11, 11), 5);

        assert_eq!(m.face(8, 12), 6);
        assert_eq!(m.face(8, 15), 6);
        assert_eq!(m.face(11, 12), 6);
        assert_eq!(m.face(11, 15), 6);
    }
}
