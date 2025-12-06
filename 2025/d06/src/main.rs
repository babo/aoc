use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let nums = input
        .lines()
        .filter(|x| x.chars().filter(|c| c.is_ascii_digit()).count() > 0)
        .map(|x| {
            x.trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let ops: String = input
        .lines()
        .skip_while(|x| !x.contains('+'))
        .map(|x| x.trim().to_string())
        .take(1)
        .collect();
    Some(
        ops.split_ascii_whitespace()
            .enumerate()
            .map(|(c, op)| {
                let s = match op {
                    "+" => 0usize,
                    "*" => 1usize,
                    _ => unreachable!("Should never happen {op}"),
                };
                nums.iter().fold(s, |acc, row| {
                    let n = row.get(c).unwrap();
                    match op {
                        "+" => acc + n,
                        "*" => acc * n,
                        _ => unreachable!("Should never happen {op}"),
                    }
                })
            })
            .sum(),
    )
}

fn solution_b(input: &str) -> Option<usize> {
    let chars = input.chars().collect::<Vec<char>>();
    let cols = input.find('\n').unwrap();
    let rows = chars.len() / cols;
    let mut total = 0;
    let mut nums = Vec::new();
    for c in (0..cols).rev() {
        let mut n = 0usize;
        for r in 0..rows - 1 {
            let p = r * (cols + 1) + c;
            let ch = *chars.get(p).unwrap();
            ch.to_digit(10).map(|d| n = n * 10 + d as usize);
        }
        if n != 0 {
            nums.push(n);
        } else {
            nums.clear();
        }
        match *chars.get((rows - 1) * (cols + 1) + c).unwrap() {
            '+' => total += nums.iter().fold(0, |acc, x| acc + x),
            '*' => total += nums.iter().fold(1, |acc, x| acc * x),
            _ => (),
        }
    }

    Some(total)
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
        assert_eq!(solution_a(&data), Some(4277556));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(3263827));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(4583860641327));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(11602774058280));
    }
}
