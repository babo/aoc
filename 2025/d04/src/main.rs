use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let data = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect::<Vec<char>>();
    let h = input.lines().filter(|x| !x.trim().is_empty()).count() as i32;
    assert!(h > 0);
    let w = data.len() as i32 / h;
    assert!(data.len() as i32 % h == 0);

    let lookup = |a, b| {
        if a >= 0 && b >= 0 && a < w && b < h {
            data.get((a + b * w) as usize)
                .map_or(0, |c| if *c == '@' { 1 } else { 0 })
        } else {
            0
        }
    };
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            if lookup(x, y) != 1 {
                continue;
            }
            let mut c = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if !(i == 0 && j == 0) {
                        c += lookup(x + i, y + j);
                    }
                }
            }
            if c < 4 {
                count += 1;
            }
        }
    }
    Some(count)
}

fn solution_b(input: &str) -> Option<usize> {
    let data = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect::<Vec<char>>();
    let h = input.lines().filter(|x| !x.trim().is_empty()).count() as i32;
    assert!(h > 0);
    let w = data.len() as i32 / h;
    assert!(data.len() as i32 % h == 0);

    let lookup = |a, b, seen: &HashSet<i32>| {
        if a >= 0 && b >= 0 && a < w && b < h {
            let p = a + b * w;
            if seen.contains(&p) {
                return 0;
            }
            data.get(p as usize)
                .map_or(0, |c| if *c == '@' { 1 } else { 0 })
        } else {
            0
        }
    };
    let mut seen = HashSet::<i32>::new();
    let mut total = 0;
    loop {
        let mut count = 0;
        for y in 0..h {
            for x in 0..w {
                if lookup(x, y, &seen) != 1 {
                    continue;
                }
                let mut c = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if !(i == 0 && j == 0) {
                            c += lookup(x + i, y + j, &seen);
                        }
                    }
                }
                if c < 4 {
                    count += 1;
                    seen.insert(x + y * w);
                }
            }
        }
        if count == 0 {
            return Some(total);
        }
        total += count;
    }
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
        assert_eq!(solution_a(&data), Some(13));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(43));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1445));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(8317));
    }
}
