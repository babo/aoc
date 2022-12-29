use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

/*
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##

*/

struct Tetris {
    chamber: Vec<u8>,
    height: usize,
}

impl Tetris {
    const CHUNKS: usize = 10000;
    const SHAPES: [u16; 5] = [
        0b01111,
        0b01001110010,
        0b010001000111,
        0b01000100010001,
        0b0110011,
    ];

    fn new() -> Self {
        Tetris {
            chamber: Vec::new(),
            height: 0,
        }
    }

    fn shape_line(kind: usize, row: usize) -> u8 {
        assert!(row < 4);
        ((Tetris::SHAPES[kind] >> (row << 2usize)) & 0b1111) as u8
    }

    fn freeze(&mut self, kind: usize, xy: (i32, i32)) {
        let (orig_x, orig_y) = xy;
        //println!("Freeze at {orig_y}");
        for dy in 0..4 {
            let y = orig_y + dy;
            let m = y.abs() as usize;
            let line = Self::shape_line(kind, dy as usize) << orig_x;
            if line == 0 {
                break;
            }
            if y >= 0 || self.chamber.is_empty() {
                self.chamber.push(line as u8);
            } else {
                let p = self.chamber.len() - 1 - (m - 1);
                //println!("Get {p} with {m} where len is {}", self.chamber.len());
                self.chamber.get_mut(p).map(|orig| *orig |= line as u8);
            }
        }
        if self.chamber.len() > Self::CHUNKS + 200 {
            let mut saved = self
                .chamber
                .iter()
                .skip(Self::CHUNKS)
                .map(|c| *c)
                .collect_vec();
            self.height += Self::CHUNKS;
            self.chamber.clear();
            self.chamber.append(&mut saved);
        }
    }

    fn step(&self, dir: Direction, kind: usize, xy: (i32, i32)) -> Option<(i32, i32)> {
        let (orig_x, orig_y) = xy;
        let (dx, dy) = match dir {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        };
        if orig_x + dx < 0 {
            return None;
        }
        let x = orig_x + dx;
        for py in 0i32..4 {
            let line = Self::shape_line(kind, py as usize);
            if (line << x) | 127 != 127 {
                return None;
            }

            let y = orig_y + py + dy;
            if y >= 0 {
                continue;
            }
            let m = y.abs() as usize;
            if m > self.chamber.len() {
                return None;
            }
            let row = self.chamber[self.chamber.len() - m];
            let scan = line << x;
            if row & scan != 0 {
                return None;
            }
        }
        Some((dx, dy))
    }

    fn _scene(&self, shape: Option<(usize, (i32, i32))>) {
        let pr_shape = |shp: u8, line: u8| {
            print!("|");
            for x in 0..7 {
                let mask = 1 << x;
                let c = if shp & mask != 0 {
                    '@'
                } else if line & mask != 0 {
                    '#'
                } else {
                    '.'
                };
                print!("{c}")
            }
            println!("|");
        };
        let get_shape = |row| match shape {
            Some((kind, (x, y))) => {
                if row >= y && row < y + 4 {
                    let py = row - y;
                    Self::shape_line(kind, py as usize) << x
                } else {
                    0
                }
            }
            None => 0,
        };

        for row in (0..8).rev() {
            let shp = get_shape(row);
            pr_shape(shp, 0);
        }
        for row in 0..self.chamber.len().min(10) {
            let line = self.chamber[self.chamber.len() - 1 - row];
            let shp = get_shape(-1 - (row as i32));
            pr_shape(shp, line);
        }
        println!("+-------+\n");
    }
}

fn simulate(input: &str, steps: usize) -> usize {
    let mut jet = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .nth(0)
        .unwrap()
        .chars()
        .cycle();
    let mut rock = (0..5).cycle();

    let mut tetris = Tetris::new();

    for _cycle in 0..steps {
        let kind = rock.next().unwrap();
        let mut orig_x: i32 = 2;
        let mut orig_y: i32 = 3;

        loop {
            let jet = if jet.next() == Some('<') {
                Direction::Left
            } else {
                Direction::Right
            };

            //tetris.scene(Some((kind, (orig_x, orig_y))));

            match tetris.step(jet, kind, (orig_x, orig_y)) {
                Some((dx, dy)) => {
                    orig_x += dx;
                    orig_y += dy;
                }
                None => (),
            }
            match tetris.step(Direction::Down, kind, (orig_x, orig_y)) {
                Some((_, dy)) => orig_y += dy,
                None => break,
            }
            //tetris.scene(Some((kind, (orig_x, orig_y))));
        }
        //tetris.scene(Some((kind, (orig_x, orig_y))));
        tetris.freeze(kind, (orig_x, orig_y));
        //tetris.scene(None);
    }
    tetris.height + tetris.chamber.len()
}

fn solution_a(input: &str) -> usize {
    simulate(input, 2022)
}

fn solution_b(input: &str) -> usize {
    let n = 1000000000000;
    let w = (n - 320) / 280;
    let ww = (n - 320) % 280;

    let a = simulate(input, 320);
    let b = simulate(input, 600);
    let c = simulate(input, 600 + ww);

    a + w * (b - a) + c - b
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
        assert_eq!(solution_a(&data), 3068);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 1514285714288);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 3171);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 0);
    }

    #[test]
    fn test_rem() {
        let data = simple().unwrap();
        let input = &data;
        let n = 19370;
        let normal = simulate(input, n);

        let w = (n - 320) / 280;
        let ww = (n - 320) % 280;

        let a = simulate(input, 320);
        let b = simulate(input, 600);
        let c = simulate(input, 600 + ww);

        println!("{a} {b} {c}");
        println!("{w} {ww}");
        assert_eq!(normal, a + (w - 0) * (b - a) + c - b);
    }
}
