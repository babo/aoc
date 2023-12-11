use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn expand(input: &str) -> String {
    let input = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .join("\n");
    let w = input.lines().next().unwrap().len();
    let cols: HashSet<usize> = input.lines().fold(HashSet::from_iter(0..w), |accu, line| {
        let h: HashSet<usize> =
            HashSet::from_iter(line.chars().enumerate().filter(|x| x.1 == '#').map(|x| x.0));
        let n = HashSet::from_iter(accu.difference(&h).copied());
        n
    });
    let input = input.lines().fold("".to_string(), |mut accu, line| {
        accu.push_str(line);
        if line.find('#').is_none() {
            accu.push('\n');
            accu.push_str(line);
        }
        accu.push('\n');
        accu
    });
    let input = input.lines().fold("".to_string(), |mut accu, line| {
        line.chars().enumerate().for_each(|(n, c)| {
            if cols.contains(&n) {
                accu.push('.');
            }
            accu.push(c);
        });
        accu.push('\n');
        accu
    });
    input
}

fn find_galaxies(input: &str) -> Vec<(usize, usize)> {
    let s = input
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut accu, (n, line)| {
            line.chars()
                .enumerate()
                .filter(|x| x.1 == '#')
                .for_each(|x| {
                    accu.insert((n, x.0));
                });
            accu
        });
    Vec::from_iter(s.iter().copied())
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    let mx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let my = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    mx + my
}

fn shortest_distance(galaxies: &[(usize, usize)], pair: (usize, usize)) -> usize {
    manhattan(galaxies[pair.0], galaxies[pair.1])
}

fn solve_a(input: &str) -> usize {
    let input = expand(input);
    let galaxies = find_galaxies(&input);

    (1..galaxies.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|x| shortest_distance(&galaxies, x))
        .sum()
}

fn solve_b(input: &str, factor: usize) -> usize {
    let input = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .join("\n");

    let w = input.lines().next().unwrap().len();
    let empty_cols: HashSet<usize> = input.lines().fold(HashSet::from_iter(0..w), |accu, line| {
        let h = HashSet::from_iter(line.chars().enumerate().filter(|x| x.1 == '#').map(|x| x.0));
        HashSet::from_iter(accu.difference(&h).copied())
    });
    let empty_rows: HashSet<usize> = HashSet::from_iter(input
        .lines()
        .enumerate()
        .filter(|x| !x.1.contains('#'))
        .map(|x| x.0));

    let galaxies = find_galaxies(&input);

    (1..galaxies.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|x| {
            let a = galaxies[x.0];
            let b = galaxies[x.1];
            let empties = (a.0.min(b.0)..a.0.max(b.0))
                .filter(|x| empty_rows.contains(x))
                .count()
                + (a.1.min(b.1)..a.1.max(b.1))
                    .filter(|x| empty_cols.contains(x))
                    .count();
            manhattan(a, b) + (factor - 1) * empties
        })
        .sum()
}

fn solution_a(input: &str) -> Option<usize> {
    Some(solve_a(input))
}

fn solution_b(input: &str) -> Option<usize> {
    Some(solve_b(input, 1000000))
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
    fn test_expand() {
        let data = simple().unwrap();
        let correct = "....#........
        .........#...
        #............
        .............
        .............
        ........#....
        .#...........
        ............#
        .............
        .............
        .........#...
        #....#.......
        "
        .lines()
        .map(|x| x.trim())
        .join("\n");
        assert_eq!(expand(&data), correct);
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(374));
    }

    #[test]
    fn test_two() {
        let data = simple().unwrap();
        assert_eq!(solve_b(&data, 2), 374);
    }

    #[test]
    fn test_ten() {
        let data = simple().unwrap();
        assert_eq!(solve_b(&data, 10), 1030);
    }

    #[test]
    fn test_hundred() {
        let data = simple().unwrap();
        assert_eq!(solve_b(&data, 100), 8410);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(10289334));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(649862989626));
    }
}
