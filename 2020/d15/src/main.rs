use std::collections::HashMap;

static DAILY: [u32; 7] = [0, 12, 6, 13, 20, 1, 17];

fn play_a(start: &[u32], steps: u32) -> u32 {
    if steps < start.len() as u32 {
        return start[steps as usize];
    }
    let mut said = HashMap::new();
    let count = start.iter().fold(0u32, |acc, x| {
        said.insert(*x, acc);
        acc + 1
    });
    let mut n = 0u32;
    for i in count..steps {
        let k = if said.contains_key(&n) {
            i - *said.get(&n).unwrap()
        } else {
            0
        };
        said.insert(n, i);
        n = k;
    }
    n
}

fn solution_a() -> u32 {
    play_a(&DAILY, 2019)
}

fn solution_b() -> i32 {
    0
}

fn main() {
    let a = solution_a();
    let b = solution_b();

    println!("Step A: {}", a);
    println!("Step B: {}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_a() {
        let input: [u32; 3] = [0, 3, 6];
        assert_eq!(play_a(&input, 2019), 436);
    }

    #[test]
    fn test_play_aa() {
        let input: [u32; 3] = [0, 3, 6];
        let expected = [0u32, 3, 6, 0, 3, 3, 1, 0, 4, 0];
        for i in expected.iter().zip(0..) {
            assert_eq!(play_a(&input, i.1), *i.0);
        }
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(solution_a(), 620);
    }
}
