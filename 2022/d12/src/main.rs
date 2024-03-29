use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Terrain {
    data: Vec<u8>,
    rows: usize,
    cols: usize,

    start: (usize, usize),
    goal: (usize, usize),
}

impl Terrain {
    fn new(input: &str) -> Self {
        let mut data: Vec<u8> = input.bytes().filter(|c| c.is_ascii_alphabetic()).collect();
        let rows = input.lines().filter(|x| !x.trim().is_empty()).count();
        let cols = data.len() / rows;

        let goal = data.iter().position(|c| *c == b'E').unwrap();
        let start = data.iter().position(|c| *c == b'S').unwrap();
        data.get_mut(start as usize).map(|x| *x = b'a');
        data.get_mut(goal as usize).map(|x| *x = b'z');

        Terrain {
            data,
            rows,
            cols,
            start: (start / cols, start % cols),
            goal: (goal / cols, goal % cols),
        }
    }

    fn get(&self, rc: &(usize, usize)) -> Option<u8> {
        let (row, col) = *rc;
        if row < self.rows && col < self.cols {
            self.data.get((row * self.cols + col) as usize).map(|c| *c)
        } else {
            None
        }
    }

    fn direction(&self, position: &(usize, usize), dir: u8) -> Option<(usize, usize)> {
        self.get(position)
            .map(|value| {
                let nb = match dir {
                    0 => (position.0, position.1 + 1),
                    1 => (position.0 + 1, position.1),
                    2 if position.1 != 0 => (position.0, position.1 - 1),
                    3 if position.0 != 0 => (position.0 - 1, position.1),
                    _ => return None,
                };
                self.get(&nb)
                    .filter(|neighbor| {
                        *neighbor >= b'a' && *neighbor <= b'z' && *neighbor <= value + 1
                    })
                    .map(|_| nb)
            })
            .flatten()
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let terrain = Terrain::new(input);
    let mut visit: Vec<((usize, usize), usize)> = Vec::new();
    let mut next: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();

    visit.push((terrain.start, 0));
    while !visit.is_empty() {
        visit.iter().for_each(|pp| {
            let (position, count) = *pp;

            if seen.get(&position).map_or(true, |prev| prev > &count) {
                seen.insert(position, count);

                (0..4)
                    .map(|dir| terrain.direction(&position, dir))
                    .for_each(|p| {
                        p.map(|v| next.insert((v, count + 1)));
                    });
            }
        });
        visit.clear();
        visit.extend(next.iter());
        next.clear();
    }

    seen.get(&terrain.goal).map(|p| *p)
}

fn solution_b(input: &str) -> Option<usize> {
    let terrain = Terrain::new(input);
    let mut visit: Vec<((usize, usize), usize)> = Vec::new();
    let mut next: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();

    for r in 0..terrain.rows {
        for c in 0..terrain.cols {
            let k = (r, c);
            if terrain.get(&k).map_or(false, |v| v == b'a') {
                visit.push((k, 0usize));
            }
        }
    }

    while !visit.is_empty() {
        visit.iter().for_each(|pp| {
            let (position, count) = *pp;

            if seen.get(&position).map_or(true, |prev| prev >= &count) {
                seen.insert(position, count);

                (0..4)
                    .map(|dir| terrain.direction(&position, dir))
                    .for_each(|p| {
                        p.map(|v| next.insert((v, count + 1)));
                    });
            }
        });
        visit.clear();
        visit.extend(next.iter());
        next.clear();
    }

    seen.get(&terrain.goal).map(|p| *p)
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
        assert_eq!(solution_a(&data), Some(31));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(29));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(481));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(480));
    }
}
