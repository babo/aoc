use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

trait P<T> {
    fn get(self: &Self, x: usize, y: usize) -> Option<usize>;
    fn w(self: &Self) -> usize;
    fn h(self: &Self) -> usize;
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DataA {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl DataA {
    pub fn new(input: &str) -> Self {
        let d: Vec<u32> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        let w = input
            .chars()
            .enumerate()
            .find(|c| c.1.is_whitespace())
            .unwrap()
            .0;
        let h = d.len() / w;

        DataA {
            data: d,
            width: w,
            height: h,
        }
    }
}

impl P<DataA> for DataA {
    fn get(self: &Self, x: usize, y: usize) -> Option<usize> {
        self.data.get(y * self.width + x).map(|v| *v as usize)
    }

    fn w(self: &Self) -> usize {
        self.width
    }

    fn h(self: &Self) -> usize {
        self.height
    }
}

fn a_star<T: P<T>>(data: &T) -> Option<usize> {
    let w = data.w();
    let h = data.h();

    let up = |(x, y): (usize, usize)| if y > 0 { Some((x, y - 1)) } else { None };
    let down = |(x, y): (usize, usize)| if y + 1 < h { Some((x, y + 1)) } else { None };
    let left = |(x, y): (usize, usize)| if x > 0 { Some((x - 1, y)) } else { None };
    let right = |(x, y): (usize, usize)| if x + 1 < w { Some((x + 1, y)) } else { None };

    let hf = |xy: (usize, usize)| w - xy.0 + h - xy.1 - 2;

    let start = (0, 0);
    let goal = (w - 1, h - 1);
    let mut open_set = HashSet::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut g_score = HashMap::new();
    open_set.insert(start);
    g_score.insert(start, 0usize);
    f_score.insert(start, hf(start));

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .map(|pos| (f_score.get(pos).map_or(usize::MAX, |v| *v), pos))
            .min()
            .unwrap()
            .1
            .clone();
        if current == goal {
            let mut weight = 0usize;
            let mut pos = current;
            while pos != start {
                let wp = data.get(pos.0, pos.1).unwrap();
                weight += wp;
                pos = *came_from.get(&pos).unwrap();
            }
            return Some(weight);
        }
        open_set.remove(&current);

        for neighbor in [up(current), down(current), left(current), right(current)] {
            neighbor.map(|n| {
                let tentative_g_score = g_score
                    .get(&current)
                    .map(|gc| data.get(n.0, n.1).map(|dn| dn + *gc))
                    .unwrap()
                    .unwrap();
                if tentative_g_score < g_score.get(&n).map_or(usize::MAX, |v| *v) {
                    came_from.insert(n, current);
                    g_score.insert(n, tentative_g_score);
                    f_score.insert(n, tentative_g_score + hf(n));
                    open_set.insert(n);
                }
            });
        }
    }
    None
}

fn solution_a(input: &str) -> Option<usize> {
    let data = DataA::new(input);
    a_star(&data)
}

struct DataB {
    data: Vec<u32>,
    width: usize,
    height: usize,
    tile: usize,
}

impl DataB {
    pub fn new(input: &str, tile: usize) -> Self {
        let d: Vec<u32> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        let w = input
            .chars()
            .enumerate()
            .find(|c| c.1.is_whitespace())
            .unwrap()
            .0;
        let h = d.len() / w;

        DataB {
            data: d,
            width: w,
            height: h,
            tile,
        }
    }
}

impl P<DataB> for DataB {
    fn get(self: &Self, x: usize, y: usize) -> Option<usize> {
        let tx = x / self.width;
        let ty = y / self.height;
        if tx >= self.tile || ty >= self.tile {
            return None;
        }
        let rx = x % self.width;
        let ry = y % self.height;
        self.data
            .get(ry * self.width + rx)
            .map(|v| 1 + (*v as usize - 1 + tx + ty) % 9)
    }

    fn w(self: &Self) -> usize {
        self.width * self.tile
    }

    fn h(self: &Self) -> usize {
        self.height * self.tile
    }
}

fn solution_b(input: &str) -> Option<usize> {
    let data = DataB::new(input, 5);
    a_star(&data)
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
        assert_eq!(solution_a(&data), Some(40));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(315));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(537));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(2881));
    }

    #[test]
    fn test_tiling() {
        let input = simple().unwrap();
        let data = DataB::new(&input, 5);

        assert_eq!(data.width, 10);
        assert_eq!(data.height, 10);
        assert_eq!(data.h(), 50);
        assert_eq!(data.w(), 50);

        assert_eq!(data.get(0, 49), Some(6));
        assert_eq!(data.get(49, 0), Some(6));
        assert_eq!(data.get(49, 49), Some(9));
        assert_eq!(data.get(49, 50), None);
        assert_eq!(data.get(50, 49), None);

        let first_row = (0..data.w())
            .map(|x| {
                data.get(x, 0)
                    .map(|v| char::from_digit(v as u32, 10).unwrap())
                    .unwrap()
            })
            .fold("".to_string(), |mut acc, v| {
                acc.push(v);
                acc
            });

        assert_eq!(
            first_row,
            "11637517422274862853338597396444961841755517295286"
        );
    }
}

// 1163751742 227486285 3338597396 4449618417 55517295286
// 1163751742 227486285 3338597396 4449618417 55517295286
