use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    High,
    One,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Player {
    hand: HandType,
    values: (u32, u32, u32, u32, u32),
    bid: u32,
}

fn calc_hand_1(values: (u32, u32, u32, u32, u32)) -> HandType {
    let a: [u32; 5] = [values.0, values.1, values.2, values.3, values.4];
    let s = a
        .iter()
        .sorted()
        .copied()
        .collect_tuple::<(u32, u32, u32, u32, u32)>()
        .unwrap();
    let hashed: HashSet<u32> = HashSet::from(a);

    if s.0 == s.4 {
        HandType::Five
    } else if s.0 == s.3 {
        HandType::Four
    } else if s.1 == s.4 {
        HandType::Four
    } else if s.0 == s.2 && s.3 == s.4 {
        HandType::FullHouse
    } else if s.0 == s.1 && s.2 == s.4 {
        HandType::FullHouse
    } else if s.0 != s.1 && s.1 != s.2 && s.2 == s.4 {
        HandType::Three
    } else if s.0 != s.1 && s.3 != s.4 && s.1 == s.3 {
        HandType::Three
    } else if s.2 != s.3 && s.3 != s.4 && s.0 == s.2 {
        HandType::Three
    } else if hashed.len() == 3 {
        HandType::TwoPairs
    } else if hashed.len() == 5 {
        HandType::High
    } else {
        HandType::One
    }
}

fn calc_hand_2(values: (u32, u32, u32, u32, u32)) -> HandType {
    let a: [u32; 5] = [values.0, values.1, values.2, values.3, values.4];
    let s = a
        .iter()
        .sorted()
        .copied()
        .collect_tuple::<(u32, u32, u32, u32, u32)>()
        .unwrap();
    let hashed: HashSet<u32> = HashSet::from(a);

    if hashed.len() == 1 {
        HandType::Five
    } else if s.0 == s.3 || s.1 == s.4 {
        if s.0 == 1 {
            HandType::Five
        } else {
            HandType::Four
        }
    } else if s.0 == s.2 && s.3 == s.4 {
        if s.0 == 1 {
            HandType::Five
        } else {
            HandType::FullHouse
        }
    } else if s.0 == s.1 && s.2 == s.4 {
        if s.0 == 1 {
            HandType::Five
        } else {
            HandType::FullHouse
        }
    } else if s.0 != s.1 && s.1 != s.2 && s.2 == s.4 {
        if s.0 == 1 {
            HandType::Four
        } else {
            HandType::Three
        }
    } else if s.0 != s.1 && s.3 != s.4 && s.1 == s.3 {
        if s.0 == 1 {
            HandType::Four
        } else {
            HandType::Three
        }
    } else if s.0 == s.2 && s.2 != s.3 && s.3 != s.4 {
        if s.0 == 1 {
            HandType::Four
        } else {
            HandType::Three
        }
    } else if hashed.len() == 3 {
        if s.0 == 1 {
            if s.1 == 1 {
                HandType::Four
            } else {
                HandType::FullHouse
            }
        } else {
            HandType::TwoPairs
        }
    } else if hashed.len() == 5 {
        if s.0 == 1 {
            HandType::One
        } else {
            HandType::High
        }
    } else {
        if s.0 == 1 {
            HandType::Three
        } else {
            HandType::One
        }
    }
}

impl Player {
    const STRENGTH_1: &'static str = "**23456789TJQKA";
    const STRENGTH_2: &'static str = "*J23456789TQKA";

    fn new(line: &str, part_1: bool) -> Self {
        let bid = line
            .chars()
            .skip(6)
            .fold(0, |accu, x| accu * 10 + x.to_digit(10).unwrap());
        let cards = line
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .chars()
            .map(|x| {
                (if part_1 {
                    Player::STRENGTH_1
                } else {
                    Player::STRENGTH_2
                })
                .chars()
                .position(|c| c == x)
                .unwrap() as u32
            })
            .collect_vec();
        let values: (u32, u32, u32, u32, u32) = cards
            .iter()
            .copied()
            .collect_tuple::<(u32, u32, u32, u32, u32)>()
            .unwrap();
        let hand = if part_1 {
            calc_hand_1(values)
        } else {
            calc_hand_2(values)
        };
        Player { hand, values, bid }
    }
}

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn calc_checksum(input: &str, part_1: bool) -> Option<usize> {
    let mut hands = input
        .lines()
        .map(|x| Player::new(x.trim(), part_1))
        .collect_vec();
    hands.sort_by(|a, b| {
        if a.hand != b.hand {
            a.hand.cmp(&b.hand)
        } else {
            a.values.cmp(&b.values)
        }
    });
    hands.iter().for_each(|x| println!("{:?}", x));

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
            .sum(),
    )
}
fn solution_a(input: &str) -> Option<usize> {
    calc_checksum(input, true)
}

fn solution_b(input: &str) -> Option<usize> {
    calc_checksum(input, false)
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
    fn test_hands() {
        assert_eq!(Player::new("AAAAA", true).hand, HandType::Five);
        assert_eq!(Player::new("AA8AA", true).hand, HandType::Four);
        assert_eq!(Player::new("23332", true).hand, HandType::FullHouse);
        assert_eq!(Player::new("TTT98", true).hand, HandType::Three);
        assert_eq!(Player::new("23432", true).hand, HandType::TwoPairs);
        assert_eq!(Player::new("A23A4", true).hand, HandType::One);
        assert_eq!(Player::new("23456", true).hand, HandType::High);
    }

    #[test]
    fn test_ordering() {
        assert!(HandType::Five == HandType::Five);
        assert!(HandType::Five > HandType::Four);
        assert!(HandType::Four == HandType::Four);
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(6440));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(5905));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(251106089));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(249620106));
    }
}
