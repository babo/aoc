use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn command_a(input: &str, pos: (usize, usize)) -> Option<(usize, usize)> {
    let mut it = input.trim().split_whitespace();
    let command = it.next();
    it.next().and_then(|x| usize::from_str_radix(x.trim(), 10).ok()).and_then(|val| command.and_then(|cmd| {
        if cmd.starts_with("forward") {
            Some((pos.0 + val, pos.1))
        } else if cmd.starts_with("up") {
            Some((pos.0, pos.1 - val))
        } else if cmd.starts_with("down") {
            Some((pos.0, pos.1 + val))
        } else {
            None
        }
    }))
}

fn solution_a(input: &str) -> Option<usize> {
    let a: (usize, usize) =
        input
            .lines()
            .fold((0usize, 0usize), |acc, line: &str| command_a(line, acc).unwrap());
    Some(a.0 * a.1)
}

fn command_b(input: &str, pos: (i64, i64, i64)) -> Option<(i64, i64, i64)> {
    let mut it = input.trim().split_whitespace();
    let command = it.next();
    it.next().and_then(|x| i64::from_str_radix(x.trim(), 10).ok()).and_then(|val| command.and_then(|cmd| {
        if cmd.starts_with("forward") {
            Some((pos.0 + val, pos.1 + val * pos.2, pos.2))
        } else if cmd.starts_with("up") {
            Some((pos.0, pos.1, pos.2 - val))
        } else if cmd.starts_with("down") {
            Some((pos.0, pos.1, pos.2 + val))
        } else {
            None
        }
    }))
}

fn solution_b(input: &str) -> Option<i64> {
    let a: (i64, i64, i64) =
        input
            .lines()
            .fold((0, 0, 0), |acc, line: &str| command_b(line, acc).unwrap());
    Some(a.0 * a.1)
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
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        assert_eq!(solution_a(&input), Some(150));
    }

    #[test]
    fn test_simple_b() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        assert_eq!(solution_b(&input), Some(900));
    }
}
