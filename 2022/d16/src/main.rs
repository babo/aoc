use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Vulcano {

}

impl Vulcano {
    // Valve YJ has flow rate=15; tunnels lead to valves OC, PE, AC
    fn new(input: &str) -> Self {
        let mut rates: HashMap<&str, u32> = HashMap::new();
        let mut tunnel: HashMap<&str, Vec<&str>> = HashMap::new();

        input.lines().map(|x|x.trim()).filter(|x| !x.is_empty()).for_each(|line| {
            println!("{line}");
            let name = line.get(6..8).unwrap();
            let semi = line.find(';').unwrap();
            let rate = u32::from_str_radix(line.get(23..semi).unwrap(), 10).unwrap();
            let vpos = line.find("valve").unwrap()+5;
            if line.get(vpos..vpos+1) == Some("s") {
                let next: Vec<_> = line.get(vpos+1..).unwrap().split(", ").collect_vec();
                tunnel.insert(name, next);
            } else {
                let mut next = Vec::<&str>::new();
                next.push(line.get(vpos+1..).unwrap());
                tunnel.insert(name, next);
            }
            rates.insert(name, rate);
        });
        for (k, v) in rates.iter() {
            println!("{k} -> {v}");
        }
        for (k, v) in tunnel.iter() {
            println!("{k} -> {:?}", v);
        }
        Vulcano {}
    }
}

fn solution_a(input: &str) -> usize {
    let _v = Vulcano::new(input);
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
