use std::fs::read_to_string;
use std::io::{self};

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn main() {
    let c = content().unwrap();

    println!("Step A: {}", c.len());
    println!("Step B: ?");
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
    fn test_combinations() -> Result<(), io::Error> {
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
        let mut found: Option<u64> = None;
        for x in a {
            if n < 25 {
                holder[n] = x.unwrap();
            } else {
                let value = x.unwrap();
                if find_product(value, &holder) == None {
                    found = Some(value);
                    println!("{}", value);
                    break;
                }
                holder[n % 25] = value;
            }
            n += 1;
        }

        assert_eq!(found, Some(31161678));
        Ok(())
    }
}
