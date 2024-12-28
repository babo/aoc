use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> usize {
    let mut connections: HashSet<[&str; 3]> = HashSet::new();
    let mut party: HashMap<&str, HashSet<&str>> = HashMap::new();

    input.trim().lines().for_each(|x| {
        let (a, b) = x.split('-').collect_tuple().unwrap();

        if let Some(x) = party.get_mut(a) {
            x.insert(b);
        } else {
            party.insert(a, HashSet::from_iter(std::iter::once(b)));
        }
        if let Some(x) = party.get_mut(b) {
            x.insert(a);
        } else {
            party.insert(b, HashSet::from_iter(std::iter::once(a)));
        }
    });

    party.iter().for_each(|(k, v)| {
        if k.starts_with("t") {
            for a in v.iter() {
                for b in v.iter() {
                    if a != b && party.get(a).unwrap().contains(b) {
                        let mut sx = [*k, *a, *b];
                        sx.sort();
                        connections.insert(sx);
                    }
                }
            }
        }
    });

    connections.len()
}

fn solution_b(input: &str) -> String {
    let mut connections: HashSet<&str> = HashSet::new();
    let mut party: HashMap<&str, HashSet<&str>> = HashMap::new();

    input.trim().lines().for_each(|x| {
        let (a, b) = x.split('-').collect_tuple().unwrap();
        connections.insert(x);

        if let Some(x) = party.get_mut(a) {
            x.insert(b);
        } else {
            party.insert(a, HashSet::from_iter(std::iter::once(b)));
        }
        if let Some(x) = party.get_mut(b) {
            x.insert(a);
        } else {
            party.insert(b, HashSet::from_iter(std::iter::once(a)));
        }
    });

    let res = party
        .iter()
        .fold((0, String::new()), |(maxi, key), (k, v)| {
            /*
            let mut p: HashSet<&str> = Itertools::combinations(v.iter(), 2).filter(|x| {
                let ab = format!("{}-{}", x[0], x[1]);
                let ba = format!("{}-{}", x[1], x[0]);
                connections.contains(ab.as_str()) || connections.contains(ba.as_str())
            }).map(|x| x.iter().map(|y| **y).collect_vec()).collect_vec().iter().flatten().map(|x| *x).collect();
            */
            let mut v = v.clone();
            v.insert(k);
            let mut keys: HashMap<String, usize> = HashMap::new();

            for a in v.iter() {
                let mut b: HashSet<&str> =
                    party.get(a).unwrap().clone().iter().map(|x| *x).collect();
                b.insert(a);
                let is: Vec<&str> = v.intersection(&b).map(|x| *x).sorted().collect_vec();
                let key = is.join(",");
                if let Some(x) = keys.get_mut(&key) {
                    *x += 1;
                } else {
                    keys.insert(key, 1);
                }
            }
            println!("{:?} {:?}", k, keys);
            let m = *keys.values().max().unwrap();
            if m > maxi {
                let (key, _) = keys.iter().find(|(_, v)| **v == m).unwrap();
                println!("{}", key);
                (m, key.clone())
            } else {
                (maxi, key)
            }
        });
    println!("{:?}", res);

    res.1
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
        assert_eq!(solution_a(&data), 7);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), "co,de,ka,ta");
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 1170);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), "bo,dd,eq,ik,lo,lu,ph,ro,rr,rw,uo,wx,yg");
    }
}
