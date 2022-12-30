use itertools::Itertools;
use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Valley {
    rows: usize,
    cols: usize,
    draw: Vec<u8>,
    start: (usize, usize),
    goal: (usize, usize),
    blizzards: HashMap<(usize, usize), u8>,
}

impl Valley {
    fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .count();
        let mut draw = Vec::from_iter(input.bytes().filter(|c| !c.is_ascii_whitespace()));
        let cols = draw.len() / rows;
        let mut blizzards = HashMap::new();

        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                if draw[c + r * cols] != b'.' {
                    blizzards.insert((r, c), draw[c + r * cols]);
                    draw[c + r * cols] = b'.';
                }
            }
        }
        let start = (0, draw.iter().position(|&x| x == b'.').unwrap());
        let goal = (
            rows - 1,
            draw.iter().rposition(|&x| x == b'.').unwrap() % cols,
        );

        Valley {
            rows,
            cols,
            draw,
            start,
            goal,
            blizzards,
        }
    }

    fn simulate(&self, t: usize) -> Vec<u8> {
        let mut now = Vec::from_iter(self.draw.iter().map(|x| *x));
        let rr = self.rows - 2;
        let cc = self.cols - 2;
        self.blizzards.iter().for_each(|(pos, b)| {
            let pos = (pos.0 - 1, pos.1 - 1);
            let np = match b {
                b'<' => (pos.0, (pos.1 + cc - t % cc) % cc),
                b'>' => (pos.0, (pos.1 + t % cc) % cc),
                b'v' => ((pos.0 + t % rr) % rr, pos.1),
                b'^' => ((pos.0 + rr - t % rr) % rr, pos.1),
                _ => unreachable!("What? {b}"),
            };
            let index = (np.0 + 1) * self.cols + np.1 + 1;
            if now[index] == b'.' {
                now[index] = *b;
            } else if now[index].is_ascii_digit() {
                now[index] += 1;
            } else {
                now[index] = b'2';
            }
        });
        now
    }

    fn _display(&self, state: &Vec<u8>) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{}", state[c + r * self.cols] as char);
            }
            println!();
        }
    }

    fn walk(&self, t: usize, forward: bool) -> Option<usize> {
        const SIDES: [(i32, i32); 5] = [(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)];
        let start = if forward { self.start } else { self.goal };
        let goal = if forward { self.goal } else { self.start };
        let mut frontier = vec![start];
        let mut next = HashSet::new();
        let mut count = 0;
        let mut t = t;
        let mut state = self.simulate(t);
        let mut found = false;

        while !frontier.is_empty() && count < 400 && found == false {
            count += 1;
            next.clear();
            let mut next_state = self.simulate(t + 1);
            frontier
                .iter()
                .filter(|(r, c)| state[r * self.cols + c] == b'.')
                .for_each(|(r, c)| {
                    SIDES
                        .iter()
                        .map(|(dr, dc)| {
                            let (r, c) = (*r as i32 + dr, *c as i32 + dc);
                            if r < 0 || c < 0 {
                                return None;
                            }
                            let (r, c) = (r as usize, c as usize);
                            if r < self.rows
                                && c < self.cols
                                && next_state[c + r * self.cols] == b'.'
                            {
                                Some((r, c))
                            } else {
                                None
                            }
                        })
                        .filter(|x| x.is_some())
                        .for_each(|p| {
                            let (r, c) = p.unwrap();
                            next.insert((r, c));
                        });
                });

            found = next.contains(&goal);
            frontier.clear();
            frontier.extend(next.iter());
            state.clear();
            state.append(&mut next_state);
            t += 1;
        }

        if found {
            Some(t)
        } else {
            None
        }
    }

    fn _like_a_star(&self) -> Option<usize> {
        const SIDES: [(i32, i32); 5] = [(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)];
        let minimal = self.goal.1 - self.start.1 + self.goal.0 - self.start.0;
        let mut frontier = vec![(1, minimal, self.start)];
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();
        let mut rng = rand::thread_rng();
        let mut count = 0;
        came_from.insert(self.start, self.start);
        cost_so_far.insert(self.start, 0);

        while !frontier.is_empty() && count < 1000 {
            count += 1;
            let priority = frontier.first().unwrap().1;
            let (t, c_cost, c_pos) = *frontier
                .iter()
                .filter(|x| x.1 == priority)
                .choose(&mut rng)
                .unwrap();
            println!("Pop: ({}, {}) {t} {c_cost}", c_pos.0, c_pos.1);
            if c_pos == self.goal {
                break;
            }
            let state = self.simulate(t);
            let neighbors = SIDES
                .iter()
                .map(|(dr, dc)| {
                    let (r, c) = (c_pos.0 as i32 + dr, c_pos.1 as i32 + dc);
                    if r < 0 || c < 0 {
                        return None;
                    }
                    let (r, c) = (r as usize, c as usize);
                    if r < self.rows && c < self.cols && state[c + r * self.cols] == b'.' {
                        let heuristic = self.goal.0 - r + self.goal.1 - c;
                        Some((heuristic, (r, c)))
                    } else {
                        println!("Over: ({r},{c}) {}", state[c + r * self.cols] as char);
                        None
                    }
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                //.sorted_by_key(|x| x.0)
                .collect_vec();

            println!("Neighbors: {}", neighbors.len());
            neighbors.iter().for_each(|next| {
                let new_cost = t + 1;
                let (heuristic, next_pos) = *next;
                let so_far = cost_so_far.get(&next_pos);
                if so_far.is_none() || so_far.map(|so_far| new_cost < *so_far) == Some(true) {
                    cost_so_far.insert(next_pos, new_cost);
                    let priority = new_cost + heuristic;
                    frontier.push((t + 1, priority, next_pos));
                    came_from.insert(next_pos, c_pos);
                } else {
                    println!(
                        "Drop: ({}, {}) {:?} {}",
                        next_pos.0, next_pos.1, so_far, new_cost
                    )
                }
            });
            let state = self.simulate(t + 1);
            if state[c_pos.0 * self.cols + c_pos.1] == b'.' {
                frontier.push((t + 1, c_cost, c_pos));
            }
            /*
            let mut sorted = frontier
                .iter()
                .sorted_by_key(|(_, priority, _)| priority)
                .map(|x| *x)
                .collect_vec();
            frontier.clear();
            frontier.append(&mut sorted);
            */
            frontier.sort_by_key(|(_, priority, _)| *priority);
            frontier
                .iter()
                .for_each(|p| println!("({}, {}) {} {} ", p.2 .0, p.2 .1, p.0, p.1,));
            println!();
        }
        println!("Yay");
        if came_from.contains_key(&self.goal) {
            let mut count = 0;
            let mut p = self.goal;
            loop {
                let prev = came_from.get(&p);
                if prev.is_none() || prev.map(|p| *p == self.start) == Some(true) {
                    break;
                }
                count += 1;
                p = *prev.unwrap();
            }
            Some(count)
        } else {
            None
        }
    }

    fn _walk(&self) -> Option<usize> {
        let sides: [(i32, i32); 5] = [(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)];
        let mut path = Vec::new();
        let mut pos = self.start;
        let mut choice = 0;
        let mut t = 0;
        let mut count = 0;
        let mut visited = HashMap::new();

        while (pos.0 != self.goal.0 || pos.1 != self.goal.1) && count < 400000 {
            count += 1;
            let mut state = self.simulate(t);
            let steps = sides
                .iter()
                .map(|(dr, dc)| {
                    let (r, c) = (pos.0 as i32 + dr, pos.1 as i32 + dc);
                    if r < 0 || c < 0 {
                        return None;
                    }
                    let (r, c) = (r as usize, c as usize);
                    if r < self.rows && c < self.cols && state[c + r * self.cols] == b'.' {
                        let d = (self.goal.0 - r) + (self.goal.1 - c);
                        Some((d, (r, c)))
                    } else {
                        None
                    }
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .sorted_by_key(|x| x.0)
                .collect_vec();
            let stay = steps
                .iter()
                .position(|x| (*x).1 .0 == pos.0 && (*x).1 .1 == pos.1);

            if choice < steps.len() {
                pos = (steps[choice].1 .0, steps[choice].1 .1);
                path.push((choice, t, pos));
                visited.insert(pos, t).map(|prev| {
                    println!("Back ({},{}) {} -> {}", pos.0, pos.1, prev, t);
                });
                t += 1;
                choice = 0;
            } else if stay.is_some() && choice == steps.len() {
                t += 1;
                choice += 1;
                path.push((choice, t, pos));
            } else {
                let prev = path.pop();
                if prev.is_none() {
                    break;
                }
                prev.map(|p| {
                    (choice, t, pos) = p;
                    choice += 1;
                });
            }

            println!("Step: {count}");
            state[pos.0 * self.cols + pos.1] = b'E';
            self._display(&state);
            println!();
        }

        if !path.is_empty() && pos == self.goal {
            let unique: HashSet<(usize, usize)> = HashSet::from_iter(path.iter().map(|x| x.2));
            path.iter().for_each(|p| {
                println!("{} {} ({}, {})", p.0, p.1, p.2 .0, p.2 .1);
                /*
                let mut state = self.simulate(p.1);
                state[p.2.0 * self.cols + p.2.1] = b'E';
                println!("Step: {}", p.1);
                self.display(&state);
                println!();
                */
            });
            println!("{} {}", path.len(), unique.len());
            path.last().map(|x| println!("{}", x.1));
            Some(path.len() - 1)
        } else {
            None
        }
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let v = Valley::new(input);
    v.walk(0, true)
}

fn solution_b(input: &str) -> Option<usize> {
    let v = Valley::new(input);
    v.walk(0, true)
        .map(|t| v.walk(t, false).map(|t| v.walk(t, true)).flatten())
        .flatten()
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
        assert_eq!(solution_a(&data), Some(18));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(54));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(301));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(859));
    }
}
