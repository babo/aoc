use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

type Brick = ((u16, u16, u16), (u16, u16, u16));

fn brick_reader(line: &str) -> Brick {
    line.split('~')
        .map(|xyz| {
            xyz.split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn get_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(brick_reader)
        .collect_vec()
}

fn as_char(x: usize) -> char {
    std::char::from_u32(65 + (x % 26) as u32).unwrap()
}

fn solution_a(input: &str) -> Option<usize> {
    let bricks = get_bricks(input);
    let bricks = bricks.iter().sorted_by_key(|b| b.0 .2).collect_vec();
    let mut single: HashSet<usize> = HashSet::new();
    let mut shared: HashSet<usize> = HashSet::new();
    let mut required: HashSet<usize> = HashSet::new();
    let mut xy: Vec<Option<(usize, u16)>> = std::iter::repeat(None).take(100).collect_vec();
    let at = |x: u16, y: u16| (x + y * 10) as usize;
    bricks.iter().enumerate().for_each(|(i, b)| {
        single.insert(i);
        println!("{i} {} {:?}", as_char(i), b);

        let max_z = (b.0 .0..b.1 .0 + 1)
            .map(|x| {
                (b.0 .1..b.1 .1 + 1)
                    .map(|y| xy[at(x, y)].map_or(0, |p| p.1))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        let new_z = max_z + b.1 .2 - b.0 .2 + 1;
        println!("Max Z: {max_z} will be {}", new_z);
        let mut removed: HashSet<usize> = HashSet::new();
        for x in b.0 .0..b.1 .0 + 1 {
            for y in b.0 .1..b.1 .1 + 1 {
                if let Some((j, xyz)) = xy[at(x, y)] {
                    if xyz == max_z {
                        println!("{} is supporting {}", as_char(j), as_char(i));
                        single.remove(&j);
                        removed.insert(j);
                    }
                }
                if let Some(p) = xy.get_mut(at(x, y)) {
                    *p = Some((i, new_z));
                }
            }
        }
        if removed.len() > 1 {
            println!(
                "Brick {} sitting on: {}",
                as_char(i),
                removed.iter().sorted().map(|x| as_char(*x)).join(", ")
            );
            removed.iter().for_each(|x| {
                shared.insert(*x);
            });
        } else {
            removed.iter().for_each(|j| {
                println!("Brick {} is required for {}", as_char(*j), as_char(i));
                required.insert(*j);
            });
        }
    });
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", xy[at(x, y)].map_or('.', |x| as_char(x.0)));
        }
        println!()
    }
    let diff = shared.difference(&required).count();
    println!(
        "Single: {}",
        single.iter().sorted().map(|x| as_char(*x)).join(", ")
    );
    println!(
        "Shared: {}",
        shared.iter().sorted().map(|x| as_char(*x)).join(", ")
    );
    Some(single.len() + diff)
}

fn calc_remove(
    index: &usize,
    depend: &HashMap<usize, Vec<usize>>,
    sit_on: &HashMap<usize, Vec<usize>>,
) -> usize {
    let mut open: HashSet<usize> = HashSet::new();
    let mut tbr: HashSet<usize> = HashSet::new();
    tbr.insert(*index);
    open.insert(*index);

    println!();
    println!("calc_remove: {index}");
    while !open.is_empty() {
        let i = *open.iter().next().unwrap();

        println!("Checking: {i}");
        if depend.contains_key(&i) {
            println!("Has dependents: {:?}", depend[&i]);
            depend[&i].iter().for_each(|j| {
                let others = sit_on[j]
                    .iter()
                    .filter(|x| **x != i && !tbr.contains(x))
                    .count();
                if others == 0 {
                    tbr.insert(*j);
                    open.insert(*j);
                }
            });
        } else if i != *index {
            tbr.insert(i);
        }

        open.remove(&i);
    }

    println!("tbr: {:?}", tbr);
    tbr.len() - 1
}

fn solution_b(input: &str) -> Option<usize> {
    let bricks = get_bricks(input);
    let bricks = bricks.iter().sorted_by_key(|b| b.0 .2).collect_vec();
    let mut toplevel: HashSet<usize> = HashSet::new();
    let mut shared: HashSet<usize> = HashSet::new();
    let mut single: HashSet<usize> = HashSet::new();
    let mut depend: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut sit_on: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut xy: Vec<Option<(usize, u16)>> = std::iter::repeat(None).take(100).collect_vec();
    let at = |x: u16, y: u16| (x + y * 10) as usize;

    bricks.iter().enumerate().for_each(|(i, b)| {
        println!("{i} {:?}", b);
        toplevel.insert(i);

        let max_z = (b.0 .0..b.1 .0 + 1)
            .map(|x| {
                (b.0 .1..b.1 .1 + 1)
                    .map(|y| xy[at(x, y)].map_or(0, |p| p.1))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        let new_z = max_z + b.1 .2 - b.0 .2 + 1;
        let mut support: HashSet<usize> = HashSet::new();
        for x in b.0 .0..b.1 .0 + 1 {
            for y in b.0 .1..b.1 .1 + 1 {
                if let Some((j, xyz)) = xy[at(x, y)] {
                    if xyz == max_z {
                        println!("{} is supporting {}", j, i);
                        toplevel.remove(&j);
                        support.insert(j);
                    }
                }
                if let Some(p) = xy.get_mut(at(x, y)) {
                    *p = Some((i, new_z));
                }
            }
        }
        sit_on.insert(i, vec![]);
        support.iter().for_each(|j| {
            if let Some(p) = sit_on.get_mut(&i) {
                p.push(*j);
            }
            if depend.contains_key(j) {
                if let Some(p) = depend.get_mut(j) {
                    p.push(i);
                }
            } else {
                depend.insert(*j, vec![i]);
            }
        });
        if support.len() > 1 {
            println!(
                "Brick {} sitting on: {}",
                i,
                support.iter().sorted().join(", ")
            );
            support.iter().for_each(|x| {
                shared.insert(*x);
            });
        } else {
            support.iter().for_each(|j| {
                println!("Brick {} is required for {}", j, i);
                single.insert(*j);
            });
        }
    });

    println!("depend:{:?}", depend);
    println!("sit_on:{:?}", sit_on);
    println!("shared:{:?}", shared);
    println!("single:{:?}", single);
    println!("toplevel:{:?}", toplevel);

    Some(
        single
            .iter()
            .map(|i| calc_remove(i, &depend, &sit_on))
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
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(5));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(7));
    }

    /*
        depend:{1: [3], 4: [5, 6], 6: [7], 7: [8], 2: [4], 0: [1, 2], 5: [7], 3: [4]}
        sit_on:{7: [5, 6], 4: [2, 3], 3: [1], 0: [], 2: [0], 6: [4], 1: [0], 8: [7], 5: [4]}
        shared:{3, 2, 5, 6}
        single:{0, 4, 1, 7}
    */

    #[test]
    fn test_tricky() {
        let data = "
        1,0,1~1,3,1
        1,0,2~1,0,2
        1,3,3~1,3,4
        1,0,5~1,0,5
        1,0,6~1,3,6
        1,0,7~1,0,7
        1,1,8~1,1,8
        1,0,9~1,3,9
        1,3,10~1,3,10
        ";
        assert_eq!(solution_b(&data), Some(14));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(407));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(59266));
    }
}
