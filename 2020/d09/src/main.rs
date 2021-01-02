use std::fs::read_to_string;
use std::io::{self};

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn solution_a() -> Option<u64> {
    let c = content().unwrap();
    let a = c
        .split_whitespace()
        .map(|x| x.parse::<u64>().ok())
        .filter(|x| x.is_some());
    let mut holder = [0u64; 25];
    let mut n = 0usize;

    let find_product = |x: u64, h: &[u64]| -> Option<u64> {
        for i in 0..25 {
            for j in 0..25 {
                if i != j && h[i] + h[j] == x {
                    return Some(x);
                }
            }
        }
        None
    };

    for x in a {
        if n < 25 {
            holder[n] = x.unwrap();
        } else {
            let value = x.unwrap();
            if find_product(value, &holder) == None {
                return Some(value);
            }
            holder[n % 25] = value;
        }
        n += 1;
    }
    None
}

fn solution_b(goal: Option<u64>) -> Option<u64> {
    if goal.is_none() {
        return None;
    }
    let goal = goal.unwrap();

    let all: Vec<u64> = content()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    let m = all.len();
    let mut f = 0usize;
    let mut t = 1usize;
    let mut s = all[0] + all[1];
    loop {
        if s == goal {
            let a = all[f .. t].iter().min().unwrap();
            let b = all[f .. t].iter().max().unwrap();
            return Some(a + b);
        }
        if s < goal {
            t += 1usize;
            if t >= m {
                return None;
            }
            s += all[t];
        } else {
            while s > goal {
                s -= all[f];
                f += 1;
                if f + 1 >= t {
                    t += 1;
                    s += all[t];
                    if t >= m {
                        return None;
                    }
                }
            }
        }
    }
}

fn main() {
    let a = solution_a();
    let b = solution_b(a);

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_file_reading() -> Result<(), io::Error> {
        assert_ne!(content()?.len(), 0);
        Ok(())
    }

    #[test]
    fn test_conversion() -> Result<(), io::Error> {
        let c = content().unwrap();
        let a = c
            .split_whitespace()
            .map(|x| x.parse::<u64>().ok())
            .filter(|x| x.is_none());
        assert_eq!(a.count(), 0);
        Ok(())
    }

    #[test]
    fn test_solution_a() -> Result<(), io::Error> {
        assert_eq!(solution_a(), Some(31161678));
        Ok(())
    }

    #[test]
    fn test_solution_b() -> Result<(), io::Error> {
        let a = solution_a();
        assert_eq!(solution_b(a), Some(5453868));
        Ok(())
    }
}
