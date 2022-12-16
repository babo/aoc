use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Vulcano<'a> {
    names: Vec<&'a str>,
    rates: HashMap<&'a str, u32>,
}

impl<'a> Vulcano<'a> {
    // Valve YJ has flow rate=15; tunnels lead to valves OC, PE, AC
    fn new(input: &'a str) -> Self {
        let names: Vec<_> = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|line| line.get(6..8).unwrap())
            .collect_vec();

        let mut rates: HashMap<&str, u32> = HashMap::new();
        let mut tunnel: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut routes: HashMap<(&str, &str), u32> = HashMap::new();

        input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .for_each(|line| {
                println!("{line}");
                let name = line.get(6..8).unwrap();
                let semi = line.find(';').unwrap();
                let rate = u32::from_str_radix(line.get(23..semi).unwrap(), 10).unwrap();
                let vp = line.find("valve").unwrap() + 5;
                if line.get(vp..vp + 1) == Some("s") {
                    let next: Vec<_> = line.get(vp + 2..).unwrap().split(", ").collect_vec();
                    next.iter().for_each(|n| {
                        routes.insert((name, n), 1);
                        routes.insert((n, name), 1);
                    });
                    tunnel.insert(name, next);
                } else {
                    let mut next = Vec::<&str>::new();
                    next.push(line.get(vp + 1..).unwrap());
                    tunnel.insert(name, next);
                }
                rates.insert(name, rate);
            });
        let mut visited = HashSet::new();
        let mut current: Vec<usize> = Vec::new();
        let mut next: Vec<usize> = Vec::new();
        current.push(0);

        while !current.is_empty() {
            next.clear();
            current.iter().for_each(|node_id| {
                if visited.insert(*node_id) {
                    let node = names[*node_id];
                    for k in tunnel[node].iter() {
                        println!("|{k}|");
                        let pos = names.iter().position(|x| x == k).unwrap();
                        next.push(pos);
                    }
                }
            });

            current.clear();
            current.extend(next.iter());
        }
        for (k, v) in tunnel.iter() {
            println!("{k} -> {:?}", v);
        }
        Vulcano { names, rates }
    }
}

fn solution_a(input: &str) -> usize {
    let v = Vulcano::new(input);
    println!("{:?}", v.names);
    for (k, v) in v.rates.iter() {
        println!("{k} -> {v}");
    }
    0
}

fn solution_b(_input: &str) -> usize {
    0
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
        assert_eq!(solution_a(&data), 1651);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 99999);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 99999);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 99999);
    }
}
