use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

type State = ((usize, usize), u8);

trait DataSource<T> {
    fn get(&self, x: usize, y: usize) -> Option<usize>;
    fn w(&self) -> usize;
    fn h(&self) -> usize;
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Maze {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let data: Vec<u32> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        let width = input
            .lines()
            .map(|x| x.trim())
            .find(|x| !x.is_empty())
            .unwrap()
            .len();
        let height = data.len() / width;

        Maze {
            data,
            width,
            height,
        }
    }
}

impl DataSource<Maze> for Maze {
    fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.data.get(y * self.width + x).map(|v| *v as usize)
    }

    fn w(&self) -> usize {
        self.width
    }

    fn h(&self) -> usize {
        self.height
    }
}

trait Move {
    fn new(width: usize, height: usize) -> Self;
    fn initial_state() -> u8;
    fn moves(&self, state: &State) -> Vec<State>;
}

struct StepA {
    width: usize,
    height: usize,
}

impl Move for StepA {
    fn new(width: usize, height: usize) -> Self {
        StepA { height, width }
    }

    fn initial_state() -> u8 {
        2 << 3
    }

    fn moves(&self, ((x, y), state): &State) -> Vec<State> {
        let xx = *x as i32;
        let yy = *y as i32;
        let steps = 2 << 3;
        let remaining = state >> 3;
        let move_on = || ((remaining - 1) << 3) | (state & 3);
        let all = match state & 3 {
            0 => {
                if remaining == 0 {
                    vec![((xx, yy - 1), steps | 3), ((xx, yy + 1), steps | 1)]
                } else {
                    vec![
                        ((xx + 1, yy), move_on()),
                        ((xx, yy - 1), steps | 3),
                        ((xx, yy + 1), steps | 1),
                    ]
                }
            }
            1 => {
                if remaining == 0 {
                    vec![((xx - 1, yy), steps | 2), ((xx + 1, yy), steps | 0)]
                } else {
                    vec![
                        ((xx, yy + 1), move_on()),
                        ((xx - 1, yy), steps | 2),
                        ((xx + 1, yy), steps | 0),
                    ]
                }
            }
            2 => {
                if remaining == 0 {
                    vec![((xx, yy - 1), steps | 3), ((xx, yy + 1), steps | 1)]
                } else {
                    vec![
                        ((xx - 1, yy), move_on()),
                        ((xx, yy - 1), steps | 3),
                        ((xx, yy + 1), steps | 1),
                    ]
                }
            }
            3 => {
                if remaining == 0 {
                    vec![((xx - 1, yy), steps | 2), ((xx + 1, yy), steps | 0)]
                } else {
                    vec![
                        ((xx, yy - 1), move_on()),
                        ((xx - 1, yy), steps | 2),
                        ((xx + 1, yy), steps | 0),
                    ]
                }
            }
            _ => unimplemented!("What a direction!"),
        };

        all.iter()
            .filter(|(p, _state)| {
                p.0 >= 0 && p.1 >= 0 && p.0 < self.width as i32 && p.1 < self.height as i32
            })
            .map(|(p, d)| ((p.0 as usize, p.1 as usize), *d))
            .collect_vec()
    }
}

struct StepB {
    width: usize,
    height: usize,
}

impl Move for StepB {
    fn new(width: usize, height: usize) -> Self {
        StepB { height, width }
    }

    fn initial_state() -> u8 {
        9 << 3
    }

