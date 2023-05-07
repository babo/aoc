use std::collections::HashSet;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Cuboid {
    is_on: bool,
    p: [i64; 6],
}

fn intersect_xyz(a1: i64, a2: i64, b1: i64, b2: i64) -> bool {
    if a2 < b1 || a1 > b2 {
        return false;
    }
    return true;
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

        let p = [
            *coordinates.get(0).unwrap(),
            *coordinates.get(1).unwrap(),
            *coordinates.get(2).unwrap(),
            *coordinates.get(3).unwrap(),
            *coordinates.get(4).unwrap(),
            *coordinates.get(5).unwrap(),
        ];
        Cuboid { is_on, p }
    }

    fn cut_at(&self, xyz: usize, pos: i64) -> (Self, Self) {
        let mut p0 = self.p.clone();
        let mut p1 = self.p.clone();
        p0[xyz + 0] = pos;
        p1[xyz + 1] = pos;

        (
            Cuboid {
                is_on: self.is_on,
                p: p0,
            },
            Cuboid {
                is_on: self.is_on,
                p: p1,
            },
        )
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
        self.p[0] > -50
            && self.p[1] < 50
            && self.p[2] > -50
            && self.p[3] < 50
            && self.p[4] > -50
            && self.p[5] < 50
    }

    fn volume(&self) -> usize {
        ((self.p[1] - self.p[0]) * (self.p[3] - self.p[2]) * (self.p[5] - self.p[4])) as usize
    }

    fn intersect(&self, other: &Self) -> bool {
        intersect_xyz(self.p[0], self.p[1], other.p[0], other.p[1])
            && intersect_xyz(self.p[2], self.p[3], other.p[2], other.p[3])
            && intersect_xyz(self.p[4], self.p[5], other.p[4], other.p[5])
    }
}

fn cut(
    cuboids: &Vec<Cuboid>,
    points: &Vec<(i64, usize, bool)>,
    offset: usize,
) -> Vec<(usize, i64)> {
    let mut cuts: HashSet<(usize, i64)> = HashSet::new();
    for i in 0..points.len() {
        if points[i].2 {
            let a = points[i].1;
            let mini = cuboids[a].p[offset];
            let maxi = cuboids[a].p[offset + 1];
            for j in i + 1..points.len() {
                if points[j].0 >= maxi {
                    break;
                }
                let b = points[j].1;
                if points[j].0 > mini && a != b && cuboids[a].intersect(&cuboids[b]) {
                    cuts.insert((a, points[j].0));
                }
            }
        }
    }
    cuts.iter()
        .sorted_by_key(|x| x.1)
        .sorted_by_key(|x| x.0)
        .map(|x| *x)
        .collect_vec()
}

fn cut_iter(cuboids: &Vec<Cuboid>, points: &Vec<(i64, usize, bool)>, offset: usize) -> Vec<Cuboid> {
    let cuts = cut(&cuboids, points, offset);
    println!("Cuts: {}", cuts.len());
    let mut index = 0;
    let mut prev: Option<Cuboid> = None;
    let mut sliced: Vec<Cuboid> = Vec::new();
    cuts.iter().for_each(|x| {
        if prev.is_none() || index != x.0 {
            prev.map(|p| sliced.push(p));
            prev = Some(cuboids[x.0].clone());
            for missing in index+1..x.0 {
                sliced.push(cuboids[missing].clone());
            }
            index = x.0;
        }
        prev.map(|p| {
            let (a, b) = p.cut_at(offset, x.1);
            sliced.push(b);
            prev = Some(a);
        });
    });
    prev.map(|p| sliced.push(p));
    sliced
}

fn count_on(cuboids: &Vec<Cuboid>) -> usize {
    let xs = cuboids
        .iter()
        .enumerate()
        .flat_map(|(i, cube)| [(cube.p[0], i, true), (cube.p[1], i, false)])
        .sorted_by_key(|x| x.0)
        .collect_vec();
    let sliced_x = cut_iter(&cuboids, &xs, 0);
    let ys = sliced_x
        .iter()
        .enumerate()
        .flat_map(|(i, cube)| [(cube.p[2], i, true), (cube.p[3], i, false)])
        .sorted_by_key(|x| x.0)
        .collect_vec();
    let sliced_y = cut_iter(&sliced_x, &ys, 2);
    let zs = sliced_y
        .iter()
        .enumerate()
        .flat_map(|(i, cube)| [(cube.p[4], i, true), (cube.p[5], i, false)])
        .sorted_by_key(|x| x.0)
        .collect_vec();
    let sliced_z = cut_iter(&sliced_y, &zs, 4);

    let v0 = cuboids.iter().fold(0, |v, p| v+p.volume());
    let v1 = sliced_x.iter().fold(0, |v, p| v+p.volume());
    let v2 = sliced_y.iter().fold(0, |v, p| v+p.volume());
    let v3 = sliced_z.iter().fold(0, |v, p| v+p.volume());
    println!("Volumes: {v0} {v1} {v2} {v3}");

    let mut cubes = HashSet::new();
    sliced_z.iter().for_each(|p| {
        if p.is_on {
            cubes.insert(p);
        } else {
            let on = Cuboid { is_on: true, p: p.p};
            if cubes.remove(&on) {
                println!("Removed");
            } else {
                println!("Not found");
            }
        }
    });
    cubes.iter().fold(0, |v, p| v+p.volume())
}

fn solution_a(input: &str) -> usize {
    let cuboids = Cuboid::read_input(input, false);
    count_on(&cuboids)
}

fn solution_b(input: &str) -> usize {
    let cuboids = Cuboid::read_input(input, true);
    count_on(&cuboids)
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
        assert_eq!(solution_b(&data), 2758514936282235);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 587785);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 1);
    }

    #[test]
    fn test_mini() {
        let input = content().unwrap();
        let cuboids = Cuboid::read_input(&input, true);

        let mini = cuboids
            .iter()
            .fold((i64::MAX, i64::MAX, i64::MAX), |prev, node| {
                (
                    prev.0.min(node.p[0]),
                    prev.1.min(node.p[2]),
                    prev.2.min(node.p[4]),
                )
            });
        let maxi = cuboids
            .iter()
            .fold((i64::MIN, i64::MIN, i64::MIN), |prev, node| {
                (
                    prev.0.max(node.p[0]),
                    prev.1.max(node.p[2]),
                    prev.2.max(node.p[4]),
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
        assert_eq!(cuboids.len(), 420);
    }

    #[test]
    fn test_volume() {
        let input = "on x=-20..20,y=-30..30,z=-1..1";
        assert_eq!(solution_a(&input), 4800);
/*
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

        let input = "on x=-20..20,y=-30..30,z=-1..1
        off x=-10..10,y=-40..40,z=-1..1
        on x=-20..20,y=-30..30,z=-1..1
        off x=-10..10,y=-30..40,z=-1..1
        off x=-20..20,y=-30..30,z=-1..1";
        assert_eq!(solution_a(&input), 0);
        */
    }
}
