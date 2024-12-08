use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let w = input.trim().find("\n").unwrap() as i32;
    let h = input.trim().lines().count() as i32;
    let grid = input
        .chars()
        .filter(|&x| !x.is_ascii_whitespace())
        .collect::<Vec<char>>();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let c = grid[(y * w + x) as usize];
            if c != '.' {
                if let Some(p) = antennas.get_mut(&c) {
                    p.push((x, y));
                } else {
                    antennas.insert(c, vec![(x, y)]);
                }
            }
        }
    }
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    antennas.iter().for_each(|(_, v)| {
        v.iter().for_each(|a| {
            v.iter().for_each(|b| {
                if a != b {
                    let x = a.0 - b.0 + a.0;
                    let y = a.1 - b.1 + a.1;
                    if x >= 0 && y >= 0 && x < w && y < h {
                        antinodes.insert((x, y));
                    }
                }
            });
        });
    });

    for y in 0..h {
        for x in 0..w {
            if antinodes.contains(&(x, y)) {
                print!("#");
            } else {
                print!("{}", grid[(y * w + x) as usize]);
            }
        }
        println!();
    }

    Some(antinodes.len())
}

fn solution_b(input: &str) -> Option<usize> {
    let w = input.trim().find("\n").unwrap() as i32;
    let h = input.trim().lines().count() as i32;
    let grid = input
        .chars()
        .filter(|&x| !x.is_ascii_whitespace())
        .collect::<Vec<char>>();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let c = grid[(y * w + x) as usize];
            if c != '.' {
                if let Some(p) = antennas.get_mut(&c) {
                    p.push((x, y));
                } else {
                    antennas.insert(c, vec![(x, y)]);
                }
            }
        }
    }
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    antennas.iter().for_each(|(_, v)| {
        v.iter().for_each(|a| {
            v.iter().for_each(|b| {
                if a != b {
                    let dx = a.0 - b.0;
                    let dy = a.1 - b.1;
                    for i in 0.. {
                        let x = a.0 + dx * i;
                        let y = a.1 + dy * i;
                        if x >= 0 && y >= 0 && x < w && y < h {
                            antinodes.insert((x, y));
                        } else {
                            break;
                        }
                    }
                }
            })
        });
    });

    for y in 0..h {
        for x in 0..w {
            if antinodes.contains(&(x, y)) {
                print!("#");
            } else {
                print!("{}", grid[(y * w + x) as usize]);
            }
        }
        println!();
    }

    Some(antinodes.len())
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
        assert_eq!(solution_a(&data), Some(14));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(34));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(396));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1200));
    }
}
