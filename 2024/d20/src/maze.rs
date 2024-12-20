use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) struct Maze {
    maze: String,
    w: isize,
    h: isize,
}

fn heuristic((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

impl Maze {
    pub(crate) fn new(input: &str) -> Self {
        let maze = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<String>();
        let w = input.trim().find('\n').unwrap() as isize;
        let h = maze.len() as isize / w;
        Maze { maze, w, h }
    }

    fn margin(&self) -> isize {
        if self.h > 20 {
            100
        } else {
            50
        }
    }

    fn p_to_coord(&self, p: isize) -> (isize, isize) {
        (p % self.w, p / self.w)
    }

    fn mc(&self, x: isize, y: isize) -> char {
        if x >= 0 && x < self.w && y >= 0 && y < self.h {
            self.maze.chars().nth((y * self.w + x) as usize).unwrap()
        } else {
            '#'
        }
    }

    pub(crate) fn start(&self) -> (isize, isize) {
        let p_to_coord = |p: isize| (p % self.w, p / self.w);
        p_to_coord(self.maze.find('S').unwrap() as isize)
    }

    pub(crate) fn goal(&self) -> (isize, isize) {
        self.p_to_coord(self.maze.find('E').unwrap() as isize)
    }

    pub(crate) fn cheat20(&self) -> usize {
        let margin = self.margin();
        println!("Margin: {}", margin);

        if let Some((path, cost)) = self.astar(self.start(), isize::MAX) {
            println!("Initial cost: {}", cost);
            let mut initial_path = HashMap::new();
            path.iter().for_each(|(p, c)| {
                initial_path.insert(*p, *c);
            });

            let mut tried = HashSet::new();
            path.iter().enumerate().for_each(|(step, (orig, _))| {
                let step = step as isize;

                let new_round = (0..20).fold(HashMap::new(), |mut acc, round| {
                    if round == 0 {
                        acc.insert(*orig, 0);
                    }
                    let mut this_round = vec![];
                    acc.keys().for_each(|(x, y)| {
                        [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)]
                            .iter()
                            .for_each(|(dx, dy)| {
                                if round != 19 || self.mc(x + dx, y + dy) != '#' {
                                    this_round.push((x + dx, y + dy));
                                }
                            });
                    });
                    if round == 0 {
                        acc.remove(orig);
                    }
                    this_round.iter().for_each(|p| {
                        if !acc.contains_key(p)
                            && (!initial_path.contains_key(p) || initial_path[p] > step + round + 1)
                        {
                            acc.insert(*p, round + 1);
                        }
                    });
                    acc
                });

                new_round.iter().for_each(|(p, sub)| {
                    if let Some((path, c)) = self.astar(*p, cost - step - *sub) {
                        let mut hasher = DefaultHasher::new();
                        path.iter().for_each(|(p, _)| p.hash(&mut hasher));
                        let h = hasher.finish();
                        tried.insert((*p, step + sub + c, h));
                    }
                });
            });

            let v: HashSet<isize> =
                HashSet::from_iter(tried.iter().map(|x| x.1).filter(|x| *x + margin <= cost));
            for x in v.iter().sorted() {
                println!(
                    "{} saves {}",
                    tried.iter().filter(|y| y.1 == *x).count(),
                    cost - x
                );
            }
            tried.iter().filter(|y| y.1 == cost - 76).for_each(|x| {
                println!("{:?}", x);
            });

            tried.iter().filter(|c| c.1 + margin <= cost).count()
        } else {
            0
        }
    }

    pub(crate) fn cheat(&self, margin: isize) -> usize {
        if let Some((path, cost)) = self.astar(self.start(), isize::MAX) {
            println!("Initial cost: {}", cost);
            let mut tried = HashSet::new();
            path.iter().enumerate().for_each(|(i, (orig, _))| {
                let i = i as isize;
                [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .for_each(|(dx, dy)| {
                        let block = (orig.0 + dx, orig.1 + dy);
                        if self.mc(block.0, block.1) == '#' {
                            let mut next_in_path = None;
                            if let Some((path, c)) = self.astar(block, cost - margin - i - 1) {
                                let n = path.iter().skip(1).next().unwrap();
                                next_in_path = Some(n.0);
                                tried.insert((block, i + 1 + c));
                            }

                            [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)]
                                .iter()
                                .for_each(|(dx, dy)| {
                                    let start = (block.0 + dx, block.1 + dy);
                                    if start != *orig
                                        && self.mc(start.0, start.1) == '.'
                                        && next_in_path != Some(start)
                                    {
                                        if let Some((_, c)) = self.astar(start, cost - i - 2) {
                                            tried.insert((start, i + 2 + c));
                                        }
                                    }
                                });
                        }
                    });
            });
            /*
            let v: HashSet<isize> = HashSet::from_iter(tried.iter().map(|x| x.1));
            for x in v.iter().sorted() {
                println!("{} saves {}", tried.iter().filter(|y| y.1 == *x).count(), cost - x);
            }
            println!("Tried: {:?}", tried);
            */
            tried.iter().filter(|c| c.1 + margin <= cost).count()
        } else {
            0
        }
    }

    // A* algorithm
    pub(crate) fn astar(
        &self,
        start: (isize, isize),
        max_cost: isize,
    ) -> Option<(Vec<((isize, isize), isize)>, isize)> {
        let goal = self.goal();
        let mut open_set = HashSet::new();
        let mut visited = HashMap::new();
        let mut f_score = HashMap::new();
        let mut g_score = HashMap::new();
        f_score.insert(start, heuristic(start, goal));
        g_score.insert(start, 0);
        open_set.insert(start);

        while !open_set.is_empty() {
            let ordered = open_set
                .iter()
                .sorted_by(|a, b| {
                    f_score
                        .get(a)
                        .unwrap_or(&isize::MAX)
                        .cmp(f_score.get(b).unwrap_or(&isize::MAX))
                })
                .map(|(a, b)| (*a, *b))
                .collect_vec();
            let pos = *ordered.first().unwrap();

            let cost = *g_score.get(&pos).unwrap();
            if cost > max_cost {
                break;
            }
            if pos == goal {
                let mut opt_path = vec![(pos, cost)];
                let mut current = pos;
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
            open_set.remove(&pos);
            let (x, y) = pos;
            let mut neighbors = vec![];
            if self.mc(x, y - 1) != '#' {
                neighbors.push((x, y - 1));
            }
            if self.mc(x, y + 1) != '#' {
                neighbors.push((x, y + 1));
            }
            if self.mc(x + 1, y) != '#' {
                neighbors.push((x + 1, y));
            }
            if self.mc(x - 1, y) != '#' {
                neighbors.push((x - 1, y));
            }
            neighbors.iter().for_each(|n_coord| {
                let n_coord = *n_coord;
                let tentative_gscore = g_score.get(&pos).unwrap() + 1;
                if g_score
                    .get(&n_coord)
                    .map(|g_neighbor| tentative_gscore < *g_neighbor)
                    .unwrap_or(true)
                {
                    open_set.insert(n_coord);
                    f_score.insert(n_coord, tentative_gscore + heuristic(n_coord, goal));
                    g_score.insert(n_coord, tentative_gscore);
                    visited.insert(n_coord, pos);
                }
            });
        }

        None
    }
}
