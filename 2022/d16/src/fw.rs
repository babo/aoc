/*
    Implement the Floyd-Warshall Algorithm

    https://brilliant.org/wiki/floyd-warshall-algorithm/
*/

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Volcano<'a> {
    names: Vec<&'a str>,
    rates: Vec<i32>,
    start: usize,
    dist: Vec<Vec<usize>>,
}

impl<'a> Volcano<'a> {
    const TIME_LIMIT: u8 = 30;
    const REDUCED_TIME_LIMIT: u8 = 26;

    // Valve YJ has flow rate=15; tunnels lead to valves OC, PE, AC
    pub fn new(input: &'a str) -> Self {
        let names: Vec<_> = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|line| line.get(6..8).unwrap())
            .collect_vec();
        let count = names.len();
        let lookup: HashMap<&str, usize> =
            HashMap::from_iter(names.iter().enumerate().map(|a| (*a.1, a.0)));

        let mut rates = vec![0; count];
        let mut tunnel: HashMap<usize, Vec<usize>> = HashMap::new();

        input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .for_each(|line| {
                let node = *lookup.get(line.get(6..8).unwrap()).unwrap();
                let semi = line.find(';').unwrap();
                let rate = i32::from_str_radix(line.get(23..semi).unwrap(), 10).unwrap();
                let vp = line.find("valve").unwrap() + 5;
                if line.get(vp..vp + 1) == Some("s") {
                    let next: Vec<_> = line
                        .get(vp + 2..)
                        .unwrap()
                        .split(", ")
                        .map(|name| *lookup.get(name).unwrap())
                        .collect();
                    tunnel.insert(node, next);
                } else {
                    let mut next = Vec::new();
                    next.push(*lookup.get(line.get(vp + 1..).unwrap()).unwrap());
                    tunnel.insert(node, next);
                }
                rates[node] = rate;
            });

        let mut dist = vec![vec![usize::MAX; count]; count];
        for i in 0..count {
            dist[i][i] = 0;

            tunnel[&i].iter().for_each(|j| {
                dist[i][*j] = 1;
                dist[*j][i] = 1;
            });
        }

        for k in 0..count {
            for i in 0..count {
                for j in 0..count {
                    if dist[i][k] != usize::MAX
                        && dist[k][j] != usize::MAX
                        && dist[i][j] > dist[i][k] + dist[k][j]
                    {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }

        print!("    ");
        names.iter().for_each(|n| print!("  {}", n));
        println!();

        for i in 0..count {
            print!("{} ", names[i]);
            for j in 0..count {
                print!("  {:2}", dist[i][j]);
            }
            println!()
        }

        let start = *lookup.get("AA").unwrap();
        Volcano {
            names,
            rates,
            start,
            dist,
        }
    }

    fn down(
        &self,
        pos: usize,
        depth: usize,
        t: u8,
        valves: &HashMap<usize, u32>,
        visited: &HashSet<usize>,
    ) -> (usize, u32) {
        let has_more = valves.len() > visited.len() + 1;

        let a = valves
            .iter()
            .filter(|x| visited.get(x.0).is_none())
            .map(|x| {
                let v1 = *x.0;
                let steps = self.dist[pos][v1] as u8;
                if steps >= t {
                    return (v1, v1, 0);
                }
                let r1 = *x.1 * (t - steps - 1) as u32;
                let d = if has_more && depth > 0 && t > steps + 2 {
                    let mut visited = HashSet::from_iter(visited.iter().map(|x| *x));
                    visited.insert(v1);
                    let a = self.down(v1, depth - 1, t - steps - 1, valves, &visited);
                    a
                } else {
                    (v1, 0)
                };
                (v1, d.0, r1 + d.1)
            })
            .max_by_key(|x| x.2)
            .unwrap();
        //println!("Choose {} + {}: {}", self.names[a.0], self.names[a.1], a.2);
        (a.0, a.2)
    }

    fn dual_down(
        &self,
        pos_a: usize,
        pos_b: usize,
        depth: usize,
        t_a: u8,
        t_b: u8,
        valves: &HashMap<usize, u32>,
        visited: &HashSet<usize>,
    ) -> u32 {
        let has_more = valves.len() > visited.len() + 1;
        let selection_a = valves
            .iter()
            .map(|x| *x.0)
            .filter(|x| visited.get(x).is_none())
            .sorted_by_key(|x| self.dist[pos_a][*x])
            .collect_vec();
        let selection_b = selection_a
            .iter()
            .map(|x| *x)
            .sorted_by_key(|x| self.dist[pos_b][*x])
            .collect_vec();
        let mut visited = HashSet::from_iter(visited.iter().map(|x| *x));
        let mut pressure_m = 0;

        for i in 0..selection_a.len() {
            let next_a = selection_a[i];
            let steps_a = self.dist[pos_a][next_a] as u8;
            if steps_a >= t_a {
                continue;
            }
            let pressure_a = *valves.get(&next_a).unwrap() * (t_a - steps_a - 1) as u32;
            if pressure_a > pressure_m {
                pressure_m = pressure_a;
            }
            visited.insert(next_a);
            for j in 0..selection_b.len() {
                let next_b = selection_b[j];

                if next_a == next_b {
                    continue;
                }
                let steps_b = self.dist[pos_b][next_b] as u8;
                if steps_b >= t_b {
                    continue;
                }
                visited.insert(next_b);
                let pressure_b = *valves.get(&next_b).unwrap() * (t_b - steps_b - 1) as u32;
                let p_down = if has_more && depth > 0 {
                    self.dual_down(
                        next_a,
                        next_b,
                        depth - 1,
                        t_a - steps_a - 1,
                        t_b - steps_b - 1,
                        valves,
                        &visited,
                    )
                } else {
                    0
                };
                visited.remove(&next_b);
                if pressure_a + pressure_b + p_down > pressure_m {
                    pressure_m = pressure_a + pressure_b + p_down;
                }
            }
            visited.remove(&next_a);
        }
        pressure_m
    }
}

pub fn best_route(input: &str) -> usize {
    let v = Volcano::new(input);
    let valves = HashMap::<usize, u32>::from_iter(
        v.rates
            .iter()
            .enumerate()
            .filter(|x| *x.1 != 0)
            .map(|x| (x.0, *x.1 as u32)),
    );
    let visited = HashSet::new();

    let a = v.down(
        v.start,
        valves.len(),
        Volcano::TIME_LIMIT,
        &valves,
        &visited,
    );
    println!("Choose {}: {}", v.names[a.0], a.1);
    a.1 as usize
}

pub fn dual_route(input: &str) -> usize {
    let v = Volcano::new(input);
    let valves = HashMap::<usize, u32>::from_iter(
        v.rates
            .iter()
            .enumerate()
            .filter(|x| *x.1 != 0)
            .map(|x| (x.0, *x.1 as u32)),
    );
    let visited = HashSet::new();

    let pressure_m = v.dual_down(
        v.start,
        v.start,
        valves.len(),
        Volcano::REDUCED_TIME_LIMIT,
        Volcano::REDUCED_TIME_LIMIT,
        &valves,
        &visited,
    );
    println!("Choose: {}", pressure_m);
    pressure_m as usize
}
