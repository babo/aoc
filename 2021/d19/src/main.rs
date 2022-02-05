use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn tr(pos: &(i32, i32, i32), orientation: u8) -> (i32, i32, i32) {
    let (x, y, z) = *pos;
    match orientation {
        0 => (x, y, z),
        1 => (-y, x, z),
        2 => (-x, -y, z),
        3 => (y, -x, z),

        4 => (x, -z, y),
        5 => (z, x, y),
        6 => (-x, z, y),
        7 => (-z, -x, y),

        8 => (x, z, -y),
        9 => (-z, x, -y),
        10 => (-x, -z, -y),
        11 => (z, -x, -y),

        12 => (y, z, x),
        13 => (-z, y, x),
        14 => (-y, -z, x),
        15 => (z, -y, x),

        16 => (z, y, -x),
        17 => (-y, z, -x),
        18 => (-z, -y, -x),
        19 => (y, -z, -x),

        20 => (-x, y, -z),
        21 => (-y, -x, -z),
        22 => (x, -y, -z),
        23 => (y, x, -z),

        _ => unreachable!("only 0 .. 23 allowed"),
    }
}

fn read_input(input: &str) -> Vec<Scanner> {
    let (last, mut scanners) = input
        .lines()
        .fold((Scanner::new(), Vec::new()), |mut acc, line| {
            let line = line.trim();
            if line.starts_with("--- scanner ") {
                line.find(char::is_numeric)
                    .map(|p| {
                        line[p..].find(char::is_whitespace).map(|ws| {
                            let n = u8::from_str_radix(&line[p..p + ws], 10).ok();
                            n.map_or(0, |x| x)
                        })
                    })
                    .flatten()
                    .map(|id| acc.0.id = id);
                acc
            } else if line.is_empty() {
                if acc.0.beacons.is_empty() {
                    acc
                } else {
                    acc.1.push(acc.0);
                    (Scanner::new(), acc.1)
                }
            } else {
                acc.0.add(line);
                (acc.0, acc.1)
            }
        });
    if last.beacons.len() > 0 {
        scanners.push(last);
    }
    scanners
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scanner {
    beacons: Vec<(i32, i32, i32)>,
    orientation: u8,
    position: (i32, i32, i32),
    id: u8,
}

impl Scanner {
    fn new() -> Self {
        let beacons: Vec<(i32, i32, i32)> = Vec::new();
        Scanner {
            beacons,
            orientation: 0,
            position: (0, 0, 0),
            id: 0,
        }
    }

    fn add(&mut self, line: &str) -> &Self {
        let coords: Vec<i32> = line
            .split(',')
            .map(|n| i32::from_str_radix(n, 10).ok())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(coords.len(), 3);
        self.beacons.push((coords[0], coords[1], coords[2]));
        self
    }

    // called on an unknown scanner
    fn dozen(&self, ref_s: &Self) -> Option<(u8, (i32, i32, i32))> {
        for orientation in 0..24 {
            let mut found: HashMap<(i32, i32, i32), u8> = HashMap::new();
            for n_bac in self.beacons.iter() {
                let nn = tr(n_bac, orientation);
                for r_bac in ref_s.beacons.iter() {
                    let rr = tr(r_bac, ref_s.orientation);
                    let dist = (rr.0 - nn.0, rr.1 - nn.1, rr.2 - nn.2);
                    if found.contains_key(&dist) {
                        let gt = found.get_mut(&dist).map(|v| {
                            *v += 1;
                            *v > 11
                        });
                        if gt == Some(true) {
                            return Some((orientation, dist));
                        }
                    } else {
                        found.insert(dist, 1);
                    }
                }
            }
        }
        None
    }
}

fn locate(input: &str) -> Vec<Scanner> {
    let mut scanners = read_input(input);
    let mut located: HashSet<usize> = HashSet::new();
    let mut to_check: HashSet<_> = (1..scanners.len()).collect();
    located.insert(0);

    while !to_check.is_empty() {
        let found = to_check.iter().find_map(|i| {
            let s: Vec<(usize, u8, (i32, i32, i32))> = located
                .iter()
                .map(|k| {
                    let ref_s = scanners.get(*k).unwrap();
                    let x = scanners.get(*i).unwrap();
                    x.dozen(ref_s).map(|found| {
                        let ori = found.0;
                        let p = found.1;
                        let p = (
                            ref_s.position.0 + p.0,
                            ref_s.position.1 + p.1,
                            ref_s.position.2 + p.2,
                        );
                        (*i, ori, p)
                    })
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect();
            s.get(0).map(|x| x.clone())
        });
        match found {
            Some((k, v, p)) => {
                to_check.remove(&k);
                located.insert(k);
                scanners.get_mut(k).map(|s| {
                    s.orientation = v;
                    s.position = p;
                });
            }
            None => unreachable!("It should always find a matching pair"),
        };
    }

    scanners
}

fn solution_a(input: &str) -> Option<usize> {
    let scanners = locate(input);

    let mut beacons: HashSet<(i32, i32, i32)> = HashSet::new();
    scanners.iter().for_each(|s| {
        s.beacons.iter().for_each(|b| {
            let bb = tr(b, s.orientation);
            beacons.insert((
                s.position.0 + bb.0,
                s.position.1 + bb.1,
                s.position.2 + bb.2,
            ));
        });
    });
    Some(beacons.len())
}

fn solution_b(input: &str) -> Option<usize> {
    let scanners = locate(input);
    let n = scanners.len();
    let mut manhattan = 0;
    for i in 0..n {
        for j in i + 1..n {
            scanners.get(i).map(|a| {
                scanners.get(j).map(|b| {
                    let m = (a.position.0 - b.position.0).abs() as usize
                        + (a.position.1 - b.position.1).abs() as usize
                        + (a.position.2 - b.position.2).abs() as usize;
                    if m > manhattan {
                        manhattan = m;
                    }
                })
            });
            println!("{} {}", i, j);
        }
    }
    Some(manhattan)
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
        assert_eq!(solution_a(&data), Some(79));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(3621));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(459));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(19130));
    }
}
