use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let w = input.trim().find('\n').unwrap();
    let data = input
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<Vec<_>>();
    let h = data.len() / w;
    let ptr = |x, y| x + y * w;
    let lookup = |x: i32, y: i32| {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x < w && y < h {
            Some(data[ptr(x, y)])
        } else {
            None
        }
    };

    let mut index = 0;
    let mut group: HashMap<(usize, usize), usize> = HashMap::new();
    let mut fence_area: HashMap<usize, (usize, usize)> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if !group.contains_key(&(x, y)) {
                index += 1;
                let c = data[ptr(x, y)];
                let mut check = HashSet::new();
                check.insert((x, y));

                let mut area = 0;
                let mut fence = 0;
                while !check.is_empty() {
                    let (x, y) = *check.iter().next().unwrap();
                    check.remove(&(x, y));
                    if group.contains_key(&(x, y)) {
                        continue;
                    }
                    area += 1;
                    fence += 4;
                    group.insert((x, y), index);
                    if lookup(x as i32 - 1, y as i32).unwrap_or(' ') == c {
                        check.insert((x - 1, y));
                        fence -= 1;
                    }
                    if lookup(x as i32 + 1, y as i32).unwrap_or(' ') == c {
                        check.insert((x + 1, y));
                        fence -= 1;
                    }
                    if lookup(x as i32, y as i32 - 1).unwrap_or(' ') == c {
                        check.insert((x, y - 1));
                        fence -= 1;
                    }
                    if lookup(x as i32, y as i32 + 1).unwrap_or(' ') == c {
                        check.insert((x, y + 1));
                        fence -= 1;
                    }
                }
                fence_area.insert(index, (area, fence));
            }
        }
    }

    Some(fence_area.values().map(|(a, f)| a * f).sum())
}

fn solution_b(input: &str) -> Option<usize> {
    let w = input.trim().find('\n').unwrap();
    let data = input
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<Vec<_>>();
    let h = data.len() / w;
    let ptr = |x, y| x + y * w;
    let lookup = |x: i32, y: i32| {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x < w && y < h {
            Some(data[ptr(x, y)])
        } else {
            None
        }
    };

    let mut index = 0;
    let mut group: HashMap<(usize, usize), usize> = HashMap::new();
    let mut fence_area: HashMap<usize, (usize, usize)> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if !group.contains_key(&(x, y)) {
                index += 1;
                let c = data[ptr(x, y)];
                let mut check = HashSet::new();
                check.insert((x, y));

                let mut area = 0;
                let mut fence: HashSet<(usize, usize, Direction)> = HashSet::new();
                while !check.is_empty() {
                    let (x, y) = *check.iter().next().unwrap();
                    check.remove(&(x, y));
                    if group.contains_key(&(x, y)) {
                        continue;
                    }
                    area += 1;
                    group.insert((x, y), index);
                    if lookup(x as i32 - 1, y as i32).unwrap_or(' ') == c {
                        check.insert((x - 1, y));
                    } else {
                        fence.insert((x, y, Direction::Left));
                    }
                    if lookup(x as i32 + 1, y as i32).unwrap_or(' ') == c {
                        check.insert((x + 1, y));
                    } else {
                        fence.insert((x, y, Direction::Right));
                    }
                    if lookup(x as i32, y as i32 - 1).unwrap_or(' ') == c {
                        check.insert((x, y - 1));
                    } else {
                        fence.insert((x, y, Direction::Down));
                    }
                    if lookup(x as i32, y as i32 + 1).unwrap_or(' ') == c {
                        check.insert((x, y + 1));
                    } else {
                        fence.insert((x, y, Direction::Up));
                    }
                }
                let mut start_pos = 0;
                while start_pos < fence.len() {
                    let c = fence.iter().nth(start_pos).unwrap().clone();
                    start_pos += 1;
                    match c.2 {
                        Direction::Up => {
                            for dx in 1..w {
                                if c.0 + dx >= w || !fence.remove(&(c.0 + dx, c.1, Direction::Up)) {
                                    break;
                                }
                            }

                            for dx in 1..w {
                                if dx > c.0 || !fence.remove(&(c.0 - dx, c.1, Direction::Up)) {
                                    break;
                                }
                            }
                        }
                        Direction::Down => {
                            for dx in 1..w {
                                if c.0 + dx >= w || !fence.remove(&(c.0 + dx, c.1, Direction::Down))
                                {
                                    break;
                                }
                            }

                            for dx in 1..w {
                                if dx > c.0 || !fence.remove(&(c.0 - dx, c.1, Direction::Down)) {
                                    break;
                                }
                            }
                        }
                        Direction::Left => {
                            for dy in 1..h {
                                if c.1 + dy >= h || !fence.remove(&(c.0, c.1 + dy, Direction::Left))
                                {
                                    break;
                                }
                            }

                            for dy in 1..h {
                                if dy > c.1 || !fence.remove(&(c.0, c.1 - dy, Direction::Left)) {
                                    break;
                                }
                            }
                        }
                        Direction::Right => {
                            for dy in 1..h {
                                if c.1 + dy >= h
                                    || !fence.remove(&(c.0, c.1 + dy, Direction::Right))
                                {
                                    break;
                                }
                            }

                            for dy in 1..h {
                                if dy > c.1 || !fence.remove(&(c.0, c.1 - dy, Direction::Right)) {
                                    break;
                                }
                            }
                        }
                    }
                }
                fence_area.insert(index, (area, fence.len()));
            }
        }
    }

    Some(fence_area.values().map(|(a, f)| a * f).sum())
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
        assert_eq!(solution_a(&data), Some(1930));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(1206));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1434856));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(891106));
    }
}
