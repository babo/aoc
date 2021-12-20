use std::collections::HashMap;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Debug)]
struct Line {
    p1: (u32, u32),
    p2: (u32, u32),
}

impl Line {
    pub fn new(line: &str) -> Self {
        let coords: Vec<u32> = line
            .split("->")
            .map(|xy| {
                xy.split(",")
                    .map(|v| u32::from_str_radix(v.trim(), 10).unwrap())
            })
            .flatten()
            .collect();
        assert_eq!(coords.len(), 4);
        let p1 = (coords[0], coords[1]);
        let p2 = (coords[2], coords[3]);
        Line { p1, p2 }
    }

    pub fn horvert(&self) -> bool {
        self.p1.0 == self.p2.0 || self.p1.1 == self.p2.1
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let lines: Vec<Line> = input
        .lines()
        .map(|line| Line::new(line))
        .filter(|x| x.horvert())
        .collect();
    let mut count: HashMap<(u32, u32), usize> = HashMap::new();

    lines.iter().for_each(|l| {
        if l.p1.1 == l.p2.1 {
            let xs = if l.p1.0 > l.p2.0 {
                (l.p2.0, l.p1.0)
            } else {
                (l.p1.0, l.p2.0)
            };
            for x in xs.0..=xs.1 {
                let k = (x, l.p1.1);
                if count.contains_key(&k) {
                    count.get_mut(&k).map(|x| *x += 1);
                } else {
                    count.insert(k, 1);
                }
            }
        } else if l.p1.0 == l.p2.0 {
            let ys = if l.p1.1 > l.p2.1 {
                (l.p2.1, l.p1.1)
            } else {
                (l.p1.1, l.p2.1)
            };
            for y in ys.0..=ys.1 {
                let k = (l.p1.0, y);
                if count.contains_key(&k) {
                    count.get_mut(&k).map(|x| *x += 1);
                } else {
                    count.insert(k, 1);
                }
            }
        }
    });
    Some(count.values().filter(|x| **x > 1).count())
}

fn solution_b(input: &str) -> Option<usize> {
    let lines: Vec<Line> = input.lines().map(|line| Line::new(line)).collect();
    let mut count: HashMap<(u32, u32), usize> = HashMap::new();

    lines.iter().for_each(|l| {
        if l.p1.1 == l.p2.1 {
            let xs = if l.p1.0 > l.p2.0 {
                (l.p2.0, l.p1.0)
            } else {
                (l.p1.0, l.p2.0)
            };
            for x in xs.0..=xs.1 {
                let k = (x, l.p1.1);
                if count.contains_key(&k) {
                    count.get_mut(&k).map(|x| *x += 1);
                } else {
                    count.insert(k, 1);
                }
            }
        } else if l.p1.0 == l.p2.0 {
            let ys = if l.p1.1 > l.p2.1 {
                (l.p2.1, l.p1.1)
            } else {
                (l.p1.1, l.p2.1)
            };
            for y in ys.0..=ys.1 {
                let k = (l.p1.0, y);
                if count.contains_key(&k) {
                    count.get_mut(&k).map(|x| *x += 1);
                } else {
                    count.insert(k, 1);
                }
            }
        } else {
            let xs = if l.p2.0 > l.p1.0 {
                (l.p1.0, l.p2.0, l.p1.1, l.p2.1 > l.p1.1)
            } else {
                (l.p2.0, l.p1.0, l.p2.1, l.p1.1 > l.p2.1)
            };
            let mut y = xs.2;
            for x in xs.0..=xs.1 {
                let k = (x, y);
                if count.contains_key(&k) {
                    count.get_mut(&k).map(|x| *x += 1);
                } else {
                    count.insert(k, 1);
                }
                if xs.3 {
                    y += 1;
                } else {
                    if y > 0 {
                        y -= 1;
                    }
                }
            }
        }
    });
    Some(count.values().filter(|x| **x > 1).count())
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

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_simple_a() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_a(&data), Some(5));
    }

    #[test]
    fn test_simple_b() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_b(&data), Some(12));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(8060));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(21577));
    }
}
