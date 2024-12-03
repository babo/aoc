use regex::Regex;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solve_a(line: &str) -> usize {
    let mut result = vec![];
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    re.captures_iter(line)
        .map(|x| {
            let a = x.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let b = x.get(3).unwrap().as_str().parse::<usize>().unwrap();
            result.push(a * b);
        })
        .count();

    result.iter().sum()
}

fn solve_b(line: &str) -> usize {
    let input = format!("do(){}", line)
        .replace("\n", "")
        .replace("do()", "\ndo()")
        .replace("don't()", "\ndon't()");

    let mut result = vec![];
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    input
        .lines()
        .filter(|x| x.starts_with("do()"))
        .for_each(|hay| {
            re.captures_iter(hay)
                .map(|x| {
                    let a = x.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    let b = x.get(3).unwrap().as_str().parse::<usize>().unwrap();
                    result.push(a * b);
                })
                .count();
        });

    result.iter().sum()
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| solve_a(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    Some(solve_b(input.trim()))
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

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_simple_a() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solution_a(data), Some(161));
    }

    #[test]
    fn test_simple_b() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solution_b(data), Some(48));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(162813399));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(53783319));
    }
}
