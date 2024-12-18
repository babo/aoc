use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn heuristic((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

// A* algorithm
fn astar(
    start: (isize, isize),
    end: (isize, isize),
    mc: &dyn Fn(isize, isize) -> char,
) -> Option<(Vec<(isize, isize)>, isize)> {
    let mut open_set = HashSet::new();
    let mut visited = HashMap::new();
    let mut f_score = HashMap::new();
    let mut g_score = HashMap::new();
    f_score.insert(start, heuristic(start, end));
    g_score.insert(start, 0);
    open_set.insert(start);

    while !open_set.is_empty() {
        let ordered = open_set
            .iter()
            .sorted_by(|a, b| {
                f_score
                    .get(a)
                    .unwrap_or(&isize::MAX)
                    .cmp(f_score.get(b).unwrap_or(&isize::MAX))
            })
            .map(|(a, b)| (*a, *b))
            .collect_vec();
        let pos = *ordered.first().unwrap();
        let cost = *g_score.get(&pos).unwrap();

        if pos == end {
            let mut opt_path = vec![pos];
            let mut current = pos;
            while current != start {
                if let Some(p) = visited.get(&current) {
                    current = *p;
                    opt_path.push(current);
                } else {
                    break;
                }
            }
            opt_path.reverse();
            return Some((opt_path, cost));
        }
        open_set.remove(&pos);
        let (x, y) = pos;
        let mut neighbors = vec![];
        if mc(x, y - 1) != '#' {
            neighbors.push((x, y - 1));
        }
        if mc(x, y + 1) != '#' {
            neighbors.push((x, y + 1));
        }
        if mc(x + 1, y) != '#' {
            neighbors.push((x + 1, y));
        }
        if mc(x - 1, y) != '#' {
            neighbors.push((x - 1, y));
        }

        neighbors.iter().for_each(|npos| {
            let tentative_gscore = g_score.get(&pos).unwrap() + 1;
            if g_score
                .get(npos)
                .map(|g_neighbor| tentative_gscore < *g_neighbor)
                .unwrap_or(true)
            {
                open_set.insert(*npos);
                f_score.insert(*npos, tentative_gscore + heuristic(*npos, end));
                g_score.insert(*npos, tentative_gscore);
                visited.insert(*npos, pos);
            }
        });
    }

    None
}

fn solution_a(input: &str) -> Option<isize> {
    let data = input
        .lines()
        .map(|x| {
            x.trim()
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple::<(u16, u16)>()
                .unwrap()
        })
        .collect_vec();
    let w = data.iter().map(|x| x.0).max().unwrap() as isize + 1;

    let steps = if w == 7 { 12 } else { 1024 };
    let map: HashSet<&(u16, u16)> = HashSet::from_iter(data.iter().take(steps));

    astar((0, 0), (w - 1, w - 1), &|x, y| {
        if x < 0 || y < 0 || x >= w || y >= w || map.contains(&(x as u16, y as u16)) {
            '#'
        } else {
            '.'
        }
    })
    .map(|(_, cost)| cost)
}

fn solution_b(input: &str) -> Option<(u16, u16)> {
    let data = input
        .lines()
        .map(|x| {
            x.trim()
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple::<(u16, u16)>()
                .unwrap()
        })
        .collect_vec();
    let w = data.iter().map(|x| x.0).max().unwrap() as isize + 1;
    let map: HashMap<&(u16, u16), usize> =
        HashMap::from_iter(data.iter().enumerate().map(|(i, x)| (x, i)));

    let mut good = if w == 7 { 12usize } else { 1024usize };
    let mut bad = data.len();
    while good + 1 < bad {
        let steps = (good + bad) / 2;
        println!("{} {} {}", good, bad, steps);
        let res = astar((0, 0), (w - 1, w - 1), &|x, y| {
            if x < 0
                || y < 0
                || x >= w
                || y >= w
                || map
                    .get(&(x as u16, y as u16))
                    .map(|&v| v < steps)
                    .unwrap_or(false)
            {
                '#'
            } else {
                '.'
            }
        });
        if res.is_some() {
            good = steps;
        } else {
            bad = steps;
        }
    }
    data.get(good).copied()
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
        assert_eq!(solution_a(&data), Some(22));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some((6, 1)));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(262));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some((22, 20)));
    }
}
