use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solve_a(line: &str) -> usize {
    let res = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .fold_while(None, |acc, x| {
            if let Some((p, pd)) = acc {
                let d = x - p;
                if d == 0 || d > 3 || d < -3 {
                    Done(Some((*x, 0)))
                } else if pd == 0 {
                    Continue(Some((*x, d)))
                } else if pd > 0 && d > 0 {
                    Continue(Some((*x, d)))
                } else if pd < 0 && d < 0 {
                    Continue(Some((*x, d)))
                } else {
                    Done(Some((*x, 0)))
                }
            } else {
                Continue(Some((*x, 0)))
            }
        });
    if res.is_done() {
        0
    } else {
        1
    }
}

fn solve_b(line: &str) -> usize {
    let correct = |a, b| a < b && a + 4 > b;

    let mut v = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let n = v.len();

    if v[n - 1] < v[0] {
        v.reverse();
    }

    let mut omit: HashSet<usize> = HashSet::new();
    let mut i = 0;
    let mut j = 0;

    while j < n {
        if omit.len() > 1 {
            break;
        }
        if j == i {
            j += 1;
            continue;
        }
        if correct(v[i], v[j]) {
            i += 1;
            j += 1;
        } else if j + 1 >= n {
            omit.insert(j);
            j += 1;
        } else if i == 0 && correct(v[j], v[j + 1]) {
            omit.insert(i);
            i = j;
            j += 1;
        } else if correct(v[i], v[j + 1]) {
            omit.insert(i);
            j += 1;
            i = j;
        } else if i > 0 && correct(v[i-1], v[j]) && correct(v[j], v[j + 1]) {
            omit.insert(i);
            i = j;
            j += 1;
        } else {
            omit.insert(j);
            omit.insert(j + 1);
        }
    }

    if omit.len() == 0 {
        return 1;
    }

    let mut brute = false;

    for j in 0..n {
        let mut w = v.clone();
        w.remove(j);

        let c = (0..n - 2)
            .map(|i| correct(w[i], w[i + 1]))
            .filter(|x| *x == false)
            .count();
        if c == 0 {
            brute = true;
            break;
        }
    }

    let mine = omit.len() == 1;

    if brute != mine {
        println!("{} {} => {}", brute, mine, line);
    }
    if mine {
        1
    } else {
        0
    }
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<usize>())
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
        assert_eq!(solution_a(&data), Some(2));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(4));
    }

    #[test]
    fn test_all_ok() {
        let data = "1 4 3 4\n81 84 83 84 87 89 91\n91 89 87 84 83 84 81\n1 1 2 3\n3 2 1 1\n1 2 2 3\n5 1 6 7\n1 3 5 6 8 9 12 9\n38 41 44 45 43 47\n67 68 70 73 71 76\n1 4 8\n1 4 8 7\n61 64 59 58 55 52 49\n";
        assert_eq!(solution_b(data), Some(data.lines().count()));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(359));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(418));
    }
}
