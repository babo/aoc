use std::fs::read_to_string;
use std::io::{self};

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn solution_a() -> u32 {
    0
}

fn solution_b() -> u128 {
    0
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
}
