use std::collections::VecDeque;
use std::fmt;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

pub fn flip10(v: u16) -> u16 {
    v.reverse_bits() >> 6
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Side {
    N,
    E,
    S,
    W,
}

impl Side {
    pub fn new(s: u8) -> Self {
        match s % 4 {
            0 => Side::N,
            1 => Side::E,
            2 => Side::S,
            _ => Side::W,
        }
    }
    pub fn opposite(&self) -> Self {
        match self {
            Side::N => Side::S,
            Side::E => Side::W,
            Side::S => Side::N,
            Side::W => Side::E,
        }
    }
}

struct Tile {
    id: u32,
    border: [u16; 4],
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Tile {
    pub fn new(id: u32, lines: &[u16; 10]) -> Self {
        let r = lines
            .iter()
            .map(|x| x & 1)
            .fold(0u16, |acc, x| (acc << 1) | x);
        let l = lines
            .iter()
            .map(|x| (x >> 9) & 1)
            .fold(0u16, |acc, x| (acc << 1) | x);

        let border: [u16; 4] = [lines[0], r, lines[9], l];
        Tile { id, border }
    }

    pub fn from_str(input: &str) -> Self {
        let raw: Vec<&str> = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(raw.len(), 11);

        let id: u32 = if raw[0].starts_with("Tile ") {
            let n = raw[0]
                .chars()
                .filter(|&x| x.is_numeric())
                .collect::<String>();

            n.parse::<u32>().unwrap()
        } else {
            panic!("Missing tile id");
        };

        let mut lines = [0u16; 10];
        raw.iter()
            .skip(1)
            .map(|line| {
                let bin: String = line
                    .chars()
                    .map(|x| match x {
                        '.' => '0',
                        '#' => '1',
                        _ => panic!("Unknow input {}", line),
                    })
                    .collect();
                u16::from_str_radix(&bin, 2).unwrap()
            })
            .enumerate()
            .for_each(|x| lines[x.0] = x.1);

        Self::new(id, &lines)
    }

    pub fn side(&self, side: Side, mode: u8) -> u16 {
        let mut index: i8 = (side as i8 + (mode & 3u8) as i8) & 3;
        let flp = match mode & 12u8 {
            12u8 => {
                index = (index + 2) & 3;
                true
            }
            8u8 => {
                index = (4 - index) & 3;
                index & 1 == 0
            }
            4u8 => {
                index = (2 - index) & 3;
                index & 1 == 1
            }
            _ => false,
        };
        if mode & 16 == 16 {
            index = 3 - index;
        }
        let v = self.border[index as usize];
        if flp {
            flip10(v)
        } else {
            v
        }
    }
}

struct GTable {
    tiles: Vec<Tile>,
    n: usize,
    square: u8,

    grid: Vec<Option<(usize, u8)>>, // which tile placed in what orientation
    location: Vec<Option<usize>>,   // where a tile placed in the grid
}

impl GTable {
    pub fn new(input: &str) -> Self {
        let tiles = partition_input(input);
        let n = tiles.len();
        let square = (n as f64).sqrt() as u8;

        GTable {
            tiles: tiles,
            n: n,
            square: square,
            grid: vec![None; n],
            location: vec![None; n],
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn gridp(&self, x: u8, y: u8) -> usize {
        (y * self.square + x) as usize
    }

    pub fn middle(&self) -> usize {
        self.gridp(self.square / 2, self.square / 2)
    }

    pub fn gridf(&self, pos: usize, dir: Side) -> Option<usize> {
        let x: u8 = pos as u8 % self.square;
        let y: u8 = pos as u8 / self.square;
        match dir {
            Side::E => {
                if x + 1 >= self.square {
                    None
                } else {
                    Some(self.gridp(x + 1, y))
                }
            }
            Side::W => {
                if x == 0 {
                    None
                } else {
                    Some(self.gridp(x - 1, y))
                }
            }
            Side::N => {
                if y == 0 {
                    None
                } else {
                    Some(self.gridp(x, y - 1))
                }
            }
            Side::S => {
                if y + 1 >= self.square {
                    None
                } else {
                    Some(self.gridp(x, y + 1))
                }
            }
        }
    }

    pub fn constrains(&self, pos: usize) -> [Option<u16>; 4] {
        let mut rtv: [Option<u16>; 4] = [None; 4];
        for i in 0..4 {
            let side = Side::new(i);
            rtv[i as usize] = self.gridf(pos, side).and_then(|x| {
                self.grid[x].and_then(|x| Some(self.tiles[x.0].side(side.opposite(), x.1)))
            })
        }
        rtv
    }

    pub fn place(&self, grid_pos: usize, skip: usize, ms: u8) -> Option<(usize, u8)> {
        let constrains = self.constrains(grid_pos);
        self.location.iter().enumerate().skip(skip).find_map(|val| {
            if val.1.is_none() {
                let index = val.0;
                let tile = &self.tiles[index];

                for m in ms..32 {
                    let mismatch = (0..4)
                        .map(|s| {
                            constrains[s].and_then(|x| Some(x == tile.side(Side::new(s as u8), m)))
                        })
                        .find(|x| *x == Some(false));
                    if mismatch.is_none() {
                        return Some((index, m));
                    }
                }
            }
            None
        })
    }

    pub fn checksum(&self) -> Option<usize> {
        [
            self.gridp(0, 0),
            self.gridp(self.square - 1, 0),
            self.gridp(0, self.square - 1),
            self.gridp(self.square - 1, self.square - 1),
        ]
        .iter()
        .map(|x| self.grid[*x])
        .map(|x| x.and_then(|x| self.tiles.get(x.0).and_then(|x| Some(x.id))))
        .fold(Some(1usize), |acc, x| {
            x.and_then(|x| Some(acc.unwrap() * x as usize))
        })
    }

    pub fn debug(&self) {
        for x in self.grid.iter().enumerate() {
            let t = x.1.and_then(|x| self.tiles.get(x.0)).unwrap();
            print!("{} ({}) ", t, x.1.and_then(|x| Some(x.1)).unwrap());
            if (x.0 + 1) as u8 % self.square == 0 {
                println!("")
            }
        }
    }
}

fn partition_input(input: &str) -> Vec<Tile> {
    let mut rtv: Vec<Tile> = Vec::new();
    let mut buffer = String::new();
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .fold(0, |count, line| {
            buffer.push_str(line);
            buffer.push('\n');

            if count >= 10 {
                rtv.push(Tile::from_str(&buffer));
                buffer.clear();
                0
            } else {
                count + 1
            }
        });
    rtv
}

fn solution_a(input: &str) -> Option<usize> {
    let mut gt = GTable::new(input);
    let mut history: Vec<usize> = Vec::new();
    let mut todo: VecDeque<usize> = VecDeque::new();
    let total = gt.len();

    println!("Number of tiles: {}", total);

    todo.push_front(gt.middle());

    let mut round = 0u32;
    let mut skip = 0usize;
    let mut ms = 0u8;
    while todo.front().is_some() {
        round += 1;
        let pos = *todo.front().unwrap();
        if gt.grid[pos].is_some() {
            todo.pop_front();
            continue;
        }
        match gt.place(pos, skip, ms) {
            Some(value) => {
                let index = value.0;
                gt.grid[pos] = Some(value);
                gt.location[index] = Some(pos);
                history.push(index);

                todo.pop_front();
                skip = 0;
                ms = 0;
                for side in 0..4 {
                    match gt.gridf(pos, Side::new(side)) {
                        Some(npos) => todo.push_back(npos),
                        None => (),
                    }
                }
            }
            None => loop {
                let h = history.pop();
                if h.is_none() {
                    println!("No more choice! {} {}", round, todo.len());
                    return None;
                }
                let index = h.unwrap();
                let gi = gt.location[index].unwrap();
                let w = gt.grid[gi].unwrap();
                gt.grid[gi] = None;
                gt.location[index] = None;
                todo.push_front(gi);

                if history.len() == 0 {
                    println!("{} {} {}", w.0, w.1, gi);
                }
                if w.1 + 1 < 20 {
                    skip = w.0;
                    ms = w.1 + 1;
                    break;
                } else if w.0 + 1 < total {
                    skip = w.0 + 1;
                    ms = 0;
                    break;
                }
            },
        }
    }

    gt.debug();
    gt.checksum()
}

fn solution_b(_input: &str) -> Option<usize> {
    None
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

    fn simple() -> String {
        read_to_string("./simple.txt").ok().unwrap()
    }

    fn solution() -> String {
        read_to_string("./simple-solution.txt").ok().unwrap()
    }

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_process_all_input() {
        let tiles = partition_input(&simple());
        assert_eq!(tiles.get(0).and_then(|x| Some(x.id)), Some(1171));
        assert_eq!(tiles.last().and_then(|x| Some(x.id)), Some(3079));
    }

    #[test]
    fn test_borders() {
        let tiles = partition_input(&simple());
        let tile = tiles.last().unwrap();
        assert_eq!(tile.border[0], 0b1010111110);
        assert_eq!(tile.border[1], 0b0100001000);
        assert_eq!(tile.border[2], 0b0010111000);
        assert_eq!(tile.border[3], 0b1001101000);
    }

    #[test]
    fn test_simple_a() {
        assert_eq!(solution_a(&simple()), Some(20899048083289));
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(solution_a(&content().unwrap()), Some(13224049461431));
    }

    #[test]
    fn test_flip10() {
        let a = u16::from_str_radix("0011010001", 2).unwrap();
        let b = u16::from_str_radix("1000101100", 2).unwrap();

        assert_eq!(flip10(a), b);
    }

    #[test]
    fn test_side() {
        let input = "
        Tile 2311:
        ....######
        .........#
        .........#
        ..........
        ..........
        #.........
        ..........
        #.........
        ..........
        #.#.#.....";

        let t = Tile::from_str(input);

        for i in 0..4 {
            println!("{} {:010b}", i, t.border[i]);
        }
        println!("----");
        for i in 0..32 {
            println!("{:02} {:010b}", i, t.side(Side::N, i));
            if i & 3 == 3 {
                println!("");
            }
        }
        assert_eq!(t.id, 2311);
        assert_eq!(t.side(Side::N, 0), t.border[0]);
        assert_eq!(t.side(Side::E, 0), t.border[1]);
        assert_eq!(t.side(Side::S, 0), t.border[2]);
        assert_eq!(t.side(Side::W, 0), t.border[3]);

        for i in 0u8..4 {
            assert_eq!(t.side(Side::N, i), t.border[i as usize]);
            assert_eq!(t.side(Side::E, i), t.border[((i + 1) % 4) as usize]);
            assert_eq!(t.side(Side::S, i), t.border[((i + 2) % 4) as usize]);
            assert_eq!(t.side(Side::W, i), t.border[((i + 3) % 4) as usize]);
        }
    }

    #[test]
    fn test_flip() {
        let input = "
        Tile 2311:
        ....######
        .........#
        .........#
        ..........
        ..........
        #.........
        ..........
        #.........
        ..........
        #.#.#.....";

        let t = Tile::from_str(input);

        assert_eq!(t.side(Side::E, 0), 0b1110000000);
        assert_eq!(t.side(Side::W, 0), 0b0000010101);
        assert_eq!(t.side(Side::N, 0), 0b0000111111);
        assert_eq!(t.side(Side::S, 0), 0b1010100000);

        assert_eq!(t.side(Side::E, 4), 0b0000000111);
        assert_eq!(t.side(Side::W, 4), 0b1010100000);
        assert_eq!(t.side(Side::N, 4), 0b1010100000);
        assert_eq!(t.side(Side::S, 4), 0b0000111111);

        assert_eq!(t.side(Side::E, 8), 0b0000010101);
        assert_eq!(t.side(Side::W, 8), 0b1110000000);
        assert_eq!(t.side(Side::N, 8), 0b1111110000);
        assert_eq!(t.side(Side::S, 8), 0b0000010101);

        assert_eq!(t.side(Side::E, 12), 0b1010100000);
        assert_eq!(t.side(Side::W, 12), 0b0000000111);
        assert_eq!(t.side(Side::N, 12), 0b0000010101);
        assert_eq!(t.side(Side::S, 12), 0b1111110000);
    }

    #[test]
    fn test_pattern_0() {
        let input = "
        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..";

        let t = Tile::from_str(input);

        assert_eq!(t.side(Side::N, 4), 0b1000110100);
        assert_eq!(t.side(Side::E, 4), 0b0100111110);
        assert_eq!(t.side(Side::S, 4), 0b1011000110);
        assert_eq!(t.side(Side::W, 4), 0b1001001011);
    }

    #[test]
    fn test_solution_patterns() {
        let a = partition_input(&simple());
        let b = partition_input(&solution());

        b.iter().for_each(|goal| {
            println!("id: {}", goal.id);
            let ts = a.iter().find(|x| x.id == goal.id).unwrap();
            let res = (0u8..32u8).find(|m| {
                println!(
                    "{}\n{} {} {} {}",
                    m,
                    ts.side(Side::N, *m),
                    ts.side(Side::E, *m),
                    ts.side(Side::S, *m),
                    ts.side(Side::W, *m)
                );
                println!(
                    "{} {} {} {}\n",
                    goal.side(Side::N, 0),
                    goal.side(Side::E, 0),
                    goal.side(Side::S, 0),
                    goal.side(Side::W, 0)
                );

                ts.side(Side::N, *m) == goal.side(Side::N, 0)
                    && ts.side(Side::E, *m) == goal.side(Side::E, 0)
                    && ts.side(Side::S, *m) == goal.side(Side::S, 0)
                    && ts.side(Side::W, *m) == goal.side(Side::W, 0)
            });
            assert!(res.is_some());
        });
    }

    impl Tile {
        pub fn test(id: u32, border: &[u16; 4]) -> Self {
            let border = *border;
            Tile { id, border }
        }
    }

    #[test]
    fn test_transform() {
        let border = [1u16, 2, 3, 4];
        let t = Tile::test(1u32, &border);
        [
            (0u8, 1u16),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 3),
            (5, flip10(2)),
            (6, 1),
            (7, flip10(4)),
            (8, flip10(1)),
            (9, 4),
            (10, flip10(3)),
            (11, 2),
            (12, flip10(3)),
            (13, flip10(4)),
            (14, flip10(1)),
            (15, flip10(2)),
        ]
        .iter()
        .for_each(|(m, val)| {
            println!("p: {}, {} == {}", m, t.side(Side::N, *m), val);
            assert_eq!(t.side(Side::N, *m), *val);
        });
    }

    #[test]
    fn test_monster() {
        assert_eq!(0, 273)
    }
}
