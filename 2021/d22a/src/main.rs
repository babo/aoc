use std::collections::HashSet;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    fn read_input(input: &str, full_size: bool) -> Vec<Self> {
        input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Cuboid::new)
            .filter(|c| full_size || c.small())
            .collect::<Vec<Cuboid>>()
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
}

fn intersect_axis(points: &Vec<(i64, usize)>, index: usize) -> HashSet<usize> {
    let mut state = 0;
    let mut intersect = HashSet::new();
    points.iter().for_each(|n| {
        let current = n.1;
        match state {
            0 => {
                if n.1 == index {
                    state = 1;
                } else {
                    if intersect.contains(&current) {
                        intersect.remove(&current);
                    } else {
                        intersect.insert(current);
                    }
                }
            }
            1 => {
                if n.1 == index {
                    state = 2;
                } else {
                    intersect.insert(current);
                }
            }
            _ => (),
        }
    });
    intersect
}

fn count_on(cuboids: &Vec<Cuboid>) -> usize {
    let mut current: Vec<Cuboid> = Vec::new();
    cuboids.iter().enumerate().for_each(|(index, next)| {
        println!("index {index}");
        current.push(*next);
        let xs = current
            .iter()
            .enumerate()
            .flat_map(|(i, cube)| [(cube.x1, i), (cube.x2, i)])
            .sorted_by_key(|x| x.0)
            .collect_vec();
        let ys = current
            .iter()
            .enumerate()
            .flat_map(|(i, cube)| [(cube.y1, i), (cube.y2, i)])
            .sorted_by_key(|x| x.0)
            .collect_vec();
        let zs = current
            .iter()
            .enumerate()
            .flat_map(|(i, cube)| [(cube.z1, i), (cube.z2, i)])
            .sorted_by_key(|x| x.0)
            .collect_vec();

        let intersect_x = intersect_axis(&xs, index);
        let intersect_y = intersect_axis(&ys, index);
        let intersect_z = intersect_axis(&zs, index);
        let intersect_xy: HashSet<_> = intersect_x.intersection(&intersect_y).map(|x| *x).collect();
        let intersect: HashSet<_> = intersect_xy
            .intersection(&intersect_z)
            .map(|x| *x)
            .collect();
        if !next.is_on {
            current.pop();
        }

        intersect.iter().for_each(|other| {
            let other = current[*other];
            let is_on = !other.is_on;
            let x1 = other.x1.max(next.x1);
            let x2 = other.x2.min(next.x2);
            let y1 = other.y1.max(next.y1);
            let y2 = other.y2.min(next.y2);
            let z1 = other.z1.max(next.z1);
            let z2 = other.z2.min(next.z2);
            current.push(Cuboid {
                is_on,
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            });
        });
    });

    current.iter().fold(0, |prev, c| {
        let v = c.volume();
        if c.is_on {
            println!("{prev} + {v}");
            prev + v
        } else if v <= prev {
            println!("{prev} - {v}");
            prev - v
        } else {
            println!("0 {prev} + {v} {}", c.is_on);
            0
        }
    })
}

fn solution_a(input: &str) -> usize {
    let cuboids = Cuboid::read_input(input, false);
    count_on(&cuboids)
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
        assert_eq!(solution_a(&data), 590784);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 1);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 587785);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 10);
    }

    #[test]
    fn test_mini() {
        let input = content().unwrap();
        let cuboids = Cuboid::read_input(&input, true);

        let mini = cuboids
            .iter()
            .fold((i64::MAX, i64::MAX, i64::MAX), |prev, node| {
                (
                    prev.0.min(node.x1),
                    prev.1.min(node.y1),
                    prev.2.min(node.z1),
                )
            });
        let maxi = cuboids
            .iter()
            .fold((i64::MIN, i64::MIN, i64::MIN), |prev, node| {
                (
                    prev.0.max(node.x1),
                    prev.1.max(node.y1),
                    prev.2.max(node.z1),
                )
            });
        println!(
            "{:?} {:?}\n{} {} {}",
            mini,
            maxi,
            maxi.0 - mini.0,
            maxi.1 - mini.1,
            maxi.2 - mini.2
        );
        assert_eq!(cuboids.len(), 421);
    }

    #[test]
    fn test_volume() {
        /*
                let input = "on x=-20..20,y=-30..30,z=-1..1";
                assert_eq!(solution_a(&input), 4800);

                let input = "on x=-20..20,y=-30..30,z=-1..1
                on x=-10..10,y=-20..20,z=-1..1";
                assert_eq!(solution_a(&input), 4800);

                let input = "on x=-20..20,y=-30..30,z=-1..1
                off x=-10..10,y=-40..40,z=-1..1";
                assert_eq!(solution_a(&input), 2400);

                let input = "on x=-20..20,y=-30..30,z=-1..1
                off x=-10..10,y=-40..40,z=-1..1
                on x=-20..20,y=-30..30,z=-1..1";
                assert_eq!(solution_a(&input), 4800);

                let input = "on x=-20..20,y=-30..30,z=-1..1
                off x=-10..10,y=-40..40,z=-1..1
                on x=-20..20,y=-30..30,z=-1..1
                off x=-10..10,y=-30..40,z=-1..1";
                assert_eq!(solution_a(&input), 2400);
        */
        let input = "on x=-20..20,y=-30..30,z=-1..1
        off x=-10..10,y=-40..40,z=-1..1
        on x=-20..20,y=-30..30,z=-1..1
        off x=-10..10,y=-30..40,z=-1..1
        off x=-20..20,y=-30..30,z=-1..1";
        assert_eq!(solution_a(&input), 0);
    }
}
