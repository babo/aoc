use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let table = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let count = table
        .iter()
        .fold((HashSet::<usize>::new(), 0), |(mut acc, mut count), row| {
            if acc.is_empty() {
                row.find('S').map(|p| {
                    acc.insert(p);
                });
            } else {
                let (rem, add) = row.chars().enumerate().fold(
                    (Vec::new(), Vec::new()),
                    |(mut rem, mut add), (p, c)| {
                        if c == '^' && acc.contains(&p) {
                            rem.push(p);
                            add.push(p - 1);
                            add.push(p + 1);
                            count += 1;
                        }
                        (rem, add)
                    },
                );
                rem.iter().for_each(|p| {
                    acc.remove(p);
                });
                add.iter().for_each(|p| {
                    acc.insert(*p);
                });
                println!("{} {}", acc.len(), count);
            }
            (acc, count)
        })
        .1;
    Some(count)
}

fn solution_b(input: &str) -> Option<usize> {
    let table = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let beams = table
        .iter()
        .fold(HashMap::<usize, usize>::new(), |prev, row| {
            if prev.is_empty() {
                return row.find('S').map_or(prev, |p| {
                    let mut s = HashMap::new();
                    s.insert(p, 1);
                    s
                });
            } else {
                let (rem, add) = row.chars().enumerate().fold(
                    (Vec::new(), Vec::new()),
                    |(mut rem, mut add), (p, c)| {
                        if c == '^' && prev.contains_key(&p) {
                            let r = *prev.get(&p).unwrap();
                            rem.push(p);
                            add.push((p - 1, r));
                            add.push((p + 1, r));
                        }
                        (rem, add)
                    },
                );
                let mut acc = prev.clone();
                rem.iter().for_each(|p| {
                    println!("remove {p}");
                    acc.remove(p);
                });
                add.iter().for_each(|(p, v)| {
                    println!("add {p}");
                    if acc.contains_key(p) {
                        acc.get_mut(p).map(|p| *p += *v);
                    } else {
                        acc.insert(*p, *v);
                    }
                });
                println!("{:?} => {:?}", prev, acc);
                acc
            }
        });
    Some(beams.values().sum())
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
        assert_eq!(solution_a(&data), Some(21));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(40));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1615));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(43560947406326));
    }
}
