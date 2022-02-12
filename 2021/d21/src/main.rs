use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn read_pos(input: &str) -> Option<(u8, u8)> {
    let nums: Vec<u8> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(": ")
                .skip(1)
                .next()
                .map(|n| u8::from_str_radix(n, 10).ok().unwrap())
        })
        .map(|x| x.unwrap())
        .collect();

    match (nums.get(0), nums.get(1)) {
        (Some(a), Some(b)) => Some((*a, *b)),
        _ => None,
    }
}

struct Dice {
    rolled: usize,
}

impl Dice {
    fn new() -> Self {
        Dice { rolled: 0 }
    }

    pub fn roll(&mut self) -> usize {
        let rtv = (0..3)
            .map(|_i| {
                let a = (self.rolled % 100) + 1;
                self.rolled += 1;
                a
            })
            .sum();
        rtv
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let (a, b) = read_pos(input).unwrap();
    let mut dice = Dice::new();
    let mut n = 0;
    let mut a = a as usize - 1;
    let mut b = b as usize - 1;
    let mut sa = 0;
    let mut sb = 0;

    while sa < 1000 && sb < 1000 {
        let r = dice.roll();
        let plus = r % 10;

        if n % 2 == 0 {
            a = (a + plus) % 10;
            sa += a + 1;
        } else {
            b = (b + plus) % 10;
            sb += b + 1;
        }
        n += 1;
    }
    let rtv = if sa > sb {
        dice.rolled * sb
    } else {
        dice.rolled * sa
    };

    Some(rtv)
}

type Quantum = u32;

fn new_q(a: u8, b: u8) -> Quantum {
    let state: Quantum = (a as u32) << 24 | (b as u32) << 16;

    state
}

fn pos_a(state: Quantum) -> u32 {
    (state >> 24) & 0xff
}

fn pos_b(state: Quantum) -> u32 {
    (state >> 16) & 0xff
}

fn score_a(state: Quantum) -> u32 {
    (state >> 8) & 0xff
}

fn score_b(state: Quantum) -> u32 {
    state & 0xff
}

fn win_a(state: Quantum) -> bool {
    score_a(state) > 20
}

fn win_b(state: Quantum) -> bool {
    score_b(state) > 20
}

fn roll(state: u32, count: usize, odd: bool) -> Vec<(Quantum, usize)> {
    if win_a(state) || win_b(state) {
        return vec![(state, count)];
    }

    let (pos, score, remain) = if odd {
        (pos_a(state), score_a(state), (state & 0x00ff00ff))
    } else {
        (pos_b(state), score_b(state), (state & 0xff00ff00))
    };

    let rtv: Vec<(Quantum, usize)> = [1, 3, 6, 7, 6, 3, 1]
        .iter()
        .zip(3..10)
        .map(|(growth, step)| {
            let p = 1 + (pos + step - 1) % 10;
            let s = score + p;
            (
                if odd {
                    p << 24 | s << 8 | remain
                } else {
                    p << 16 | s | remain
                },
                count * growth,
            )
        })
        .collect();
    rtv
}

fn core_b(input: &str) -> Option<(usize, usize)> {
    let (a, b) = read_pos(input).unwrap();

    let mut sab = (0usize, 0usize);
    let mut state = vec![(new_q(a, b), 1)];

    for i in 1..23 {
        let odd = i & 1 == 1;
        state = state
            .iter()
            .map(|x| roll(x.0, x.1, odd))
            .flatten()
            .collect();

        sab = state.iter().fold((0, 0), |(t_a, t_b), (sn, count)| {
            if win_a(*sn) {
                (t_a + count, t_b)
            } else if win_b(*sn) {
                (t_a, t_b + count)
            } else {
                (t_a, t_b)
            }
        });
        if state
            .iter()
            .find(|(sn, _c)| score_a(*sn) < 21 && score_b(*sn) < 21)
            .is_none()
        {
            break;
        }
    }

    Some(sab)
}

fn solution_b(input: &str) -> Option<usize> {
    core_b(input).map(|(a, b)| a.max(b))
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
        assert_eq!(solution_a(&data), Some(739785));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(core_b(&data), Some((444356092776315, 341960390180808)));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(518418));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(116741133558209));
    }
}
