use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Blueprint {
    id: u32,
    cost: [u32; 12],
}

impl Blueprint {
    const OVERLAP: u32 = 4;

    // Blueprint 30: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.
    fn new(input: &str) -> Self {
        let colon = input.find(':').unwrap();
        let id = u32::from_str_radix(input.get(10..colon).unwrap(), 10).unwrap();
        let parts = input
            .get(colon + 2..)
            .unwrap()
            .split(".")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect_vec()
            .iter()
            .map(|desc| {
                let x = desc.split(' ').collect_vec();
                let p1 = u32::from_str_radix(x[4], 10).unwrap();
                let p2 = if x.len() > 6 {
                    u32::from_str_radix(x[7], 10).unwrap()
                } else {
                    0
                };
                (x[1], p1, p2)
            })
            .collect_vec();
        let cost = [
            parts[0].1, 0, 0, parts[1].1, 0, 0, parts[2].1, parts[2].2, 0, parts[3].1, 0,
            parts[3].2,
        ];
        Blueprint { id, cost }
    }

    fn run(&self, t: u32, robots: [u32; 4], stocks: [u32; 4]) -> u32 {
        if t == 0 {
            return stocks[3];
        }
        let mut decisions = (0..4)
            .map(|i| {
                stocks[0] >= self.cost[i * 3]
                    && stocks[1] >= self.cost[i * 3 + 1]
                    && stocks[2] >= self.cost[i * 3 + 2]
            })
            .collect_vec();
        if decisions[3] {
            decisions[0] = false;
            decisions[1] = false;
            decisions[2] = false;
        }
        if decisions[1] && robots[2] > Blueprint::OVERLAP {
            decisions[1] = false;
        }
        if decisions[0] && robots[0] > Blueprint::OVERLAP {
            decisions[0] = false;
        }
        let m = (0..4)
            .map(|d| {
                if decisions[d] {
                    let mut s = stocks.clone();
                    let mut r = robots.clone();
                    for i in 0..3 {
                        s[i] -= self.cost[d * 3 + i];
                    }
                    for i in 0..4 {
                        s[i] += r[i];
                    }
                    r[d] += 1;
                    self.run(t - 1, r, s)
                } else {
                    0
                }
            })
            .max()
            .unwrap();
        let dcount = decisions.iter().filter(|d| **d).count();
        if dcount == 0 || (dcount == 1 && (decisions[0] || decisions[1] || decisions[2])) {
            let mut s = stocks.clone();
            for i in 0..4 {
                s[i] += robots[i];
            }
            m.max(self.run(t - 1, robots, s))
        } else {
            m
        }
    }

    fn simulate(&self, t: u32) -> u32 {
        let robots = [1, 0, 0, 0];
        let stocks = [0; 4];
        self.run(t, robots, stocks)
    }
}

fn solution_a(input: &str) -> usize {
    let plans = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|line| Blueprint::new(line))
        .collect_vec();
    plans
        .iter()
        .map(|bp| {
            let n = bp.simulate(24);
            bp.id as usize * n as usize
        })
        .sum()
}

fn solution_b(input: &str, keep: usize) -> usize {
    let plans = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .take(keep)
        .map(|line| Blueprint::new(line))
        .collect_vec();
    plans.iter().fold(1, |prev, bp| {
        let n = bp.simulate(32);
        println!("Plan {}: {}", bp.id, n);
        prev * n as usize
    })
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c, 3);

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
        assert_eq!(solution_a(&data), 33);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data, 2), 56 * 62);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 2193);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c, 3), 7200);
    }
}
