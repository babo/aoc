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
    const PART_2: usize = 1000000000000usize;
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

use std::iter::Cycle;
use std::ops::Range;
use std::str::Chars;

struct Jet<'a> {
    jet: Cycle<Chars<'a>>,
    rock: Cycle<Range<usize>>,
    num_actions: usize,
}

impl<'a> Jet<'a> {
    fn new(input: &'a str) -> Self {
        let jet = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .nth(0)
            .unwrap()
            .chars()
            .cycle();
        let rock = (0usize..5).cycle();
        let num_actions = input.chars().filter(|c| *c == '<' || *c == '>').count();

        Jet {
            jet,
            rock,
            num_actions,
        }
    }
}

fn cyclist(input: &str, steps: usize, total: usize, grmax: usize) -> usize {
    let mut env = Jet::new(input);
    let mut tetris = Tetris::new();
    let mut i = 0usize;

    let mut prev_h = 0;
    let mut prev_r = 0;
    let mut growth = Vec::new();
    let mut enough = steps;
    for num_rocks in 0..steps {
        if num_rocks >= enough {
            break;
        }
        let kind = env.rock.next().unwrap();
        let mut orig_x: i32 = 2;
        let mut orig_y: i32 = 3;

        loop {
            let jet = if env.jet.next() == Some('<') {
                Direction::Left
            } else {
                Direction::Right
            };
            i += 1;
            if i % env.num_actions == 0 {
                let height = tetris.height + tetris.chamber.len();
                println!(
                    "rotate {}: {} {}",
                    i / env.num_actions,
                    height - prev_h,
                    num_rocks - prev_r
                );
                growth.push((height - prev_h, num_rocks - prev_r));
                prev_r = num_rocks;
                prev_h = height;
                if growth.len() == grmax {
                    let nr = growth.iter().fold(0, |prev, x| prev + x.1);
                    let rem = total - nr;
                    enough = num_rocks + rem % growth[grmax-1].1;
                }
            }

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
        }

        tetris.freeze(kind, (orig_x, orig_y));
    }

    let nr = growth.iter().fold(0, |prev, x| prev + x.1);
    let rem = total - nr;
    let n = rem / growth[grmax-1].1;

    tetris.height + tetris.chamber.len() + growth[grmax-1].0 * n
}

fn simulate(input: &str, steps: usize, report: Option<usize>) -> usize {
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

    let mut prev = 0;
    for cycle in 0..steps {
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
        report.map(|x| {
            if cycle % x == 0 {
                let h = tetris.height + tetris.chamber.len();
                println!("{cycle}: {}", h - prev);
                prev = h;
            }
        });
    }
    tetris.height + tetris.chamber.len()
}

fn solution_a(input: &str) -> usize {
    simulate(input, 2022, None)
}

fn solution_b(input: &str) -> usize {
    cyclist(input, 60000, Tetris::PART_2, 4)
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
        assert_eq!(cyclist(&data, 60000, Tetris::PART_2, 8), 1514285714288);
        //assert_eq!(solution_b(&data), 1514285714288);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 3171);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 1586627906921);
    }

    #[test]
    fn test_rem() {
        let data = simple().unwrap();
        let input = &data;
        let n = 19370;
        let normal = simulate(input, n, None);

        let w = (n - 320) / 280;
        let ww = (n - 320) % 280;

        let a = simulate(input, 320, None);
        let b = simulate(input, 600, None);
        let c = simulate(input, 600 + ww, None);

        assert_eq!(normal, a + (w - 0) * (b - a) + c - b);
    }

    #[test]
    fn test_cyclist() {
        let data = content().unwrap();
        let input = &data;
        let n = 7497659;
        let correct = simulate(input, n, None);
        assert_eq!(cyclist(input, 60000, n, 4), correct);
    }
}
