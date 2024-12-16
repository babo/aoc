use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn heuristic((current, dir): ((usize, usize), Direction), end: (usize, usize)) -> usize {
    let (x, y) = current;
    let dx = end.0 as isize - x as isize;
    let dy = end.1 as isize - y as isize;

    let t1 = match (dx.cmp(&0), dir) {
        (Ordering::Less, Direction::North) => 0,
        (Ordering::Equal, Direction::North) => 1000,
        (Ordering::Greater, Direction::North) => 2000,
        (Ordering::Less, Direction::South) => 2000,
        (Ordering::Equal, Direction::South) => 1000,
        (Ordering::Greater, Direction::South) => 0,
        _ => 0usize,
    };
    let t2 = match (dy.cmp(&0), dir) {
        (Ordering::Less, Direction::East) => 0,
        (Ordering::Equal, Direction::East) => 1000,
        (Ordering::Greater, Direction::East) => 2000,
        (Ordering::Less, Direction::West) => 2000,
        (Ordering::Equal, Direction::West) => 1000,
        (Ordering::Greater, Direction::West) => 0,
        _ => 0usize,
    };
    // dx.unsigned_abs() + dy.unsigned_abs() + t1 + t2;
    dx.unsigned_abs() + dy.unsigned_abs()
}

fn turn_price(from: Direction, to: Direction) -> usize {
    match (from, to) {
        (Direction::North, Direction::East) => 1000,
        (Direction::North, Direction::West) => 1000,
        (Direction::North, Direction::South) => 2000,
        (Direction::East, Direction::North) => 1000,
        (Direction::East, Direction::South) => 1000,
        (Direction::East, Direction::West) => 2000,
        (Direction::South, Direction::East) => 1000,
        (Direction::South, Direction::West) => 1000,
        (Direction::South, Direction::North) => 2000,
        (Direction::West, Direction::East) => 2000,
        (Direction::West, Direction::South) => 1000,
        (Direction::West, Direction::North) => 1000,
        _ => 0,
    }
}

struct Maze {
    maze: String,
    w: usize,
    h: usize,
}

impl Maze {
    fn new(input: &str) -> Self {
        let maze = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<String>();
        let w = input.trim().find('\n').unwrap();
        let h = maze.len() / w;
        Maze { maze, w, h }
    }

    fn p_to_coord(&self, p: usize) -> (usize, usize) {
        (p % self.w, p / self.w)
    }

    fn mc(&self, x: usize, y: usize) -> char {
        self.maze.chars().nth(y * self.w + x).unwrap()
    }

    fn start(&self) -> ((usize, usize), Direction) {
        let p_to_coord = |p: usize| (p % self.w, p / self.w);
        let start = p_to_coord(self.maze.find('S').unwrap());
        (start, Direction::East)
    }

    fn goal(&self) -> (usize, usize) {
        self.p_to_coord(self.maze.find('E').unwrap())
    }

