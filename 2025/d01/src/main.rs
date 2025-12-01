use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .fold((50i64, 0usize), |acc, line| {
                let inst = line
                    .trim()
                    .replace("L", "-")
                    .replace("R", "")
                    .parse::<i64>();

                if inst.is_err() {
                    acc
                } else {
                    let (pos, count) = acc;
                    let n = (pos + inst.unwrap() + 100) % 100;
                    (n, if n == 0 { count + 1 } else { count })
                }
            })
            .1,
    )
}

fn count_b(pos: i64, count: usize, mut rot: i64) -> (i64, usize) {
    let mut inc = 0;
    if rot >= 100 {
        inc = rot / 100;
        rot %= 100;
    } else if rot <= -100 {
        inc = rot / -100;
        rot %= 100;
    }
    let mut n = pos + rot;
    if n >= 100 {
        inc += 1;
        n -= 100;
    } else if n <= 0 {
        if pos != 0 {
            inc += 1;
        }
        n = (100 + n) % 100;
    }
    println!("{rot} {n} {inc}");
    (n, count + inc as usize)
}

fn solution_b(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .fold((50i64, 0usize), |acc, line| {
                let inst = line
                    .trim()
                    .replace("L", "-")
                    .replace("R", "")
                    .parse::<i64>();

                if inst.is_err() {
                    acc
                } else {
                    let (prev, count) = acc;
                    count_b(prev, count, inst.unwrap())
                }
            })
            .1,
    )
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
        assert_eq!(solution_a(&data), Some(3));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(6));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1191));
    }

    // > 6835 && < 7110
    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(6858));
    }
}
