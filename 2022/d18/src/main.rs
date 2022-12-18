use std::collections::HashSet;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn read_coords(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|xyx| {
            xyx.split(',')
                .map(|c| 1 + usize::from_str_radix(c, 10).unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn solution_a(input: &str) -> Option<usize> {
    let cubes = read_coords(input);
    let n = 6 * cubes.len();
    let mut sides = HashSet::new();

    cubes.iter().for_each(|cube| {
        let (x, y, z) = *cube;
        sides.insert((x, y, z, x + 1, y + 1, z));
        sides.insert((x, y, z, x + 1, y, z + 1));
        sides.insert((x, y + 1, z, x + 1, y + 1, z + 1));
        sides.insert((x, y, z + 1, x + 1, y + 1, z + 1));
        sides.insert((x, y, z, x, y + 1, z + 1));
        sides.insert((x + 1, y, z, x + 1, y + 1, z + 1));
    });
    Some(n - 2 * (n - sides.len()))
}

fn solution_b(input: &str) -> Option<usize> {
    let cubes = read_coords(input);
    let origin = HashSet::<(usize, usize, usize)>::from_iter(cubes.iter().map(|c| *c));

    let max_x = cubes.iter().map(|a| a.0).max().unwrap() + 1;
    let max_y = cubes.iter().map(|a| a.1).max().unwrap() + 1;
    let max_z = cubes.iter().map(|a| a.2).max().unwrap() + 1;

    let mut visit = Vec::new();
    for z in 0..=max_z {
        for y in 0..=max_y {
            visit.push((0, y, z));
            visit.push((max_x, y, z));
        }
    }
    for z in 0..=max_z {
        for x in 0..=max_x {
            visit.push((x, 0, z));
            visit.push((x, max_y, z));
        }
    }
    for y in 0..=max_y {
        for x in 0..=max_x {
            visit.push((x, y, 0));
            visit.push((x, y, max_z));
        }
    }

    let mut air = HashSet::new();
    let mut outside = HashSet::<(usize, usize, usize)>::from_iter(visit.iter().map(|c| *c));
    let mut next = Vec::<(usize, usize, usize)>::new();

    let mut update =
        |coord, outside: &HashSet<(usize, usize, usize)>, next: &mut Vec<(usize, usize, usize)>| {
            if !outside.contains(&coord) {
                if origin.contains(&coord) {
                    air.insert(coord);
                } else {
                    next.push(coord);
                }
            }
        };

    while !visit.is_empty() {
        println!("Visit {}", visit.len());
        visit.iter().for_each(|c| {
            let (x, y, z) = *c;
            if x != 0 {
                update((x - 1, y, z), &outside, &mut next);
            }
            if x + 1 <= max_x {
                update((x + 1, y, z), &outside, &mut next);
            }
            if y != 0 {
                update((x, y - 1, z), &outside, &mut next);
            }
            if y + 1 <= max_y {
                update((x, y + 1, z), &outside, &mut next);
            }
            if z != 0 {
                update((x, y, z - 1), &outside, &mut next);
            }
            if z + 1 <= max_z {
                update((x, y, z + 1), &outside, &mut next);
            }
        });
        outside.extend(next.iter().map(|c| *c));
        visit.clear();
        visit.append(&mut next);
    }

    println!("Found {} from {}", air.len(), origin.len());
    Some(
        air.iter()
            .map(|c| {
                let (x, y, z) = *c;
                let mut n = 0;
                if outside.contains(&(x + 1, y, z)) {
                    n += 1;
                }
                if outside.contains(&(x - 1, y, z)) {
                    n += 1;
                }
                if outside.contains(&(x, y + 1, z)) {
                    n += 1;
                }
                if outside.contains(&(x, y - 1, z)) {
                    n += 1;
                }
                if outside.contains(&(x, y, z + 1)) {
                    n += 1;
                }
                if outside.contains(&(x, y, z - 1)) {
                    n += 1;
                }
                n
            })
            .sum(),
    )
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
    fn test_mini_a() {
        let data = "1,1,1\n2,1,1";
        assert_eq!(solution_a(&data), Some(10));
    }

    #[test]
    fn test_mini_b() {
        let data = read_to_string("./mini.txt").unwrap();
        assert_eq!(solution_b(&data), Some(18));
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(64));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(58));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(4628));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(2582));
    }
}