    fn moves(&self, ((x, y), state): &State) -> Vec<State> {
        let xx = *x as i32;
        let yy = *y as i32;
        let steps = 9 << 3;
        let remaining = state >> 3;
        let move_on = || ((remaining - 1) << 3) | (state & 3);
        let all = match state & 3 {
            0 => {
                if remaining == 0 {
                    vec![((xx, yy - 1), steps | 3), ((xx, yy + 1), steps | 1)]
                } else if remaining > 6 {
                    vec![((xx + 1, yy), move_on())]
                } else {
                    vec![
                        ((xx + 1, yy), move_on()),
                        ((xx, yy - 1), steps | 3),
                        ((xx, yy + 1), steps | 1),
                    ]
                }
            }
            1 => {
                if remaining == 0 {
                    vec![((xx - 1, yy), steps | 2), ((xx + 1, yy), steps | 0)]
                } else if remaining > 6 {
                    vec![((xx, yy + 1), move_on())]
                } else {
                    vec![
                        ((xx, yy + 1), move_on()),
                        ((xx - 1, yy), steps | 2),
                        ((xx + 1, yy), steps | 0),
                    ]
                }
            }
            2 => {
                if remaining == 0 {
                    vec![((xx, yy - 1), steps | 3), ((xx, yy + 1), steps | 1)]
                } else if remaining > 6 {
                    vec![((xx - 1, yy), move_on())]
                } else {
                    vec![
                        ((xx - 1, yy), move_on()),
                        ((xx, yy - 1), steps | 3),
                        ((xx, yy + 1), steps | 1),
                    ]
                }
            }
            3 => {
                if remaining == 0 {
                    vec![((xx - 1, yy), steps | 2), ((xx + 1, yy), steps | 0)]
                } else if remaining > 6 {
                    vec![((xx, yy - 1), move_on())]
                } else {
                    vec![
                        ((xx, yy - 1), move_on()),
                        ((xx - 1, yy), steps | 2),
                        ((xx + 1, yy), steps | 0),
                    ]
                }
            }
            _ => unimplemented!("What a direction!"),
        };

        all.iter()
            .filter(|(p, state)| {
                p.0 >= 0
                    && p.1 >= 0
                    && p.0 < self.width as i32
                    && p.1 < self.height as i32
                    && if state >> 3 == steps {
                        match state & 3 {
                            0 => (p.0 + 3) < self.width as i32,
                            1 => (p.1 + 3) < self.height as i32,
                            2 => p.0 >= 3,
                            3 => p.1 >= 3,
                            _ => unimplemented!("What a dir!"),
                        }
                    } else {
                        true
                    }
            })
            .map(|(p, d)| ((p.0 as usize, p.1 as usize), *d))
            .collect_vec()
    }
}

fn a_star<T: DataSource<T>, M: Move>(data: &T) -> Option<usize> {
    let w = data.w();
    let h = data.h();
    let mover = M::new(w, h);

    // Heuristic function
    let hf = |xy: (usize, usize)| w - xy.0 + h - xy.1 - 2;

    let start = ((0, 0), M::initial_state());
    let goal = (w - 1, h - 1);
    let mut open_set = HashSet::new();
    let mut came_from: HashMap<State, State> = HashMap::new();
    let mut f_score: HashMap<State, usize> = HashMap::new();
    let mut g_score = HashMap::new();
    open_set.insert(start);
    g_score.insert(start, 0usize);
    f_score.insert(start, hf(start.0));

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .map(|pos| (f_score.get(pos).map_or(usize::MAX, |v| *v), pos))
            .min()
            .unwrap()
            .1
            .clone();
        if current.0 == goal {
            let mut dbg = std::iter::repeat('.').take(w * h).collect_vec();

            let mut weight = 0usize;
            let mut pos = current;
            while pos.0 != start.0 {
                if let Some(c) = dbg.get_mut(pos.0 .0 + pos.0 .1 * w) {
                    *c = match pos.1 & 3 {
                        0 => '>',
                        1 => 'v',
                        2 => '<',
                        3 => '^',
                        _ => unimplemented!("What a char!"),
                    }
                }
                let wp = data.get(pos.0 .0, pos.0 .1).unwrap();
                weight += wp;
                pos = *came_from.get(&pos).unwrap();
            }
            for y in 0..h {
                println!("{}", dbg[(y * w)..((y + 1) * w)].iter().join(""))
            }
            return Some(weight);
        }
        open_set.remove(&current);

        for neighbor in mover.moves(&current).iter() {
            let n = *neighbor;
            let tentative_g_score = g_score
                .get(&current)
                .map(|gc| *gc + data.get(n.0 .0, n.0 .1).unwrap())
                .unwrap();
            if tentative_g_score < g_score.get(&n).map_or(usize::MAX, |v| *v) {
                came_from.insert(n, current);
                g_score.insert(n, tentative_g_score);
                f_score.insert(n, tentative_g_score + hf(n.0));
                open_set.insert(n);
            }
        }
    }
    None
}

fn solution_a(input: &str) -> Option<usize> {
    let data = Maze::new(input);
    a_star::<Maze, StepA>(&data)
}

fn solution_b(input: &str) -> Option<usize> {
    let data = Maze::new(input);
    a_star::<Maze, StepB>(&data)
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
        assert_eq!(solution_a(&data), Some(102));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(94));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(902));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1073));
    }
}
