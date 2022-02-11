use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn read_input(input: &str) -> (String, String, i32, i32) {
    let input = String::from_iter(input.chars().map(|c| match c {
        '#' => '1',
        '.' => '0',
        _ => c,
    }));

    let n = input.lines().position(|l| l.is_empty()).unwrap();
    let algo = input
        .lines()
        .take(n)
        .map(|l| l.trim())
        .fold(String::new(), |acc, l| acc + l);
    let (pixels, w, h) = input
        .lines()
        .skip(n + 1)
        .map(|l| l.trim())
        .fold((String::new(), 0, 0), |acc, l| {
            (acc.0 + l, l.len(), acc.2 + 1)
        });

    (algo, pixels, w as i32, h)
}

fn solution(input: &str, rounds: usize) -> Option<usize> {
    let (algo, pixels, w, h) = read_input(input);
    let mut h = h;
    let mut w = w;
    let mut pixels = pixels.to_string();

    let step_1 = algo.chars().next().unwrap();
    let step_0 = if step_1 == '0' {
        '0'
    } else {
        algo.chars().skip(511).next().unwrap()
    };

    for round in 0..rounds {
        println!("Round {}", round);
        let pixel = |(x, y)| {
            if x < 0 || x >= w || y < 0 || y >= h {
                match round % 2 {
                    0 => {
                        if round == 0 {
                            '0'
                        } else {
                            step_0
                        }
                    }
                    1 => step_1,
                    _ => unreachable!("No steps beyond"),
                }
            } else {
                let n = x * w + y;
                pixels.chars().skip(n as usize).next().unwrap()
            }
        };
        let mut res = String::new();
        for i in -1..w + 1 {
            for j in -1..h + 1 {
                let mut nine = String::new();
                for x in i - 1..i + 2 {
                    for y in j - 1..j + 2 {
                        let c = pixel((x, y));
                        nine.push(c);
                    }
                }
                let index = usize::from_str_radix(nine.as_str(), 2).ok().unwrap();
                res.push(algo.chars().skip(index).next().unwrap());
            }
        }
        w += 2;
        h += 2;
        pixels = res;
    }

    Some(pixels.chars().filter(|c| *c == '1').count())
}

fn solution_a(input: &str) -> Option<usize> {
    solution(input, 2)
}

fn solution_b(input: &str) -> Option<usize> {
    solution(input, 50)
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
        assert_eq!(solution_a(&data), Some(35));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(3351));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(5819));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(18516));
    }
}
