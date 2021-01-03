use std::fs::read_to_string;

const W: usize = 96usize;
const H: usize = 93usize;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn to_floorplan(input: &str) -> [u16; W * H] {
    let mut plan = [0u16; W * H];

    let mut y = W;
    for line in input.split_whitespace() {
        let mut x = 1usize;
        for c in line.chars() {
            if c == 'L' {
                plan[x + y] = 1024;
            }
            x += 1;
        }
        y += W;
    }
    plan
}

fn _pprint(plan: &[u16; W * H]) {
    for y in 0..H {
        for x in 0..W {
            if plan[x + y * W] & 1024 == 1024 {
                print!(
                    "{}",
                    if plan[x + y * W] & 512 == 512 {
                        "#"
                    } else {
                        "L"
                    }
                );
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn turn(plan: &[u16; W * H]) -> Option<[u16; W * H]> {
    let mut empty: Vec<(usize, usize)> = Vec::new();
    let mut occupy: Vec<(usize, usize)> = Vec::new();

    for i in 0..plan.len() {
        if plan[i] & 1024 == 1024 {
            let xy = (i % W, i / W);

            if (plan[i] & 512) == 512 && (plan[i] & 255) >= 4 {
                empty.push(xy);
            }
            if (plan[i] & 1023) == 0 {
                occupy.push(xy);
            }
        }
    }
    // println!("Changes: {} {}", empty.len(), occupy.len());
    if empty.len() == 0 && occupy.len() == 0 {
        return None;
    }
    let mut rtv = [0u16; W * H];
    for i in 0..plan.len() {
        rtv[i] = plan[i];
    }

    for xy in empty.iter() {
        let (x, y) = xy;
        for xx in x - 1..x + 2 {
            for yy in y - 1..y + 2 {
                if !(xx == *x && yy == *y) {
                    let p = xx + yy * W;
                    if rtv[p] & 1024 == 1024 {
                        rtv[p] = (rtv[p] & 1536) + (rtv[p] & 255) - 1;
                    }
                }
            }
        }
        rtv[x + y * W] -= 512;
    }
    for xy in occupy.iter() {
        let (x, y) = xy;
        for xx in x - 1..x + 2 {
            for yy in y - 1..y + 2 {
                if !(xx == *x && yy == *y) {
                    let p = xx + yy * W;
                    if rtv[p] & 1024 == 1024 {
                        rtv[p] = (rtv[p] & 1536) + (rtv[p] & 255) + 1;
                    }
                }
            }
        }
        rtv[x + y * W] += 512;
    }
    Some(rtv)
}

fn solution_a(content: &str) -> usize {
    let mut fp = to_floorplan(content);
    loop {
        match turn(&fp) {
            None => break,
            Some(n) => fp = n,
        }
        // pprint(&fp);
    }
    let mut c = 0usize;
    for x in fp.iter() {
        if x & 1536 == 1536 {
            c += 1;
        }
    }
    c
}

fn solution_b() -> u128 {
    0
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b();

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_reading_map() {
        let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let a = solution_a(&sample);
        assert_eq!(a, 37);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let a = solution_a(&c);
        assert_eq!(a, 2178);
    }
}
