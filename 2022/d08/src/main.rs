use std::fs::read_to_string;
use std::collections::HashSet;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let cols = input.find('\n').map(|first| input.get(first+1..).unwrap().find('\n').unwrap()).unwrap();
    let data: Vec<u8> = input.chars().filter(|x| x.is_ascii_digit()).map(|x| x as u8 - '0' as u8 + 1).collect();
    let rows = data.len() / cols;
    let nn = |r: usize, c: usize| -> u8 { data[r*cols + c]};
    let mut res = HashSet::<(usize, usize)>::new();
    let mut m;

    for r in 0usize..rows {
        m = 0;
        for c in 0usize..cols {
            let n = nn(r, c);
            if n > m {
                res.insert((r, c));
                m = n;
            }
        }

        m = 0;
        for c in (0usize..cols).rev() {
            let n = nn(r, c);
            if n > m {
                res.insert((r, c));
                m = n;
            }
        }
    }

    for c in 0usize..cols {
        m = 0;
        for r in 0usize..rows {
            let n = nn(r, c);
            if n > m {
                res.insert((r, c));
                m = n;
            }
        }
        m = 0;
        for r in (0usize..rows).rev() {
            let n = nn(r, c);
            if n > m {
                res.insert((r, c));
                m = n;
            }
        }
    }
    Some(res.len())
}

fn solution_b(input: &str) -> Option<usize> {
    let cols = input.find('\n').map(|first| input.get(first+1..).unwrap().find('\n').unwrap()).unwrap();
    let data: Vec<u8> = input.chars().filter(|x| x.is_ascii_digit()).map(|x| x as u8 - '0' as u8 + 1).collect();
    let rows = data.len() / cols;
    let nn = |r: usize, c: usize| -> u8 { data[r*cols + c]};

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
