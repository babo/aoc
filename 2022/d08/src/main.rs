use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let data: Vec<&str> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0usize)
        .collect();
    let cols = data.len();
    let rows = data[0].len();
    let ptr = |r: usize, c: usize| -> usize { r * cols + c };
    let nn = |r: usize, c: usize| -> u8 { data[r].chars().nth(c).unwrap() as u8 };
    let mut res = Vec::<i8>::new();
    let mut m;

    for r in 0usize..rows {
        m = '0' as u8 - 1;
        for c in 0usize..cols {
            let n = nn(r, c);
            res.push(if n > m { 1 } else { 0 });
            if n > m {
                m = n;
            }
        }

        m = '0' as u8 - 1;
        for c in (0usize..cols).rev() {
            let n = nn(r, c);
            if n > m {
                res.get_mut(ptr(r, c)).map(|x| *x += 1);
                m = n;
            }
        }
    }

    for c in 0usize..cols {
        m = '0' as u8 - 1;
        for r in 0usize..rows {
            let n = nn(r, c);
            if n > m {
                res.get_mut(ptr(r, c)).map(|x| *x += 1);
                m = n;
            }
        }
        m = '0' as u8 - 1;
        for r in (0usize..rows).rev() {
            let n = nn(r, c);
            if n > m {
                res.get_mut(ptr(r, c)).map(|x| *x += 1);
                m = n;
            }
        }
    }
    let rtv = res.iter().filter(|x| (**x) != 0).count();
    Some(rtv)
}

fn solution_b(input: &str) -> Option<usize> {
    let data: Vec<&str> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0usize)
        .collect();
    let cols = data.len();
    let rows = data[0].len();
    let nn = |r: usize, c: usize| -> u8 { data[r].chars().nth(c).unwrap() as u8 };

    let mut score = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            let h = nn(r, c);
            let mut s = 1;

            let mut n = 0;
            for rr in r + 1..rows {
                n += 1;
                if nn(rr, c) >= h {
                    break;
                }
            }
            s *= n;

            n = 0;
            for rr in (0..=r - 1).rev() {
                n += 1;
                if nn(rr, c) >= h {
                    break;
                }
            }
            s *= n;

            n = 0;
            for cc in c + 1..cols {
                n += 1;
                if nn(r, cc) >= h {
                    break;
                }
            }
            s *= n;

            n = 0;
            for cc in (0..=c - 1).rev() {
                n += 1;
                if nn(r, cc) >= h {
                    break;
                }
            }
            s *= n;

            if s >= score {
                score = s;
            }
        }
    }
    Some(score)
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
        assert_eq!(solution_a(&data), Some(21));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(8));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1809));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(479400));
    }
}
