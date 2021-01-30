use std::fs::read_to_string;

#[derive(Copy, Clone, PartialEq, Debug)]
enum TS {
    Number(u64),
    LeftBracet,
    RightBracket,
    Plus,
    Times,
}

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn tokenize(input: &str) -> Vec<TS> {
    input
        .chars()
        .map(|x| match x {
            '(' => Some(TS::LeftBracet),
            ')' => Some(TS::RightBracket),
            '+' => Some(TS::Plus),
            '*' => Some(TS::Times),
            '0'..='9' => Some(TS::Number(x.to_digit(10).unwrap() as u64)),
            _ => None,
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

fn evaluate_b(input: &[TS]) -> u64 {
    let mut stack: Vec<TS> = Vec::new();

    input.iter().for_each(|x| match x {
        TS::Plus | TS::Times | TS::LeftBracet => stack.push(*x),
        TS::Number(right) => match stack.last() {
            None | Some(TS::LeftBracet) | Some(TS::Times) => {
                stack.push(*x);
            }
            Some(TS::Plus) => {
                let is_plus = stack.pop() == Some(TS::Plus);
                if let Some(TS::Number(left)) = stack.pop() {
                    let value = if is_plus { left + right } else { left * right };
                    stack.push(TS::Number(value));
                } else {
                    panic!("Not enough argument");
                }
            }
            _ => panic!("Unknown state"),
        },
        TS::RightBracket => {
            let mut left = None;
            loop {
                match stack.pop() {
                    Some(TS::Number(n)) => {
                        if left.is_none() {
                            left = Some(n);
                        } else {
                            left = Some(left.unwrap() * n);
                        }
                    }
                    Some(TS::Times) => (),
                    Some(TS::LeftBracet) => break,
                    w @ _ => panic!("Missing left bracket {:?}", w),
                }
            }

            if left.is_none() {
                panic!("Missing bracket value");
            }
            match stack.last() {
                None | Some(TS::Times) | Some(TS::LeftBracet) => {
                    stack.push(TS::Number(left.unwrap()))
                }
                Some(TS::Plus) => {
                    stack.pop();
                    match stack.pop() {
                        Some(TS::Number(right)) => stack.push(TS::Number(left.unwrap() + right)),
                        w @ _ => panic!("There is no number at the stack but {:?}", w),
                    }
                }
                w @ _ => panic!("There is no way to have here {:?}", w),
            }
        }
    });

    let mut left = None;

    loop {
        match stack.pop() {
            Some(TS::Number(n)) => {
                if left.is_none() {
                    left = Some(n);
                } else {
                    left = Some(left.unwrap() * n);
                }
            }
            Some(TS::Times) => (),
            None => return left.unwrap(),
            w @ _ => panic!("Missing left bracket {:?}", w),
        }
    }
}

fn evaluate_a(input: &[TS]) -> u64 {
    let mut stack: Vec<TS> = Vec::new();
    let mut is_a_plus = true;

    input.iter().for_each(|x| match x {
        TS::Plus | TS::Times | TS::LeftBracet => stack.push(*x),
        TS::Number(right) => match stack.last() {
            None | Some(TS::LeftBracet) => {
                stack.push(*x);
            }
            Some(TS::Plus) | Some(TS::Times) => {
                is_a_plus = stack.pop() == Some(TS::Plus);
                if let Some(TS::Number(left)) = stack.pop() {
                    let value = if is_a_plus {
                        left + right
                    } else {
                        left * right
                    };
                    stack.push(TS::Number(value));
                } else {
                    panic!("Not enough argument");
                }
            }
            _ => panic!("Unknown state"),
        },
        TS::RightBracket => {
            let mut left = None;
            loop {
                match stack.pop() {
                    Some(TS::Number(right)) => {
                        if left.is_none() {
                            left = Some(right);
                        } else {
                            left = Some(if is_a_plus {
                                left.unwrap() + right
                            } else {
                                left.unwrap() * right
                            });
                        }
                    }
                    Some(TS::Plus) => is_a_plus = true,
                    Some(TS::Times) => is_a_plus = false,
                    Some(TS::LeftBracet) => break,
                    w @ _ => panic!("Missing left bracket {:?}", w),
                }
            }
            if left.is_none() {
                panic!("Empty bracket");
            }
            match stack.last() {
                None | Some(TS::LeftBracet) => stack.push(TS::Number(left.unwrap())),
                Some(TS::Plus) | Some(TS::Times) => {
                    is_a_plus = stack.pop() == Some(TS::Plus);
                    match stack.pop() {
                        Some(TS::Number(right)) => stack.push(TS::Number(if is_a_plus {
                            left.unwrap() + right
                        } else {
                            left.unwrap() * right
                        })),
                        w @ _ => panic!("There is no number at the stack but {:?}", w),
                    }
                }
                w @ _ => panic!("There is no way to have here {:?}", w),
            }
        }
    });

    if stack.len() != 1 {
        panic!("Stack should hold only a single number");
    }

    match stack.pop() {
        Some(TS::Number(n)) => return n,
        w @ _ => panic!("The only option here is a number, not {:?}", w),
    }
}

fn solution_a(input: &str) -> u64 {
    input
        .lines()
        .map(tokenize)
        .map(|x| evaluate_a(&x))
        .sum::<u64>()
}

fn solution_b(input: &str) -> u64 {
    input
        .lines()
        .map(tokenize)
        .map(|x| evaluate_b(&x))
        .sum::<u64>()
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {:?}", a);
    println!("Step B: {}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_evalute_a() {
        let input = "1+3*(4) + ((5 * 1)   + 4)";
        assert_eq!(evaluate_a(&tokenize(input)), 25);
    }

    #[test]
    fn test_solution_a() {
        assert_eq!(
            content()
                .unwrap()
                .lines()
                .map(|line| evaluate_a(&tokenize(line)))
                .sum::<u64>(),
            5019432542701u64
        );
    }

    #[test]
    fn test_evalute_b() {
        let samples = [
            ("1", 1),
            ("1+1+1 * 3 + 4 * 5 + 6", 231),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];

        for smp in samples.iter() {
            println!("{}", smp.0);
            assert_eq!(evaluate_b(&tokenize(smp.0)), smp.1);
        }
    }

    #[test]
    fn test_solution_b() {
        assert_eq!(
            content()
                .unwrap()
                .lines()
                .map(|line| evaluate_b(&tokenize(line)))
                .sum::<u64>(),
            70518821989947
        );
    }
}
