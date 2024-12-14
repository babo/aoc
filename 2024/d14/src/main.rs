use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::vec;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn read_line(line: &str) -> (i32, i32, i32, i32) {
    line.chars()
        .map(|x| {
            if x.is_ascii_digit() || x == '-' {
                x
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn solve_a(line: &str, w: i32, h: i32, steps: i32) -> (i32, i32) {
    let parts = read_line(line);
    let mut x = (parts.0 + parts.2 * steps) % w;
    let mut y = (parts.1 + parts.3 * steps) % h;
    if x < 0 {
        x += w;
    }
    if y < 0 {
        y += h;
    }

    (x, y)
}

fn solution_a(input: &str, w: i32, h: i32, steps: i32) -> usize {
    let t = input.lines().map(|x| solve_a(x.trim(), w, h, steps)).fold(
        HashMap::new(),
        |mut acc, (x, y)| {
            if let Some(v) = acc.get_mut(&(x, y)) {
                *v += 1;
            } else {
                acc.insert((x, y), 1);
            }
            acc
        },
    );

    let mut count = vec![0usize, 0, 0, 0];
    for y in 0..h {
        for x in 0..w {
            if x != (w / 2) && y != (h / 2) {
                if let Some(v) = t.get(&(x, y)) {
                    print!("{}", v);
                    let a = if x <= (w / 2) { 0 } else { 1 };
                    let b = if y <= (h / 2) { 0 } else { 2 };
                    count[a | b] += v;
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }
        println!()
    }
    println!("{:?}", count);
    count.iter().fold(1usize, |acc, x| acc * x)
}

fn solution_b(input: &str) -> Option<i32> {
    let (w, h) = (101_i32, 103_i32);
    let robots = input.lines().map(|x| read_line(x)).collect::<Vec<_>>();
    let mut tree: Vec<char> = vec!['.'; (w * h) as usize];
    for t in 1..300000 {
        tree.fill_with(|| '.');
        robots.iter().for_each(|parts| {
            let mut x = (parts.0 + parts.2 * t) % w;
            let mut y = (parts.1 + parts.3 * t) % h;
            if x < 0 {
                x += w;
            }
            if y < 0 {
                y += h;
            }
            tree[(y * w + x) as usize] = '#';
        });
        let count: usize = (4..80)
            .map(|l| {
                let a = l * w as usize;
                let b = a + w as usize;
                let row = tree[a..b].iter().collect::<String>();
                if row.contains("#####") {
                    1
                } else {
                    0
                }
            })
            .sum();
        if count > 2 {
            println!("Time: {}", t);
            for y in 0..h {
                for x in 0..w {
                    print!("{}", tree[(y * w + x) as usize]);
                }
                println!();
            }
            println!();
            return Some(t as i32);
        }
    }

    None
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c, 101, 103, 100);
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
        assert_eq!(solution_a(&data, 11, 7, 100), 12);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let v = solution_a(&c, 101, 103, 100);
        assert_eq!(v, 210587128);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(7286));
    }
}
