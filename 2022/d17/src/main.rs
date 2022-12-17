use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

/*
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##

*/

fn solution_a(input: &str) -> usize {
    let mut jet = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .nth(0)
        .unwrap()
        .chars()
        .cycle();
    let mut rock = (0..5).cycle();
    let mut top = [0i32; 7];
    let shapes: [u16; 5] = [
        0b01111,
        0b01001110010,
        0b010001000111,
        0b01000100010001,
        0b0110011,
    ];

    let scene = |(orig_x, orig_y), kind: usize, top: &[i32; 7]| {
        let my = top.iter().min().unwrap();
        for y in (*my..orig_y+4).rev() {
            print!("|");
            for x in 0..7 {
                let c = if x >= orig_x && x < orig_x + 4 && y >= orig_y && y < orig_y + 4 {
                    let (sx, sy) = (x-orig_x, y-orig_y);
                    let p = sy*4+sx;
                    if shapes[kind] & (1u16 << p) != 0u16 {
                        '@'
                    } else {
                        '.'
                    }
                } else {
                    '.'
                };
                print!("{}", if top[x as usize] >= y { '#' } else { c});
            }
            println!("|");
        }
        println!("+-------+\n");
    };

    let step = |dir, (orig_x, orig_y), kind, top: &[i32; 7]| {
        let (dx, dy) = match dir {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        };
        for py in 0..4 {
            let y = orig_y + py + dy;
            for px in 0..4 {
                let p = py * 4 + px;
                if shapes[kind] & (1 << p) != 0 {
                    let x = orig_x + px + dx;
                    if x < 0 || x > 6 {
                        return None;
                    }
                    if top[x as usize] >= y {
                        return None;
                    }
                }
            }
        }
        Some((dx, dy))
    };

    top[0] = 4;
    top[1] = 5;
    top[2] = 2;
    top[3] = 3;
    for kind in 1..2 {
        let mut orig_x: i32 = 2;
        let mut orig_y: i32 = top.iter().max().unwrap() + 4;

        for _ in 0..1 {
            match step(Direction::Right, (orig_x, orig_y), kind, &top) {
                Some((dx, dy)) => {
                    orig_x += dx;
                    orig_y += dy;
                }
                None => (),
            }
        }
        for _ in 0..9 {
            match step(Direction::Down, (orig_x, orig_y), kind, &top) {
                Some((_, dy)) => orig_y += dy,
                None => (),
            }
        }

        scene((orig_x, orig_y), kind, &top);

        for px in 0..4 {
            for py in 0..4 {
                let p = py * 4 + px;
                let x = orig_x + px;
                if shapes[kind] & (1 << p) != 0 {
                    top[x as usize] = top[x as usize].max(orig_y + py);
                }
            }
        }
        scene((orig_x, orig_y), kind, &top);
    }

    println!("Sopres");

    for _cycle in 0..2022
    {
        let kind = rock.next().unwrap();
        let mut orig_x: i32 = 2;
        let mut orig_y: i32 = top.iter().max().unwrap() + 4;

        loop {
            //println!("SSSSSS {kind}");
            //scene((orig_x, orig_y), kind, &top);

            let jet = if jet.next() == Some('<') {
                Direction::Left
            } else {
                Direction::Right
            };

            //println!("{}", if jet == Direction::Left { '<' } else { '>' });
            match step(jet, (orig_x, orig_y), kind, &top) {
                Some((dx, _)) => orig_x += dx,
                None => (),
            }
            match step(Direction::Down, (orig_x, orig_y), kind, &top) {
                Some((_, dy)) => orig_y += dy,
                None => break,
            }
            //scene((orig_x, orig_y), kind, &top);
        }

        for px in 0..4 {
            for py in 0..4 {
                let p = py * 4 + px;
                let x = orig_x + px;
                if shapes[kind] & (1 << p) != 0 {
                    top[x as usize] = top[x as usize].max(orig_y + py);
                }
            }
        }
    }
    *top.iter().max().unwrap() as usize
}

fn solution_b(_input: &str) -> Option<usize> {
    None
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
        assert_eq!(solution_a(&data), 3068);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 3201+1);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
