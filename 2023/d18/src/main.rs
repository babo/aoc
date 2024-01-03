use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn fill(source: &[char], w: usize, h: usize) -> Vec<char> {
    let pos = |x: usize, y: usize| (w * y + x);
    let mut data = source.iter().copied().collect_vec();

    for x in 0..w {
        for y in 0..h {
            if data[pos(x, y)] == 'x' {
                for i in x + 1..w {
                    if data[pos(i, y)] == '.' {
                        if let Some(c) = data.get_mut(pos(i, y)) {
                            *c = 'x';
                        }
                    } else {
                        break;
                    }
                }
                for i in (0..x).rev() {
                    if data[pos(i, y)] == '.' {
                        if let Some(c) = data.get_mut(pos(i, y)) {
                            *c = 'x';
                        }
                    } else {
                        break;
                    }
                }
                for j in y + 1..h {
                    if data[pos(x, j)] == '.' {
                        if let Some(c) = data.get_mut(pos(x, j)) {
                            *c = 'x';
                        }
                    } else {
                        break;
                    }
                }
                for j in (0..y).rev() {
                    if data[pos(x, j)] == '.' {
                        if let Some(c) = data.get_mut(pos(x, j)) {
                            *c = 'x';
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    data
}

fn solution_a(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let lr = false; //input.lines().skip(1).next().map(|x| x.starts_with("U ")) == Some(true);
    let coord = input
        .lines()
        .fold(((0, 0), vec![(0, 0)]), |(c, mut a), line| {
            let parts: (&str, &str, &str) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let d = parts.0.chars().next().unwrap();
            let n = parts.1.parse::<i32>().unwrap();

            let c = match d {
                'R' => (c.0 + n, c.1),
                'L' => (c.0 - n, c.1),
                'U' => (c.0, c.1 - n),
                'D' => (c.0, c.1 + n),
                _ => unimplemented!("What a char!"),
            };
            a.push(c);
            (c, a)
        })
        .1;
    let mi = (
        coord.iter().map(|x| x.0).min().unwrap(),
        coord.iter().map(|x| x.1).min().unwrap(),
    );
    let ma = (
        coord.iter().map(|x| x.0).max().unwrap(),
        coord.iter().map(|x| x.1).max().unwrap(),
    );
    println!("{:?} {:?}", mi, ma);

    let w = (ma.0 - mi.0) as usize + 1;
    let h = (ma.1 - mi.1) as usize + 1;
    let start_pos = ((0 - mi.0) as usize, (0 - mi.1) as usize);
    println!("{w}x{h}");
    let pos = |x: usize, y: usize| (w * y + x);
    let data = std::iter::repeat('.').take(w * h).collect_vec();
    let data = input
        .lines()
        .fold((start_pos, data), |((mut x, mut y), mut data), line| {
            let parts: (&str, &str, &str) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let n = parts.1.parse::<usize>().unwrap();
            match parts.0.chars().next().unwrap() {
                'R' => {
                    for _ in 0..n {
                        if let Some(c) = data.get_mut(pos(x, y)) {
                            *c = '#';
                        }
                        if let Some(c) = data.get_mut(pos(x, if lr { y.max(1) - 1 } else { y + 1 }))
                        {
                            if *c == '.' {
                                *c = 'x';
                            }
                        }
                        x += 1;
                    }
                }
                'L' => {
                    for _ in 0..n {
                        if let Some(c) = data.get_mut(pos(x, y)) {
                            *c = '#';
                        }
                        if let Some(c) = data.get_mut(pos(x, if lr { y + 1 } else { y.max(1) - 1 }))
                        {
                            if *c == '.' {
                                *c = 'x';
                            }
                        }
                        x -= 1;
                    }
                }
                'D' => {
                    for _ in 0..n {
                        if let Some(c) = data.get_mut(pos(x, y)) {
                            *c = '#';
                        }
                        if let Some(c) = data.get_mut(pos(if lr { x + 1 } else { x.max(1) - 1 }, y))
                        {
                            if *c == '.' {
                                *c = 'x';
                            }
                        }
                        y += 1;
                    }
                }
                'U' => {
                    for _ in 0..n {
                        if let Some(c) = data.get_mut(pos(x, y)) {
                            *c = '#';
                        }
                        if let Some(c) = data.get_mut(pos(if lr { x.max(1) - 1 } else { x + 1 }, y))
                        {
                            if *c == '.' {
                                *c = 'x';
                            }
                        }
                        y -= 1;
                    }
                }
                _ => unimplemented!("What a direction!"),
            }
            ((x, y), data)
        })
        .1;

    let data = fill(&data, w, h);
    for i in 0..h {
        println!("{}", data[i * w..(i + 1) * w].iter().join(""));
    }
    let a = data.iter().filter(|x| **x == '#').count();
    let b = data.iter().filter(|x| **x == 'x').count();

    println!("border: {a} inside: {b} total: {}", a + b);
    Some(a + b)
}

fn parse_1(line: &str) -> (u8, usize) {
    let a: (&str, &str) = line
        .split_ascii_whitespace()
        .take(2)
        .collect_tuple()
        .unwrap();
    (
        match a.0.chars().next().unwrap() {
            'U' => 3,
            'D' => 1,
            'R' => 0,
            'L' => 2,
            _ => unimplemented!("What a dir"),
        },
        a.1.parse::<usize>().unwrap(),
    )
}

fn parse_2(line: &str) -> (u8, usize) {
    let code = line.split_ascii_whitespace().skip(2).next().unwrap();
    let code = code
        .chars()
        .map(|x| x.to_digit(16))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap() as usize)
        .collect_vec();
    let d = *code.iter().last().unwrap();
    let n = code
        .iter()
        .rev()
        .skip(1)
        .rev()
        .fold(0usize, |accu, x| accu * 16 + x);

    let dc = match d % 4 {
        0 => 'R',
        1 => 'D',
        2 => 'L',
        3 => 'U',
        _ => unimplemented!("NO way"),
    };
    println!("{} {} nothing", dc, n);
    ((d % 4) as u8, n)
}

type Side = ((i64, i64), i64, u8);
type Coord = ((i64, i64), (i64, i64));

fn read_input(input: &str) -> Vec<Side> {
    let simplified: Vec<(u8, usize)> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(parse_1)
        .collect_vec();

    let coord = simplified
        .iter()
        .fold(((0i64, 0i64), vec![]), |(c, mut a), (direction, size)| {
            let isize = *size as i64;
            let c = match direction {
                0 => (c.0 + isize, c.1),
                2 => (c.0 - isize, c.1),
                3 => (c.0, c.1 - isize),
                1 => (c.0, c.1 + isize),
                _ => unimplemented!("What a char!"),
            };
            a.push((c, *size, *direction));
            (c, a)
        })
        .1;
    let start_pos = (
        coord.iter().map(|x| x.0 .0).min().unwrap(),
        coord.iter().map(|x| x.0 .1).min().unwrap(),
    );
    coord
        .iter()
        .map(|(c, l, d)| {
            let x = c.0 - start_pos.0;
            let y = c.1 - start_pos.1;
            let il = *l as i64;
            let (x, y) = match d {
                0 => (x - il, y),
                2 => (x + il, y),
                1 => (x, y - il),
                3 => (x, y + il),
                _ => unimplemented!("No way"),
            };
            ((x, y), il, *d)
        })
        .collect_vec()
}

fn read_inner(input: &str) -> Vec<Side> {
    let simplified: Vec<(u8, usize)> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(parse_1)
        .collect_vec();

    let compensate = |(pd, ps): (u8, usize), (d, s): (u8, usize)| match (pd, d) {
        (0, 1) => (ps - 1, s),
        (0, 3) => (ps, s + 1),
        (1, 0) => (ps, s + 1),
        (1, 2) => (ps - 1, s),
        (2, 1) => (ps, s + 1),
        (2, 3) => (ps - 1, s),
        (3, 0) => (ps - 1, s),
        (3, 2) => (ps, s + 1),
        _ => unimplemented!("Not a supported combo"),
    };

    let (mut simplified, ny): (Vec<(u8, usize)>, Option<(u8, usize)>) =
        simplified
            .iter()
            .fold((Vec::new(), None), |(mut accu, prev), curr| {
                if let Some((pd, ps)) = prev {
                    let (ps, cs) = compensate((pd, ps), *curr);
                    accu.push((pd, ps));
                    (accu, Some((curr.0, cs)))
                } else {
                    (accu, Some(*curr))
                }
            });
    if let Some((d, s)) = ny {
        let ns = if let Some((pd, _ps)) = simplified.last() {
            compensate((*pd, s), (d, s)).0
        } else {
            s
        };
        simplified.push((d, ns));
    }

    let coord = simplified
        .iter()
        .fold(((0i64, 0i64), vec![]), |(c, mut a), (direction, size)| {
            let isize = *size as i64;
            let c = match direction {
                0 => (c.0 + isize, c.1),
                2 => (c.0 - isize, c.1),
                3 => (c.0, c.1 - isize),
                1 => (c.0, c.1 + isize),
                _ => unimplemented!("What a char!"),
            };
            a.push((c, *size, *direction));
            (c, a)
        })
        .1;
    let start_pos = (
        coord.iter().map(|x| x.0 .0).min().unwrap(),
        coord.iter().map(|x| x.0 .1).min().unwrap(),
    );
    coord
        .iter()
        .map(|(c, l, d)| {
            let x = c.0 - start_pos.0;
            let y = c.1 - start_pos.1;
            let il = *l as i64;
            let (x, y) = match d {
                0 => (x - il, y),
                2 => (x + il, y),
                1 => (x, y - il),
                3 => (x, y + il),
                _ => unimplemented!("No way"),
            };
            ((x, y), il, *d)
        })
        .collect_vec()
}

fn endpoints(side: &Side) -> Coord {
    match side.2 {
        0 => (side.0, (side.0 .0 + side.1, side.0 .1)),
        2 => ((side.0 .0 - side.1, side.0 .1), side.0),
        1 => (side.0, (side.0 .0, side.0 .1 + side.1)),
        3 => ((side.0 .0, side.0 .1 - side.1), side.0),
        _ => unimplemented!("Invalid direction"),
    }
}

fn cut_side(main: &Side, other: &Side) -> Option<Side> {
    if (other.2 + 2) % 4 != main.2 {
        None
    } else {
        let ma = endpoints(main);
        let ot = endpoints(other);
        if main.2 % 2 == 0 {
            if ot.1 .0 < ma.0 .0 || ot.0 .0 > ma.1 .0 {
                None
            } else {
                let ax = ma.0 .0.max(ot.0 .0);
                let bx = ma.1 .0.min(ot.1 .0);
                if ax == bx {
                    None
                } else if other.2 == 0 {
                    Some(((ax, ot.0 .1), bx - ax, other.2))
                } else {
                    Some(((bx, ot.0 .1), bx - ax, other.2))
                }
            }
        } else {
            if ot.1 .1 < ma.0 .1 || ot.0 .1 > ma.1 .1 {
                None
            } else {
                let ay = ma.0 .1.max(ot.0 .1);
                let by = ma.1 .1.min(ot.1 .1);
                if ay == by {
                    None
                } else if other.2 == 3 {
                    Some(((ot.0 .0, ay), by - ay, other.2))
                } else {
                    Some(((ot.0 .0, by), by - ay, other.2))
                }
            }
        }
    }
}

fn sides_above(main: Side, sides: &[&Side]) -> Vec<Side> {
    let dir = (main.2 + 2) & 2;
    let base_y = main.0 .1;
    sides
        .iter()
        .filter(|x| x.2 == dir)
        .filter(|p| p.0 .1 > base_y)
        .map(|side| cut_side(&main, side))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|p| ((p.0 .0, p.0 .1 - base_y), p.1, p.2))
        .collect_vec()
}

fn solution_b(input: &str) -> Option<usize> {
    let outer = read_input(input);
    let border = outer.iter().map(|x| x.1).sum::<i64>();
    let inner = read_inner(input);
    let horizontal = inner.iter().filter(|x| x.2 % 2 == 0).collect_vec();

    println!("{:?}", inner);
    println!("Border: {border}");

    let collected_area = horizontal
        .iter()
        .filter(|x| x.2 == 0)
        .map(|current| {
            println!("{:?}", current);
            let above = sides_above(**current, &horizontal);
            let above = above.iter().sorted_by_key(|((_, y), _, _)| y).collect_vec();

            let collected_area = above
                .iter()
                .map(|x| endpoints(x))
                .fold(
                    (Vec::new(), 0usize),
                    |(mut prev, area): (Vec<Coord>, usize), curr| {
                        let cutted = prev.iter().fold(Some(curr), |accu, p| {
                            if let Some(c) = accu {
                                println!("    ({}, {}) ({}, {})", p.0 .0, p.1 .0, c.0 .0, c.0 .1);
                                if c.0 .0 >= p.1 .0 || c.1 .0 <= p.0 .0 {
                                    Some(c)
                                } else if c.0 .0 >= p.0 .0 && c.1 .0 <= p.1 .0 {
                                    None
                                } else {
                                    Some((
                                        (c.0 .0.max(p.0 .0), c.0 .1),
                                        (c.1 .0.min(p.1 .0), c.1 .1),
                                    ))
                                }
                            } else {
                                None
                            }
                        });
                        let d = if let Some(c) = cutted {
                            if c.1 .0 > c.0 .0 {
                                //println!("    {:?} ({}, {})", curr, c.0.0, c.1.0);
                                prev.push(c);
                                c.1 .0 - c.0 .0
                            } else {
                                0
                            }
                        } else {
                            0
                        };

                        let h = curr.0 .1;
                        let area_under = d * h;
                        println!("{d}x{h}={area_under}");
                        (prev, area + area_under as usize)
                    },
                )
                .1;

            println!("\tarea above: {collected_area}");
            collected_area
        })
        .sum::<usize>();
    println!(
        "\tborder: {border} collected_area: {collected_area} total: {}",
        border as usize + collected_area
    );
    Some(border as usize + collected_area)
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
        assert_eq!(solution_a(&data), Some(62));
    }

    #[test]
    fn test_parsing() {
        assert_eq!(parse_2("a b (70c710)"), (0, 461937));
        assert_eq!(parse_2("a b (0dc571)"), (1, 56407));
        assert_eq!(parse_2("a b (5713f0)"), (0, 356671));
        assert_eq!(parse_2("a b (d2c081)"), (1, 863240));
        assert_eq!(parse_2("a b (59c680)"), (0, 367720));
        assert_eq!(parse_2("a b (411b91)"), (1, 266681));
        assert_eq!(parse_2("a b (8ceee2)"), (2, 577262));
        assert_eq!(parse_2("a b (caa173)"), (3, 829975));
        assert_eq!(parse_2("a b (1b58a2)"), (2, 112010));
        assert_eq!(parse_2("a b (caa171)"), (1, 829975));
        assert_eq!(parse_2("a b (7807d2)"), (2, 491645));
        assert_eq!(parse_2("a b (a77fa3)"), (3, 686074));
        assert_eq!(parse_2("a b (015232)"), (2, 5411));
        assert_eq!(parse_2("a b (7a21e3)"), (3, 500254));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(952408144115));
    }

    #[test]
    fn test_vilk() {
        let data = "
        R 20 nothing
        D 5 nothing
        R 10 nothing
        D 35 nothing
        R 15 nothing
        D 10 nothing
        L 20 nothing
        U 5 nothing
        L 10 nothing
        D 10 nothing
        L 5 nothing
        U 45 nothing
        L 10 nothing
        U 10 nothing";
        assert_eq!(solution_b(&data), Some(1281));
    }

    #[test]
    fn test_hulk() {
        let data = "
        R 20 nothing
        D 11 nothing
        R 20 nothing
        U 10 nothing
        R 10 nothing
        D 20 nothing
        L 8 nothing
        D 10 nothing
        R 6 nothing
        D 5 nothing
        L 20 nothing
        U 10 nothing
        L 20 nothing
        D 20 nothing
        L 10 nothing
        U 15 nothing
        L 15 nothing
        U 15 nothing
        R 10 nothing
        U 7 nothing
        R 6 nothing
        U 10 nothing";
        assert_eq!(solution_a(&data), solution_b(&data));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(61661));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let r = solution_b(&c);
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r < 136519297262412);
        assert_eq!(r, 61661);
    }

    #[test]
    fn test_minicube() {
        let c = "R 5 a
        D 2 a
        R 2 a
        D 2 a
        L 7 a
        U 4 a";
        assert_eq!(solution_a(&c), solution_b(&c));
    }

    #[test]
    fn test_endpoints() {
        let (a, b) = endpoints(&((0, 0), 6, 0));
        assert_eq!(a, (0, 0));
        assert_eq!(b, (6, 0));
        let (a, b) = endpoints(&((6, 0), 6, 2));
        assert_eq!(a, (0, 0));
        assert_eq!(b, (6, 0));

        let (a, b) = endpoints(&((0, 0), 6, 1));
        assert_eq!(a, (0, 0));
        assert_eq!(b, (0, 6));

        let (a, b) = endpoints(&((0, 6), 6, 3));
        assert_eq!(a, (0, 0));
        assert_eq!(b, (0, 6));
    }

    #[test]
    fn test_cut_side() {
        let a = ((0, 0), 10, 0);
        let b = ((10, 0), 10, 2);
        assert_eq!(Some(b), cut_side(&a, &b));
        assert_eq!(Some(a), cut_side(&b, &a));

        let b = ((12, 0), 5, 2);
        let c = ((10, 0), 3, 2);
        assert_eq!(Some(c), cut_side(&a, &b));

        let a = ((0, 0), 6, 0);
        let b = ((6, 9), 5, 2);
        assert_eq!(Some(c), cut_side(&a, &b));
    }
}