    // A* algorithm
    fn astar(
        &self,
        start: ((usize, usize), Direction),
        end: (usize, usize),
        start_cost: usize,
        max_cost: usize,
    ) -> Option<(Vec<(((usize, usize), Direction), usize)>, usize)> {
        let mut open_set = HashSet::new();
        let mut visited = HashMap::new();
        let mut f_score = HashMap::new();
        let mut g_score = HashMap::new();
        f_score.insert(start, heuristic(start, end));
        g_score.insert(start, start_cost);
        open_set.insert(start);

        while !open_set.is_empty() {
            let ordered = open_set
                .iter()
                .sorted_by(|a, b| {
                    f_score
                        .get(a)
                        .unwrap_or(&usize::MAX)
                        .cmp(f_score.get(b).unwrap_or(&usize::MAX))
                })
                .map(|(a, b)| (*a, *b))
                .collect_vec();
            let pos_dir = *ordered.first().unwrap();
/*
            let s = f_score.get(&pos_dir).unwrap_or(&usize::MAX);
            let count = ordered.iter().take_while(|x| f_score.get(x) == Some(s)).count();
            println!("Open set: {:?}", count);
*/
            let cost = *g_score.get(&pos_dir).unwrap();
            if cost > max_cost {
                break;
            }
            let (pos, dir) = pos_dir;
            if pos == end {
                let mut opt_path = vec![(pos_dir, cost)];
                let mut current = pos_dir;
                while current != start {
                    if let Some(p) = visited.get(&current) {
                        current = *p;
                        opt_path.push((current, *g_score.get(&current).unwrap()));
                    } else {
                        break;
                    }
                }
                opt_path.reverse();
                return Some((opt_path, cost));
            }
            open_set.remove(&pos_dir);
            let (x, y) = pos;
            let mut neighbors = vec![];
            if self.mc(x, y - 1) != '#' {
                let tp = turn_price(dir, Direction::North);
                neighbors.push(((x, y - 1), Direction::North, tp + 1));
            }
            if self.mc(x, y + 1) != '#' {
                let tp = turn_price(dir, Direction::South);
                neighbors.push(((x, y + 1), Direction::South, tp + 1));
            }
            if self.mc(x + 1, y) != '#' {
                let tp = turn_price(dir, Direction::East);
                neighbors.push(((x + 1, y), Direction::East, tp + 1));
            }
            if self.mc(x - 1, y) != '#' {
                let tp = turn_price(dir, Direction::West);
                neighbors.push(((x - 1, y), Direction::West, tp + 1));
            }
            neighbors.iter().for_each(|(n_coord, n_dir, edge_weight)| {
                let n_coord_dir = (*n_coord, *n_dir);
                let tentative_gscore = g_score.get(&pos_dir).unwrap() + edge_weight;
                if g_score
                    .get(&n_coord_dir)
                    .map(|g_neighbor| tentative_gscore < *g_neighbor)
                    .unwrap_or(true)
                {
                    open_set.insert(n_coord_dir);
                    f_score.insert(n_coord_dir, tentative_gscore + heuristic(n_coord_dir, end));
                    g_score.insert(n_coord_dir, tentative_gscore);
                    visited.insert(n_coord_dir, pos_dir);
                }
            });
        }

        None
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    maze.astar(maze.start(), maze.goal(), 0, usize::MAX)
        .map(|(_, cost)| cost)
}

fn solution_b(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    let (path, max_cost) = maze
        .astar(maze.start(), maze.goal(), 0, usize::MAX)
        .unwrap();
    let mut visited = HashSet::new();
    let mut best_path = vec![path];

    while !best_path.is_empty() {
        let path = best_path.pop().unwrap();
        path.iter().for_each(|(xyd, _)| {
            visited.insert(*xyd);
        });

        for i in 0..path.len() {
            let (((x, y), dir), step_cost) = path[i];

            if maze.mc(x, y - 1) != '#' {
                let ns = ((x, y - 1), Direction::North);
                if !visited.contains(&ns) {
                    let start_cost = step_cost + turn_price(dir, ns.1) + 1;
                    if let Some((path, _)) = maze.astar(ns, maze.goal(), start_cost, max_cost) {
                        best_path.push(path);
                    }
                }
            }
            if maze.mc(x, y + 1) != '#' {
                let ns = ((x, y + 1), Direction::South);
                if !visited.contains(&ns) {
                    let start_cost = step_cost + turn_price(dir, ns.1) + 1;
                    if let Some((path, _)) = maze.astar(ns, maze.goal(), start_cost, max_cost) {
                        best_path.push(path);
                    }
                }
            }
            if maze.mc(x + 1, y) != '#' {
                let ns = ((x + 1, y), Direction::East);
                if !visited.contains(&ns) {
                    let start_cost = step_cost + turn_price(dir, ns.1) + 1;
                    if let Some((path, _)) = maze.astar(ns, maze.goal(), start_cost, max_cost) {
                        best_path.push(path);
                    }
                }
            }
            if maze.mc(x - 1, y) != '#' {
                let ns = ((x - 1, y), Direction::West);
                if !visited.contains(&ns) {
                    let start_cost = step_cost + turn_price(dir, ns.1) + 1;
                    if let Some((path, _)) = maze.astar(ns, maze.goal(), start_cost, max_cost) {
                        best_path.push(path);
                    }
                }
            }
        }
    }
    let visited: HashSet<(usize, usize)> = visited.iter().map(|x| x.0).collect();
    for y in 0..maze.h {
        for x in 0..maze.w {
            if visited.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", maze.mc(x, y));
            }
        }
        println!();
    }
    Some(visited.len())
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
    fn test_mini_a() {
        let data = "###############
                    #.......#....E#
                    #.#.###.#.###.#
                    #.....#.#...#.#
                    #.###.#####.#.#
                    #.#.#.......#.#
                    #.#.#####.###.#
                    #...........#.#
                    ###.#.#####.#.#
                    #...#.....#.#.#
                    #.#.#.###.#.#.#
                    #.....#...#.#.#
                    #.###.#.#.#.#.#
                    #S..#.....#...#
                    ###############";
        assert_eq!(solution_a(data), Some(7036));
    }

    #[test]
    fn test_mini_b() {
        let data = "###############
                    #.......#....E#
                    #.#.###.#.###.#
                    #.....#.#...#.#
                    #.###.#####.#.#
                    #.#.#.......#.#
                    #.#.#####.###.#
                    #...........#.#
                    ###.#.#####.#.#
                    #...#.....#.#.#
                    #.#.#.###.#.#.#
                    #.....#...#.#.#
                    #.###.#.#.#.#.#
                    #S..#.....#...#
                    ###############";
        assert_eq!(solution_b(data), Some(45));
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(11048));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(64));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(122492));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(520));
    }
}
