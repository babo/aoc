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
    println!("Left inside: {lr}");
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
    Some(data.iter().filter(|x| **x != '.').count())
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

    ((d % 4) as u8, n)
}

fn solution_b(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let coord = input
        .lines()
        .fold(((0i64, 0i64), vec![]), |(c, mut a), line| {
            let (direction, size) = parse_1(line);
            let isize = size as i64;
            let c = match direction {
                0 => (c.0 + isize, c.1),
                2 => (c.0 - isize, c.1),
                3 => (c.0, c.1 - isize),
                1 => (c.0, c.1 + isize),
                _ => unimplemented!("What a char!"),
            };
            a.push((c, direction, size));
            (c, a)
        })
        .1;
    let start_pos = (
        coord.iter().map(|x| x.0 .0).min().unwrap(),
        coord.iter().map(|x| x.0 .1).min().unwrap(),
    );
    let sides = coord
        .iter()
        .map(|(c, d, l)| {
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
            ((x, y), *d, il)
        })
        .collect_vec();
    let base = sides.iter().map(|x| x.2).sum::<i64>();
    let horizontal = sides.iter().filter(|x| x.1 % 2 == 0).collect_vec();

    println!("{:?}", sides);
    println!("Base: {base}");
    println!("horizontal: {} lines", horizontal.len());

    Some(
        horizontal
            .iter()
            .filter(|x| x.1 == 0)
            .fold(base as usize, |area, current| {
                let (pos, _, length) = *current;
                let length = *length;
                println!("{} {}", pos.0, length);
                let above = horizontal
                    .iter()
                    .filter(|((_, y), dir, _)| *dir == 2 && *y > pos.1)
                    .filter(|((x, _), _, l)| *x > pos.0 && *x < pos.0 + length + *l)
                    .map(|((x, y), _, l)| {
                        let l = pos.0.max(*x - *l);
                        let r = (*x + 1).min(pos.0 + length);
                        (l - pos.0, r - l, *y - pos.1)
                    })
                    .sorted_by_key(|(_, _, y)| *y)
                    .collect_vec();
                above
                    .iter()
                    .for_each(|(x, l, y)| println!("    {x} {l} y: {y}"));

                let new_area = above
                    .iter()
                    .fold((vec![(0, length)], 0), |accu, side| {
                        let (not_filled, area) = accu;
                        let s_left = side.0 + 1;
                        let s_right = side.0 + side.1;
                        let s_height = side.2 - 1;
                        let x =
                            not_filled
                                .iter()
                                .fold((Vec::new(), area), |(mut remain, area), nf| {
                                    let left = nf.0;
                                    let right = nf.0 + nf.1;
                                    if s_left >= right || s_right <= left {
                                        remain.push(*nf);
                                        return (remain, area);
                                    }
                                    if left < s_left {
                                        remain.push((left, s_left - left));
                                    }
                                    if right > s_right {
                                        remain.push((s_right, right - s_right));
                                    }
                                    // println!("{} {} - {} {}", s_right, right, s_left, left);
                                    let d = right.min(s_right) - left.max(s_left);
                                    let na = d * s_height;
                                    // println!("area: {}x{}={}", d, s_height, na);
                                    (remain, area + na)
                                });
                        // println!("{:?}", x.0);
                        x
                    })
                    .1 as usize;
                area + new_area
            }),
    )
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
        assert_eq!(parse_2("a b (#70c710)"), (0, 461937));
        assert_eq!(parse_2("a b (#0dc571)"), (1, 56407));
        assert_eq!(parse_2("a b (#8ceee2)"), (2, 577262));
        assert_eq!(parse_2("a b (#7a21e3)"), (3, 500254));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(952408144115));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(61661));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(61661));
    }
}
