use std::fs::read_to_string;
use std::io::{self};

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn solution_a() -> Option<u32> {
    let mut all: Vec<u32> = content()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().ok())
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    all.sort();
    all.push(all.last().unwrap() + 3);
    let rtv = all
        .iter()
        .fold((0u32, 0u32, 0u32), |acc, x| -> (u32, u32, u32) {
            match x - acc.0 {
                1 => (*x, acc.1 + 1, acc.2),
                3 => (*x, acc.1, acc.2 + 1),
                _ => panic!("What a difference: {}", x - acc.0),
            }
        });
    Some(rtv.1 * rtv.2)
}

fn solution_b() -> Option<u32> {
    None
}

fn main() {
    let a = solution_a();
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
    fn test_file_reading() -> Result<(), io::Error> {
        assert_ne!(content()?.len(), 0);
        Ok(())
    }

    #[test]
    fn test_conversion() -> Result<(), io::Error> {
        let c = content().unwrap();
        let a = c
            .split_whitespace()
            .map(|x| x.parse::<u32>().ok())
            .filter(|x| x.is_none());
        assert_eq!(a.count(), 0);
        Ok(())
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(solution_a(), Some(1856));
    }
}
