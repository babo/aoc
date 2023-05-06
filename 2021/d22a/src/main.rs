use std::collections::HashSet;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

fn count_on(cuboids: &Vec<Cuboid>) -> usize {
    let xs = cuboids
        .iter()
        .enumerate()
        .flat_map(|(i, cube)| [(cube.p[0], i, true), (cube.p[1], i, false)])
        .sorted_by_key(|x| x.0)
        .collect_vec();
    let ys = cuboids
        .iter()
        .enumerate()
        .flat_map(|(i, cube)| [(cube.p[2], i, true), (cube.p[3], i, false)])
        .sorted_by_key(|x| x.0)
        .collect_vec();
    let zs = cuboids
        .iter()
        .enumerate()
        .flat_map(|(i, cube)| [(cube.p[4], i, true), (cube.p[5], i, false)])
        .sorted_by_key(|x| x.0)
        .collect_vec();

    let cuts_x = cut(&cuboids, &xs, 0);
    let mut index: Option<usize> = None;
    let mut prev: Option<Cuboid> = None;
    let mut step_1: Vec<Cuboid> = Vec::new();
    cuts_x.iter().for_each(|x| {
        if prev.is_none() || index.map_or(true, |p| p != x.0) {
            prev.map(|p| step_1.push(p));
            prev = Some(cuboids[x.0].clone());
            index = Some(x.0);
        }
        prev.map(|p| {
            let (a, b) = p.cut_at(0, x.1);
            step_1.push(b);
            prev = Some(a);
        });
        println!(
            "x {} {} -> {} {}",
            x.0, x.1, cuboids[x.0].p[0], cuboids[x.0].p[1]
        );
    });
    prev.map(|p| step_1.push(p));
    let step_1 = step_1;

    index = None;
    prev = None;
    let cuts_y = cut(&step_1, &ys, 2);
    let mut step_2: Vec<Cuboid> = Vec::new();
    cuts_y.iter().for_each(|x| {
        if prev.is_none() || index.map_or(true, |p| p != x.0) {
            prev.map(|p| step_2.push(p));
            prev = Some(step_1[x.0].clone());
            index = Some(x.0);
        }
        prev.map(|p| {
            let (a, b) = p.cut_at(2, x.1);
            step_2.push(b);
            prev = Some(a);
        });

        println!(
            "y {} {} -> {} {}",
            x.0, x.1, step_1[x.0].p[2], step_1[x.0].p[3]
        );
    });
    prev.map(|p| step_2.push(p));
    let step_2 = step_2;

    index = None;
    prev = None;
    let cuts_z = cut(&step_2, &zs, 4);
    let mut step_3: Vec<Cuboid> = Vec::new();
    cuts_z.iter().for_each(|x| {
        if prev.is_none() || index.map_or(true, |p| p != x.0) {
            prev.map(|p| step_3.push(p));
            prev = Some(step_2[x.0].clone());
            index = Some(x.0);
        }
        prev.map(|p| {
            let (a, b) = p.cut_at(4, x.1);
            step_3.push(b);
            prev = Some(a);
        });

        println!(
            "z {} {} -> {} {}",
            x.0, x.1, step_2[x.0].p[4], step_2[x.0].p[5]
        );
    });
    0
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
        assert_eq!(cuboids.len(), 421);
    }

    #[test]
    fn test_volume() {
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

        let input = "on x=-20..20,y=-30..30,z=-1..1
        off x=-10..10,y=-40..40,z=-1..1
        on x=-20..20,y=-30..30,z=-1..1
        off x=-10..10,y=-30..40,z=-1..1
        off x=-20..20,y=-30..30,z=-1..1";
        assert_eq!(solution_a(&input), 0);
    }
}
