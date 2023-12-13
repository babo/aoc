use intersection::hash_set;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn swap(data: &str) -> String {
    let w = data.lines().next().unwrap().len();
    (0..w).fold("".to_string(), |accu, x| {
        let mut line = data.lines().fold(accu, |mut acc, line| {
            acc.push(line.chars().nth(x).unwrap());
            acc
        });
        line.push('\n');
        line
    })
}

fn mirror(data: &str) -> Vec<usize> {
    let w = data.lines().next().unwrap().len();
    let vertical = data.lines().fold(
        HashSet::from_iter(0..w - 1),
        |accu: HashSet<usize>, line| {
            let potential = accu
                .iter()
                .filter(|m| {
                    let m = **m;
                    (0..w)
                        .map(|d| {
                            if m + 1 + d < w && m >= d {
                                let a = m + 1 + d;
                                let b = m - d;
                                line.get(a..a + 1) == line.get(b..b + 1)
                            } else {
                                true
                            }
                        })
                        .all(|x| x)
                })
                .copied()
                .collect_vec();
            let potential = HashSet::from_iter(potential.iter().copied());
            hash_set::intersection([accu, potential])
        },
    );
    vertical.iter().copied().collect_vec()
}

fn solve_a(data: &str, minmax: bool) -> Vec<usize> {
    let mut a = mirror(data).iter().map(|x| x + 1).collect_vec();
    a.extend(mirror(&swap(data)).iter().map(|x| (x + 1) * 100));
    a.sort();
    if minmax {
        a
    } else {
        a.iter().rev().copied().collect_vec()
    }
}

fn solve_b(data: &str) -> Option<usize> {
    //println!("data:\n{data}");
    //println!();
    let w = data.lines().next().map(|x| x.len()).unwrap();
    let h = data.lines().count();
    let orig = *solve_a(data, true).first().unwrap();
    //println!("orig: {orig}");
    let mut datav = data.chars().collect_vec();
    for c in 0..w {
        for r in 0..h {
            let index = r * (w + 1) + c;
            let before = *datav.get(index).unwrap();
            assert_ne!(before, '\n');

            if let Some(c) = datav.get_mut(index) {
                *c = if before == '.' { '#' } else { '.' };
            }
            let modified = datav.iter().join("");

            let res = solve_a(&modified, false);
            if let Some(c) = datav.get_mut(index) {
                *c = before;
            }

            if let Some(found) = res.iter().find(|x| **x != orig) {
                //println!("{c} {r} {before} {:?} {:?}", orig, res);
                return Some(*found);
            }
        }
    }
    None
}

fn solution_a(input: &str) -> Option<usize> {
    let a = input.lines().fold((0, "".to_string()), |accu, x| {
        if x.trim().is_empty() {
            let m = accu.0
                + if let Some(r) = solve_a(&accu.1, true).first() {
                    *r
                } else {
                    0
                };
            (m, "".to_string())
        } else {
            let mut s = accu.1;
            if !s.is_empty() {
                s.push('\n');
            }
            s.push_str(x.trim());
            (accu.0, s)
        }
    });
    Some(
        a.0 + if a.1.is_empty() {
            0
        } else if let Some(r) = solve_a(&a.1, true).first() {
            *r
        } else {
            0
        },
    )
}

fn solution_b(input: &str) -> Option<usize> {
    let a = input.lines().fold((0, "".to_string()), |accu, x| {
        if x.trim().is_empty() {
            let m = accu.0 + if let Some(r) = solve_b(&accu.1) { r } else { 0 };
            (m, "".to_string())
        } else {
            let mut s = accu.1;
            if !s.is_empty() {
                s.push('\n');
            }
            s.push_str(x.trim());
            (accu.0, s)
        }
    });
    Some(
        a.0 + if a.1.is_empty() {
            0
        } else if let Some(r) = solve_b(&a.1) {
            r
        } else {
            0
        },
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
    fn test_swap() {
        assert_eq!(swap("abc\n123"), "a1\nb2\nc3\n");
        assert_eq!(swap("ab"), "a\nb\n");
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(405));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(400));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(32723));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let r = solution_b(&c).unwrap();
        println!("{r}");
        assert!(r < 44143);
        assert!(r > 32257);
        assert_eq!(Some(r), Some(34536));
    }
}
