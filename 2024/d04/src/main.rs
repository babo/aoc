use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let h = input.lines().count();
    let w = input.lines().next()?.trim().len();
    let t = input.replace("\n", "");
    let c = |x: usize, y: usize| t.chars().nth(y * w + x).unwrap();
    let mut count = 0;

    for y in 0..h {
        for x in 0..w {
            if c(x, y) != 'X' {
                continue;
            }
            if y + 3 < h && c(x, y + 1) == 'M' && c(x, y + 2) == 'A' && c(x, y + 3) == 'S' {
                count += 1;
            }
            if x + 3 < w && c(x + 1, y) == 'M' && c(x + 2, y) == 'A' && c(x + 3, y) == 'S' {
                count += 1;
            }
            if y >= 3 && c(x, y - 1) == 'M' && c(x, y - 2) == 'A' && c(x, y - 3) == 'S' {
                count += 1;
            }
            if x >= 3 && c(x - 1, y) == 'M' && c(x - 2, y) == 'A' && c(x - 3, y) == 'S' {
                count += 1;
            }

            if y + 3 < h && x + 3 < w && c(x + 1, y + 1) == 'M' && c(x + 2, y + 2) == 'A' && c(x + 3, y + 3) == 'S' {
                count += 1;
            }
            if y >= 3 && x + 3 < w && c(x + 1, y - 1) == 'M' && c(x + 2, y - 2) == 'A' && c(x + 3, y - 3) == 'S' {
                count += 1;
            }
            if y + 3 < h && x >= 3 && c(x - 1, y + 1) == 'M' && c(x - 2, y + 2) == 'A' && c(x - 3, y + 3) == 'S' {
                count += 1;
            }
            if y >= 3 && x >= 3 && c(x - 1, y - 1) == 'M' && c(x - 2, y - 2) == 'A' && c(x - 3, y - 3) == 'S' {
                count += 1;
            }
        }
    }

    Some(count)
}

fn solution_b(input: &str) -> Option<usize> {
    let h = input.lines().count();
    let w = input.lines().next()?.trim().len();
    let t = input.replace("\n", "");
    let c = |x: usize, y: usize| t.chars().nth(y * w + x).unwrap();
    let mut count = 0;

    for y in 1..h-1 {
        for x in 1..w-1 {
            if c(x, y) != 'A' {
                continue;
            }
            if (c(x - 1, y - 1) == 'M' && c(x + 1, y + 1) == 'S') || (c(x - 1, y -1) == 'S' && c(x + 1, y + 1) == 'M') {
                if (c(x - 1, y + 1) == 'M' && c(x + 1, y - 1) == 'S') || (c(x - 1, y + 1) == 'S' && c(x + 1, y - 1) == 'M') {
                    count += 1;
                }
            }
        }
    }

    Some(count)
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
        assert_eq!(solution_a(&data), Some(18));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(9));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(2493));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(1890));
    }
}
