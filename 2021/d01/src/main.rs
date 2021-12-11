use std::fs::read_to_string;

fn replace<T: Copy>(a: &(T, T, T, T), pos: usize, val: T) -> (T, T, T, T) {
    match pos % 4usize {
        0 => (val, a.1, a.2, a.3),
        1 => (a.0, val, a.2, a.3),
        2 => (a.0, a.1, val, a.3),
        _ => (a.0, a.1, a.2, val)
    }
}

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let a: (Option<usize>, usize) =
        input
            .lines()
            .fold((None, 0usize), |acc: (Option<usize>, usize), x: &str| {
                usize::from_str_radix(x.trim(), 10).map_or(acc, |current| {
                    acc.0.map_or((Some(current), acc.1), |prev| {
                        (
                            Some(current),
                            if prev < current { acc.1 + 1 } else { acc.1 },
                        )
                    })
                })
            });
    Some(a.1)
}

fn solution_b(input: &str) -> Option<usize> {
    let a: ((usize, usize, usize, usize), usize, usize) =
        input
            .lines()
            .fold(((0usize, 0, 0, 0), 0usize, 0usize), |acc, x: &str| {
                usize::from_str_radix(x.trim(), 10).map_or(acc, |current| {
                    let pos = acc.1 % 4usize;
                    let val = replace(&acc.0, pos, current);
                    let gt = acc.1 > 2 && match pos {
                        0 => val.0 > val.1,
                        1 => val.1 > val.2,
                        2 => val.2 > val.3,
                        _ => val.3 > val.0
                    };

                    (val, acc.1 + 1, if gt {acc.2 + 1} else { acc.2 })
                })
            });
    Some(a.2)
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
        let input = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";
        assert_eq!(solution_a(&input), Some(7));
    }

    #[test]
    fn test_simple_b() {
        let input = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";
        assert_eq!(solution_b(&input), Some(5));
    }
}
