use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let mut rtv = 0usize;
    let data = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let n = data.len();
    let mut a = 0;
    let mut b = if n % 2 == 0 { n } else { n + 1 };
    let mut g = 0;
    let mut pos = 0usize;

    while a < b {
        if a % 2 == 0 {
            for _ in 0..data[a] {
                rtv += pos * (a / 2);
                pos += 1;
            }
        } else {
            for _ in 0..data[a] {
                if g == 0 {
                    if a + 1 >= b {
                        break;
                    }
                    b -= 2;
                    g = data[b];
                }
                rtv += pos * (b / 2);
                g -= 1;
                pos += 1;
            }
        }
        a += 1;
    }
    while g > 0 {
        rtv += pos * b / 2;
        g -= 1;
        pos += 1;
    }

    Some(rtv)
}

fn solution_b(input: &str) -> Option<usize> {
    let data = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let n = data.len();

    let (mut space, _maxi) = (0..n / 2).fold((vec![], 0), |(mut acc, m), i| {
        let s = data[1 + i * 2];
        acc.push((s, i, 1));
        (acc, m.max(s))
    }); // space, index, subpos
    let mut files = (0..=n / 2).map(|x| (x, x, 0)).collect_vec(); // (original, current, subpos)

    for j in (0..=n / 2).rev() {
        let s = data[j * 2];
        if let Some((pos, curr)) = space.iter().take(j).find_position(|x| x.0 >= s) {
            let new_pos = curr.clone();

            if let Some(it) = space.get_mut(pos) {
                *it = (it.0 - s, it.1, it.2 + 1);
            };
            if let Some(it) = files.get_mut(j) {
                *it = (it.0, new_pos.1, new_pos.2);
            };
        }
    }
    let reordered = files.iter().sorted_by(|a, b| {
        if a.1 < b.1 {
            std::cmp::Ordering::Less
        } else if a.1 > b.1 {
            std::cmp::Ordering::Greater
        } else {
            assert!(a.2 != b.2);
            if a.2 < b.2 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    });

    let (_, rtv, _) = reordered.fold((0usize, 0usize, None), |(mut pos, mut acc, prev), x| {
        if let Some(p) = prev {
            if p != x.1 {
                if let Some((sp, _, _)) = space.get(p) {
                    for _ in 0..*sp {
                        print!("x");
                        pos += 1;
                    }
                }

                if x.2 != 0 {
                    for _ in 0..data[x.1 * 2] {
                        print!("+");
                        pos += 1;
                    }
                }
                for i in (p + 1)..x.1 {
                    for _ in 0..data[i * 2] {
                        print!("_");
                        pos += 1;
                    }
                    for _ in 0..data[i * 2 + 1] {
                        print!(".");
                        pos += 1;
                    }
                }
            }
        }

        let c = data[x.0 * 2];
        for _ in 0..c {
            print!("{}", x.0);
            acc += pos * x.0;
            pos += 1;
        }
        (pos, acc, Some(x.1))
    });

    Some(rtv)
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
        assert_eq!(solution_a(&data), Some(1928));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(2858));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(6332189866718));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(6353648390778));
    }
}
