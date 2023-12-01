use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solve_a(line: &str) -> usize {
    if line.is_empty() {
        return 0;
    }
    let nums = line
        .chars()
        .filter(|x| x.is_numeric())
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let a = nums.first().unwrap();
    let b = nums.last().unwrap();
    (a * 10 + b) as usize
}

fn first_digit(line: &str) -> char {
    let mut a = line.chars().enumerate().find(|x| x.1.is_numeric());
    "zero one two three four five six seven eight nine"
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|ds| {
            let r = ds.0.to_string().chars().next().unwrap();
            let f = line.find(ds.1).map_or(line.len(), |x| x);
            a = a.map_or(
                Some((f, r)),
                |x| {
                    if f < x.0 {
                        Some((f, r))
                    } else {
                        Some(x)
                    }
                },
            );
        });
    a.map_or('0', |x| x.1)
}

fn last_digit(line: &str) -> char {
    let mut a = line.chars().enumerate().filter(|x| x.1.is_numeric()).last();
    "zero one two three four five six seven eight nine"
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|ds| {
            let r = ds.0.to_string().chars().next().unwrap();
            let f = line.rfind(ds.1).map_or(0, |x| x);
            a = a.map_or(
                Some((f, r)),
                |x| {
                    if f > x.0 {
                        Some((f, r))
                    } else {
                        Some(x)
                    }
                },
            );
        });
    a.map_or('0', |x| x.1)
}

fn solve_b(line: &str) -> usize {
    if line.is_empty() {
        return 0;
    }
    let a = first_digit(line).to_digit(10).unwrap();
    let b = last_digit(line).to_digit(10).unwrap();

    (a * 10 + b) as usize
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_b(x.trim())).sum::<usize>())
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
        let data = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(solution_a(&data), Some(142));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(281));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(55002));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(55093));
    }
}
