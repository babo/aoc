use std::fs::read_to_string;

const W: usize = 96usize;
const H: usize = 93usize;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn to_floorplan(input: &str) -> [u16; W * W] {
    let mut plan = [0u16; W * W];

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

fn _pprint(plan: &[u16; W * W]) {
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

fn turn_a(plan: &[u16; W * W]) -> Option<[u16; W * W]> {
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
    let mut rtv = [0u16; W * W];
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

fn turn_b(plan: &[u16; W * W]) -> Option<[u16; W * W]> {
    let mut empty: Vec<(usize, usize)> = Vec::new();
    let mut occupy: Vec<(usize, usize)> = Vec::new();

    for i in 0..plan.len() {
        if plan[i] & 1024 == 1024 {
            let xy = (i % W, i / W);

            if (plan[i] & 512) == 512 && (plan[i] & 255) > 4 {
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
    let mut rtv = [0u16; W * W];
    for i in 0..plan.len() {
        rtv[i] = plan[i];
    }

    for xy in empty.iter() {
        let (x, y) = xy;
        rtv[x + y * W] -= 512;
        let m = visible_seats(*x, *y, &rtv);
        for mp in m.iter() {
            match mp {
                Some(p) => {
                    if rtv[*p] & 1024 == 1024 {
                        rtv[*p] = (rtv[*p] & 1536) + (rtv[*p] & 255) - 1;
                    }
                }
                None => (),
            }
        }
    }
    for xy in occupy.iter() {
        let (x, y) = xy;
        rtv[x + y * W] += 512;
        let m = visible_seats(*x, *y, &rtv);
        for mp in m.iter() {
            match mp {
                Some(p) => {
                    if rtv[*p] & 1024 == 1024 {
                        rtv[*p] = (rtv[*p] & 1536) + (rtv[*p] & 255) + 1;
                    }
                }
                None => (),
            }
        }
    }
    Some(rtv)
}

fn visible_seats(x: usize, y: usize, plan: &[u16; W * W]) -> [Option<usize>; 8] {
    let mut nearest: [Option<usize>; 8] = [None; 8];

    for xx in x + 1..W {
        if plan[xx + y * W] & 1024 == 1024 {
            nearest[0] = Some(xx + y * W);
            break;
        }
    }
    for xx in (0..x).rev() {
        if plan[xx + y * W] & 1024 == 1024 {
            nearest[1] = Some(xx + y * W);
            break;
        }
    }
    for yy in y + 1..H {
        if plan[x + yy * W] & 1024 == 1024 {
            nearest[2] = Some(x + yy * W);
            break;
        }
    }
    for yy in (0..y).rev() {
        if plan[x + yy * W] & 1024 == 1024 {
            nearest[3] = Some(x + yy * W);
            break;
        }
    }
    for d in 1..W {
        let xx = x + d;
        let yy = y + d;
        if xx >= W || yy >= H {
            break;
        }
        if plan[xx + yy * W] & 1024 == 1024 {
            nearest[4] = Some(xx + yy * W);
            break;
        }
    }
    for d in 1..W {
        let xx = x - d;
        let yy = y - d;
        if plan[xx + yy * W] & 1024 == 1024 {
            nearest[5] = Some(xx + yy * W);
            break;
        }
        if xx == 0 || yy == 0 {
            break;
        }
    }
    for d in 1..W {
        let xx = x + d;
        let yy = y - d;
        if xx >= W {
            break;
        }
        if plan[xx + yy * W] & 1024 == 1024 {
            nearest[6] = Some(xx + yy * W);
            break;
        }
        if yy == 0 {
            break;
        }
    }
    for d in 1..W {
        let xx = x - d;
        let yy = y + d;
        if yy >= H {
            break;
        }
        if plan[xx + yy * W] & 1024 == 1024 {
            nearest[7] = Some(xx + yy * W);
            break;
        }
        if xx == 0 {
            break;
        }
    }
    nearest
}

fn solution_a(content: &str) -> usize {
    let mut fp = to_floorplan(content);
    loop {
        match turn_a(&fp) {
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

fn solution_b(content: &str) -> usize {
    let mut fp = to_floorplan(content);
    loop {
        match turn_b(&fp) {
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

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {}", a);
    println!("Step B: {}", b);
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
    fn test_turn_a() {
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

    #[test]
    fn test_visible_seats() {
        let sample = "L...L...L
.........
.........
.........
L...L...L
.........
.........
.........
L...L...L";
        let plan = to_floorplan(sample);
        // _pprint(&plan);
        let seats = visible_seats(5, 5, &plan);
        for x in seats.iter() {
            assert_eq!(x.is_some(), true);
        }
    }

    #[test]
    fn test_turn_b() {
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

        let a = solution_b(&sample);
        assert_eq!(a, 26);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let a = solution_b(&c);
        assert_eq!(a, 1978);
    }
}
