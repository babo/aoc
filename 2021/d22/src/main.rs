use std::fs::read_to_string;
use std::collections::HashSet;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cuboid {
    is_on: bool,
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

impl Cuboid {
    fn new(line: &str) -> Self {
        let line = String::from(line) + " ";
        let is_on = match line.get(0..3) {
            Some("on ") => true,
            Some("off") => false,
            _ => unreachable!("Should start with on or off"),
        };
        let coordinates: Vec<i64> = line
            .chars()
            .fold((Vec::new(), String::new()), |mut acc, c| {
                if char::is_numeric(c) || (c == '-' && acc.1.is_empty()) {
                    acc.1.push(c);
                    acc
                } else if !acc.1.is_empty() {
                    let n = i64::from_str_radix(acc.1.as_str(), 10).unwrap();
                    acc.0.push(n);
                    (acc.0, String::new())
                } else {
                    acc
                }
            })
            .0;

        Cuboid {
            is_on,
            x1: *coordinates.get(0).unwrap(),
            x2: *coordinates.get(1).unwrap(),
            y1: *coordinates.get(2).unwrap(),
            y2: *coordinates.get(3).unwrap(),
            z1: *coordinates.get(4).unwrap(),
            z2: *coordinates.get(5).unwrap(),
        }
    }

    fn small(&self) -> bool {
        self.x1 > -50
            && self.x2 < 50
            && self.y1 > -50
            && self.y2 < 50
            && self.z1 > -50
            && self.z2 < 50
    }

    fn volume(&self) -> usize {
        ((self.x2 - self.x1) * (self.y2 - self.y1) * (self.z2 - self.z1)) as usize
    }

    fn union(&self, other: &Self) -> Option<Self> {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);

        if x1 > x2 || y1 > y2 || z1 > z2 {
            None
        } else {
            Some(Cuboid {
                is_on: self.is_on,
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        }
    }
}

fn read_input(input: &str, full_size: bool) -> Vec<Cuboid> {
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(Cuboid::new)
        .filter(|c| full_size || c.small())
        .collect::<Vec<Cuboid>>()
}

fn count_on(cuboids: &Vec<Cuboid>, prev: usize, is_add: bool) -> usize {
    println!(
        "count_on: {} add: {} len: {}",
        prev,
        is_add,
        cuboids.len()
    );
    if cuboids.is_empty() {
        return prev;
    }

    let mut current = 0;
    let mut minus = 0usize;
    let mut round: Vec<Cuboid> = Vec::new();
    for i in 0..cuboids.len() {
        match cuboids.get(i).map(|x| if x.is_on { x.volume() } else { 0 }) {
            Some(v) => {
                println!("index: {} volume: {}", i, v);
                current += v
            }
            None => unreachable!("All keys must be valid"),
        }
        for j in 0..i {
            match cuboids[i].union(&cuboids[j]) {
                Some(union) => {
                    let v = union.volume();
                    println!("union: ({}, {}) volume {}", j, i, v);

                    if union.is_on {
                        round.push(union);
                    } else {
                        minus += v;
                    }
                }
                None => (),
            }
        }
    }
    if is_add {
        println!("");
        count_on(&round, prev + current - minus, !is_add)
    } else {
        let rtv = prev + minus - current;
        println!("{}+{}-{} = {}", prev, minus, current, rtv);
        prev + minus - current
    }
}

fn solution(input: &str, full_size: bool) -> Option<usize> {
    let count = read_input(input, full_size)
        .iter()
        .fold(HashSet::new(), |mut hm, cub| {
            for x in cub.x1..=cub.x2 {
                for y in cub.y1..=cub.y2 {
                    for z in cub.z1..=cub.z2 {
                        let key = (x, y, z);
                        if cub.is_on {
                            if !hm.contains(&key) {
                                hm.insert(key);
                            }
                        } else {
                            if hm.contains(&key) {
                                hm.remove(&key);
                            }
                        }
                    }
                }
            }

            hm
        })
        .len();
    Some(count)
}

fn solution_a(input: &str) -> Option<usize> {
    let cuboids = read_input(input, false);

    let c = count_on(&cuboids, 0, true);
    Some(c)
}

fn solution_b(input: &str) -> Option<usize> {
    let cuboids = read_input(input, true);
    let mm = cuboids.iter().fold((0, 0, 0, 0, 0, 0), |acc, c| {
        (
            acc.0.min(c.x1),
            acc.1.max(c.x1),
            acc.2.min(c.y1),
            acc.3.max(c.y1),
            acc.4.min(c.z1),
            acc.5.max(c.z1),
        )
    });
    println!("count: {}", cuboids.len());
    println!("{} {} {} {} {} {}", mm.0, mm.1, mm.2, mm.3, mm.4, mm.5);
    None
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
    fn test_overlap() {
        let data = "on x=10..12,y=10..12,z=10..12
        on x=10..12,y=10..12,z=10..12
        on x=10..12,y=10..12,z=10..12
        on x=10..12,y=10..12,z=10..12";
        assert_eq!(solution_a(&data), Some(27));
    }

    #[test]
    fn test_2d_1() {
        let data = "on x=0..5,y=1..4,z=0..1
        on x=1..4,y=0..5,z=0..1
        on x=2..3,y=2..3,z=0..1
        ";
        assert_eq!(solution_a(&data), Some(21));
    }

    #[test]
    fn test_2d_2() {
        let data = "on x=0..5,y=1..4,z=0..1
        on x=2..3,y=2..3,z=0..1
        on x=2..3,y=2..3,z=0..1
        ";
        assert_eq!(solution_a(&data), Some(15));
    }

    #[test]
    fn test_2d_3() {
        let data = "on x=0..5,y=0..5,z=0..1
        on x=0..5,y=0..5,z=0..1
        on x=0..5,y=0..5,z=0..1
        ";
        assert_eq!(solution_a(&data), Some(25));
    }

    #[test]
    fn test_mini_a() {
        let data = "on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13
        off x=9..11,y=9..11,z=9..11
        on x=10..10,y=10..10,z=10..10";
        assert_eq!(solution_a(&data), Some(39));
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(590784));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(0));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
