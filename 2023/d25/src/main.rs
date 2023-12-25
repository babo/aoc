use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_b(_line: &str) -> usize {
    0
}

fn try_split(
    connections: &HashMap<&str, Vec<&str>>,
    start: &str,
    candidates: &[&str],
    wires: ((&str, &str), (&str, &str), (&str, &str)),
) -> Option<(usize, usize)> {
    let wires = vec![
        wires.0,
        wires.1,
        wires.2,
        (wires.0 .1, wires.0 .0),
        (wires.1 .1, wires.1 .0),
        (wires.2 .1, wires.2 .0),
    ];
    let wires: HashMap<&str, &str> = HashMap::from_iter(wires);
    if wires.len() != 6 {
        return None;
    }

    let mut candidates: HashMap<&str, usize> =
        HashMap::from_iter(candidates.iter().map(|x| (*x, 0)));

    let mut seen: HashSet<&str> = HashSet::new();
    let mut open: Vec<&str> = vec![start];
    while !open.is_empty() {
        open = open.iter().fold(Vec::new(), |mut accu, node| {
            if seen.insert(node) {
                if let Some(p) = candidates.get_mut(node) {
                    *p += 1;
                }
                let pair = wires.get(node);
                connections[node]
                    .iter()
                    .filter(|k| pair.map_or(true, |x| **k != *x))
                    .filter(|x| !seen.contains(*x))
                    .for_each(|x| {
                        accu.push(x);
                    });
            }
            accu
        });
    }
    let others = candidates
        .iter()
        .filter(|x| *x.1 == 0)
        .map(|x| x.0)
        .copied()
        .collect_vec();
    if others.is_empty() {
        return None;
    }
    let left_size = seen.len();

    open = others;
    seen = HashSet::new();
    while !open.is_empty() {
        open = open.iter().fold(Vec::new(), |mut accu, node| {
            if seen.insert(node) {
                if let Some(p) = candidates.get_mut(node) {
                    *p += 1;
                }
                let pair = wires.get(node);
                connections[node]
                    .iter()
                    .filter(|k| pair.map_or(true, |x| **k != *x))
                    .filter(|x| !seen.contains(*x))
                    .for_each(|x| {
                        accu.push(x);
                    });
            }
            accu
        });
    }
    let right_size = seen.len();
    println!("{:?}", wires);
    println!("{left_size}");
    println!("{right_size}");

    Some((left_size, right_size))
}

fn split_graph(connections: &HashMap<&str, Vec<&str>>) -> Option<usize> {
    let res = connections
        .keys()
        .map(|k| {
            let mut seen: HashSet<&str> = HashSet::new();
            let mut open: Vec<(&str, usize)> = vec![(k, 1)];
            let mut steps: Vec<Vec<&str>> = Vec::new();

            while !open.is_empty() {
                open = open.iter().fold(Vec::new(), |mut accu, (node, level)| {
                    if seen.insert(node) {
                        accu.push((node, *level));

                        while steps.len() < *level {
                            steps.push(Vec::new());
                        }
                        if let Some(p) = steps.get_mut(level - 1) {
                            p.push(node);
                        }
                        connections[node]
                            .iter()
                            .filter(|x| !seen.contains(*x))
                            .for_each(|x| {
                                accu.push((x, level + 1));
                            });
                    }

                    accu
                });
            }
            steps
        })
        .map(|x| (x.len(), x))
        .sorted_by_key(|x| x.0)
        .collect_vec();
    let mini = res.iter().map(|x| x.0).min().unwrap();
    let candidates = res.iter().filter(|x| x.0 == mini).collect_vec();

    let n = candidates.len();
    let mut rtv = None;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let a = candidates[i].1.first().unwrap().first().unwrap();
                let b = candidates[j].1.first().unwrap().first().unwrap();
                let c = candidates[k].1.first().unwrap().first().unwrap();
                candidates[i].1.iter().skip(1).take(1).for_each(|x| {
                    let probes = candidates[i]
                        .1
                        .iter()
                        .skip(2)
                        .take(1)
                        .next()
                        .unwrap()
                        .iter()
                        .copied()
                        .collect_vec();
                    x.iter().for_each(|aa| {
                        candidates[j].1.iter().skip(1).take(1).for_each(|x| {
                            x.iter().for_each(|bb| {
                                candidates[k].1.iter().skip(1).take(1).for_each(|x| {
                                    x.iter().for_each(|cc| {
                                        let wires = ((*a, *aa), (*b, *bb), (*c, *cc));
                                        if let Some(x) = try_split(connections, a, &probes, wires)
                                        {
                                            rtv = Some(x.0 * x.1)
                                        }
                                    })
                                });
                            })
                        });
                    })
                });
            }
        }
    }

    rtv
}

fn _dbg(connections: &HashMap<&str, Vec<&str>>) {
    println!("graph {{");
    connections.keys().sorted().for_each(|k| {
        print!("    {k} -- {{");
        connections[k]
            .iter()
            .sorted()
            .enumerate()
            .for_each(|(i, v)| {
                print!("{}{v}", if i == 0 { " " } else { ", " });
            });
        println!(" }}");
    });
    println!("}}");
}

fn solution_a(input: &str) -> Option<usize> {
    let connections: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .fold(HashMap::new(), |mut accu, line| {
            let mut it = line.split(": ");
            let k = it.next().unwrap();
            if !accu.contains_key(k) {
                accu.insert(k, vec![]);
            }

            it.next()
                .unwrap()
                .split_ascii_whitespace()
                .fold(accu, |mut accu, v| {
                    if let Some(p) = accu.get_mut(v) {
                        p.push(k);
                    } else {
                        accu.insert(v, vec![k]);
                    }
                    if let Some(p) = accu.get_mut(k) {
                        p.push(v);
                    }
                    accu
                })
        });

    //dbg(&connections);
    split_graph(&connections)
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<usize>())
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
        assert_eq!(solution_a(&data), Some(54));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(554064));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
