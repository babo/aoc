use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn move_knot(hp: (i32, i32), tp: (i32, i32)) -> (i32, i32) {
    match (hp.0 - tp.0, hp.1 - tp.1) {
        (0, y) if y.abs() > 1 => (tp.0, tp.1 + y / 2),
        (x, 0) if x.abs() > 1 => (tp.0 + x / 2, tp.1),
        (x, y) if x.abs() > 1 || y.abs() > 1 => (
            tp.0 + if x >= 0 { 1 } else { -1 },
            tp.1 + if y >= 0 { 1 } else { -1 },
        ),
        _ => tp,
    }
}

fn move_head(hp: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        'U' => (hp.0, hp.1 + 1),
        'D' => (hp.0, hp.1 - 1),
        'L' => (hp.0 - 1, hp.1),
        'R' => (hp.0 + 1, hp.1),
        _ => unreachable!("What?"),
    }
}

fn solution_a(input: &str) -> usize {
    let mut hp = (0i32, 0i32);
    let mut tp = (0, 0);
    let mut pos = HashSet::<(i32, i32)>::new();
    pos.insert(tp);

    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .for_each(|inst| {
            let dir = inst.chars().nth(0).unwrap();
            let dist = inst
                .get(2..)
                .map(|d| i32::from_str_radix(d, 10).unwrap())
                .unwrap();

            for _ in 0..dist {
                hp = move_head(hp, dir);
                tp = move_knot(hp, tp);
                pos.insert(tp);
            }
        });
    pos.len()
}

fn solution_b(input: &str) -> usize {
    let mut knots = [(0i32, 0i32); 10];
    let mut pos = HashSet::<(i32, i32)>::new();
    pos.insert(knots[9]);

    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .for_each(|inst| {
            let dir = inst.chars().nth(0).unwrap();
            let dist = inst
                .get(2..)
                .map(|d| i32::from_str_radix(d, 10).unwrap())
                .unwrap();

            for _ in 0..dist {
                knots[0] = move_head(knots[0], dir);
                for i in 0..9 {
                    knots[i + 1] = move_knot(knots[i], knots[i + 1]);
                }
                pos.insert(knots[9]);
            }
        });
    pos.len()
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
        assert_eq!(solution_a(&data), 13);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 1);
    }

    #[test]
    fn test_complex_b() {
        let data = read_to_string("./complex.txt").unwrap();
        assert_eq!(solution_b(&data), 36);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 6044);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 2384);
    }
}
