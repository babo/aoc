use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let w = input.trim().find('\n').unwrap() as i32;
    let instructions = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .join("")
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<_>>();
    let maze = input
        .lines()
        .map(|x| x.trim())
        .take_while(|line| !line.is_empty())
        .collect_vec()
        .join("");
    let h = maze.len() as i32 / w;
    let p_to_coord = |p: i32| (p % w, p / w);
    let coord_to_c = |x: i32, y: i32| maze.chars().nth((y * w + x) as usize).unwrap();
    let wall = |x: i32, y: i32| x <= 0 || y <= 0 || x >= w || y >= h || coord_to_c(x, y) == '#';
    let robot = p_to_coord(maze.chars().position(|c| c == '@').unwrap() as i32);

    let mut boxes = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            if coord_to_c(x, y) == 'O' {
                boxes.insert((x, y));
            }
        }
    }

    let ((rx, ry), b) =
        instructions
            .iter()
            .fold((robot, boxes), |((rx, ry), mut boxes), (dx, dy)| {
                /*
                                println!("{} {}", dx, dy);
                                for y in 0..h {
                                    for x in 0..w {
                                        if wall(x, y) {
                                            print!("#");
                                        } else if (x, y) == (rx, ry) {
                                            print!("@");
                                        } else if boxes.contains(&(x, y)) {
                                            print!("O");
                                        } else {
                                            print!(".");
                                        }
                                    }
                                    println!();
                                }

                */
                let nx = rx + dx;
                let ny = ry + dy;
                if wall(nx, ny) {
                    return ((rx, ry), boxes);
                }
                if boxes.contains(&(nx, ny)) {
                    for i in 1.. {
                        let bnx = nx + dx * i;
                        let bny = ny + dy * i;
                        if wall(bnx, bny) {
                            return ((rx, ry), boxes);
                        }
                        if !boxes.contains(&(bnx, bny)) {
                            boxes.remove(&(nx, ny));
                            boxes.insert((bnx, bny));
                            break;
                        }
                    }
                }
                ((nx, ny), boxes)
            });

    for y in 0..h {
        for x in 0..w {
            if wall(x, y) {
                print!("#");
            } else if (x, y) == (rx, ry) {
                print!("@");
            } else if b.contains(&(x, y)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Some(b.iter().map(|(x, y)| (y * 100 + x) as usize).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    let instructions = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .join("")
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<_>>();
    let maze = input
        .lines()
        .map(|x| x.trim())
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            line.replace("#", "WW")
                .replace("O", "[]")
                .replace(".", "..")
                .replace("@", "@.")
        })
        .collect_vec()
        .join("");
    let w = 2 * input.trim().find('\n').unwrap() as i32;
    let h = maze.len() as i32 / w;
    let p_to_coord = |p: i32| (p % w, p / w);
    let coord_to_c = |x: i32, y: i32| maze.chars().nth((y * w + x) as usize).unwrap();
    let wall = |x: i32, y: i32| x <= 0 || y <= 0 || x >= w || y >= h || coord_to_c(x, y) == 'W';
    let robot = p_to_coord(maze.chars().position(|c| c == '@').unwrap() as i32);

    let mut boxes: HashSet<(i32, i32, char)> = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            if coord_to_c(x, y) == '[' || coord_to_c(x, y) == ']' {
                boxes.insert((x, y, coord_to_c(x, y)));
            }
        }
    }

    let ((rx, ry), b) =
        instructions
            .iter()
            .fold((robot, boxes), |((rx, ry), mut boxes), (dx, dy)| {
                /*
                                println!("{} {}", dx, dy);
                                for y in 0..h {
                                    for x in 0..w {
                                        if wall(x, y) {
                                            print!("#");
                                        } else if (x, y) == (rx, ry) {
                                            print!("@");
                                        } else if boxes.contains(&(x, y, '[')) {
                                            print!("[");
                                        } else if boxes.contains(&(x, y, ']')) {
                                            print!("]");
                                        } else {
                                            print!(".");
                                        }
                                    }
                                    println!();
                                }
                */
                let nx = rx + dx;
                let ny = ry + dy;
                if wall(nx, ny) {
                    return ((rx, ry), boxes);
                }
                if boxes.contains(&(nx, ny, '[')) || boxes.contains(&(nx, ny, ']')) {
                    if *dy == 0 {
                        for i in 1.. {
                            let bnx = nx + dx * i;
                            let bny = ny + dy * i;
                            if wall(bnx, bny) {
                                return ((rx, ry), boxes);
                            }
                            if !boxes.contains(&(bnx, bny, '['))
                                && !boxes.contains(&(bnx, bny, ']'))
                            {
                                let exp = if *dx < 0 { ('[', ']') } else { (']', '[') };
                                let mut j = i;
                                while j > 1 {
                                    let b1 = boxes.insert((nx + dx * j, ny, exp.0));
                                    assert!(b1);
                                    j -= 1;
                                    let b2 = boxes.remove(&(nx + dx * j, ny, exp.0));
                                    assert!(b2);
                                    let b3 = boxes.insert((nx + dx * j, ny, exp.1));
                                    assert!(b3);
                                    j -= 1;
                                    let b4 = boxes.remove(&(nx + dx * j, ny, exp.1));
                                    assert!(b4);
                                }
                                break;
                            }
                        }
                    } else {
                        let mut to_move = HashSet::new();
                        if boxes.contains(&(nx, ny, '[')) {
                            to_move.insert((nx, ny, '['));
                            to_move.insert((nx + 1, ny, ']'));
                        } else {
                            to_move.insert((nx, ny, ']'));
                            to_move.insert((nx - 1, ny, '['));
                        }
                        loop {
                            let (nb, hit_wall) = to_move.iter().fold(
                                (vec![], false),
                                |(mut acc, mut hit_wall), (x, y, _)| {
                                    if !hit_wall {
                                        let bnx = x + dx;
                                        let bny = y + dy;
                                        if wall(bnx, bny) {
                                            hit_wall = true;
                                        } else if boxes.contains(&(bnx, bny, '['))
                                            && !to_move.contains(&(bnx, bny, '['))
                                        {
                                            acc.push((bnx, bny, '['));
                                            acc.push((bnx + 1, bny, ']'));
                                        } else if boxes.contains(&(bnx, bny, ']'))
                                            && !to_move.contains(&(bnx, bny, ']'))
                                        {
                                            acc.push((bnx, bny, ']'));
                                            acc.push((bnx - 1, bny, '['));
                                        }
                                    }
                                    (acc, hit_wall)
                                },
                            );
                            if hit_wall {
                                return ((rx, ry), boxes);
                            }
                            if nb.is_empty() {
                                break;
                            }
                            to_move.extend(nb.iter());
                        }
                        to_move.iter().for_each(|x| {
                            let b1 = boxes.remove(x);
                            assert!(b1);
                        });
                        to_move.iter().for_each(|(x, y, c)| {
                            let b1 = boxes.insert((x + dx, y + dy, *c));
                            assert!(b1);
                        });
                    }
                }
                ((nx, ny), boxes)
            });

    for y in 0..h {
        for x in 0..w {
            if wall(x, y) {
                print!("#");
            } else if (x, y) == (rx, ry) {
                print!("@");
            } else if b.contains(&(x, y, '[')) {
                print!("[");
            } else if b.contains(&(x, y, ']')) {
                print!("]");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Some(
        b.iter()
            .filter(|(_, _, c)| *c == '[')
            .map(|(x, y, _)| (y * 100 + x) as usize)
            .sum::<usize>(),
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
    fn test_mini_a() {
        let data = "########
                    #..O.O.#
                    ##@.O..#
                    #...O..#
                    #.#.O..#
                    #...O..#
                    #......#
                    ########

                    <^^>>>vv<v>>v<<";
        assert_eq!(solution_a(data), Some(2028));
    }

    #[test]
    fn test_mini_b() {
        let data = "#######
                    #...#.#
                    #.....#
                    #..OO@#
                    #..O..#
                    #.....#
                    #######

                    <vv<<^^<<^^
                    ";
        assert_eq!(solution_b(data), Some(618));
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(10092));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(9021));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1429911));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1453087));
    }
}
