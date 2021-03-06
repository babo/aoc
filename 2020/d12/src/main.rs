use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn md(ew: i32, ns: i32) -> i32 {
    ew.abs() + ns.abs()
}

fn solution_a(content: &str) -> i32 {
    process_instructions_a(content)
}

fn solution_b(content: &str) -> i32 {
    process_instructions_b(content)
}

fn process_instructions_a(input: &str) -> i32 {
    let mut east = 0i32;
    let mut north = 0i32;
    let mut dir = 0i16;

    for line in input.split_whitespace() {
        let nv = line.get(1..).unwrap().parse::<i32>().unwrap();
        let fc = if line.chars().nth(0) == Some('F') {
            match dir {
                0 => Some('E'),
                3 => Some('N'),
                2 => Some('W'),
                1 => Some('S'),
                _ => panic!("What? {}", dir),
            }
        } else {
            line.chars().nth(0)
        };

        match fc {
            Some('N') => north += nv,
            Some('S') => north -= nv,
            Some('E') => east += nv,
            Some('W') => east -= nv,
            Some('L') => {
                for r in 0..4 {
                    if r * 90 >= nv {
                        break;
                    }
                    dir = (4 + (dir - 1)) % 4;
                }
            }
            Some('R') => {
                for r in 0..4 {
                    if r * 90 >= nv {
                        break;
                    }
                    dir = (dir + 1) % 4
                }
            }
            _ => panic!("What? {}", line),
        }
    }
    md(east, north)
}

fn process_instructions_b(input: &str) -> i32 {
    let mut wp_e = 10i32;
    let mut wp_n = 1i32;
    let mut east = 0i32;
    let mut north = 0i32;

    for line in input.split_whitespace() {
        let nv = line.get(1..).unwrap().parse::<i32>().unwrap();
        match line.chars().nth(0) {
            Some('N') => wp_n += nv,
            Some('S') => wp_n -= nv,
            Some('E') => wp_e += nv,
            Some('W') => wp_e -= nv,
            Some('L') => {
                for r in 0..4 {
                    if r * 90 >= nv {
                        break;
                    }
                    let tmp = wp_e;
                    wp_e = -wp_n;
                    wp_n = tmp;
                }
            }
            Some('R') => {
                for r in 0..4 {
                    if r * 90 >= nv {
                        break;
                    }
                    let tmp = wp_e;
                    wp_e = wp_n;
                    wp_n = -tmp;
                }
            }
            Some('F') => {
                east += nv * wp_e;
                north += nv * wp_n;
            }
            _ => panic!("What? {}", line),
        }
    }
    md(east, north)
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {}", a);
    println!("Step B: {}", b);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_simple_a() {
        let sample = "F10
N3
F7
R90
F11";
        let a = solution_a(sample);
        assert_eq!(a, 25);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let a = solution_a(&c);
        assert_eq!(a, 1186);
    }

    #[test]
    fn test_simple_b() {
        let sample = "F10
N3
F7
R90
F11";
        let a = solution_b(sample);
        assert_eq!(a, 286);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let a = solution_b(&c);
        assert_eq!(a, 1186);
    }
}
