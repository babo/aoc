use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

const WINNER: [u64; 10] = [
    0x1f << 0,
    0x1f << 5,
    0x1f << 10,
    0x1f << 15,
    0x1f << 20,
    (1 << 0) + (1 << 5) + (1 << 10) + (1 << 15) + (1 << 20),
    (1 << 1) + (1 << 6) + (1 << 11) + (1 << 16) + (1 << 21),
    (1 << 2) + (1 << 7) + (1 << 12) + (1 << 17) + (1 << 22),
    (1 << 3) + (1 << 8) + (1 << 13) + (1 << 18) + (1 << 23),
    (1 << 4) + (1 << 9) + (1 << 14) + (1 << 19) + (1 << 24),
];

#[derive(Clone, Debug)]
struct Table {
    hash: HashMap<u8, u8>,
    drawn: u64,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut nums = [0u8; 25];
        self.hash
            .keys()
            .for_each(|k| nums[self.hash[k] as usize] = *k);
        for i in 0..5 {
            for j in 0..5 {
                let rtv = write!(f, "{:2} ", nums[i * 5 + j]);
                if !rtv.is_ok() {
                    return rtv;
                }
            }
            if i < 4 {
                let rtv = write!(f, "\n");
                if !rtv.is_ok() {
                    return rtv;
                }
            }
        }
        write!(f, "\n")
    }
}

impl Table {
    fn new(lines: &mut std::str::Lines) -> Option<Self> {
        if lines.next().is_some() {
            let mut hash: HashMap<u8, u8> = HashMap::new();
            let count = lines.take(5).fold(0, |acc, l| {
                l.split_whitespace().fold(acc, |acc, x| {
                    let num = u8::from_str_radix(x.trim(), 10).unwrap();
                    hash.insert(num, acc);
                    acc + 1
                })
            });
            assert_eq!(count, 25);
            return Some(Table { hash, drawn: 0u64 });
        }
        None
    }

    pub fn draw(&self, n: u8) -> Self {
        let drawn = self
            .hash
            .get(&n)
            .map_or(self.drawn, |pos| self.drawn | (1 << pos));
        Table {
            hash: self.hash.clone(),
            drawn,
        }
    }

    pub fn is_winner(&self) -> bool {
        WINNER
            .iter()
            .find(|x| (self.drawn & *x) ^ *x == 0)
            .map_or(false, |_| true)
    }

    pub fn score(&self, n: u8) -> u64 {
        let n: u64 = n as u64;
        self.hash.keys().fold(0u64, |acc, k| {
            self.hash
                .get(k)
                .map(|pos| {
                    if self.drawn & (1u64 << *pos) != 0 {
                        0u64
                    } else {
                        (n * *k as u64) as u64
                    }
                })
                .map_or(acc, |v| acc + v)
        })
    }
}

fn solution_a(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let draw: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| u8::from_str_radix(x.trim(), 10).unwrap())
        .collect();

    let mut tables: Vec<Table> = Vec::new();
    loop {
        match Table::new(&mut lines) {
            Some(t) => tables.push(t),
            None => break,
        }
    }
    assert_ne!(tables.len(), 0);

    draw.iter().find_map(|n| {
        for elem in tables.iter_mut() {
            *elem = elem.draw(*n);
        }
        tables
            .iter()
            .filter(|x| x.is_winner())
            .map(|x| x.score(*n))
            .max()
    })
}

fn solution_b(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let draw: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| u8::from_str_radix(x.trim(), 10).unwrap())
        .collect();

    let mut tables: Vec<Table> = Vec::new();
    loop {
        match Table::new(&mut lines) {
            Some(t) => tables.push(t),
            None => break,
        }
    }
    assert_ne!(tables.len(), 0);

    draw.iter()
        .map(|n| {
            for elem in tables.iter_mut() {
                *elem = elem.draw(*n);
            }
            let w = tables
                .iter()
                .filter(|x| x.is_winner() && x.score(1) > 0)
                .map(|x| x.score(*n))
                .max();
            tables = tables.iter().filter(|x| x.is_winner() == false).map(|x| x.clone()).collect();
            w
        })
        .filter(|x| x.is_some())
        .last()
        .unwrap()
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
    fn test_board_winner() {
        let c = "

         1  2  3  4  5
        11 12 13 14 15
        21 22 23 24 25
        31 32 33 34 35
        41 42 43 44 45
        ";
        let mut lines = c.lines();
        lines.next();
        let nums: [u8; 5] = [5, 15, 25, 35, 45];
        let table = nums
            .iter()
            .fold(Table::new(&mut lines).unwrap(), |acc, n| acc.draw(*n));
        println!("{} {:b}", table, table.drawn);
        assert_eq!(table.is_winner(), true);
    }

    #[test]
    fn test_simple_a() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_a(&data), Some(4512));
    }

    #[test]
    fn test_simple_b() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_b(&data), Some(1924));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(11536));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1284));
    }
}
