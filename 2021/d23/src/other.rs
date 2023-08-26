use std::fmt;

struct Burrow {
    value: u64,
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 8..15 {
            write!(f, "{} ", self.as_char(i))?;
            if i > 8 && i < 13 {
                write!(f, " .  ")?;
            }
        }
        write!(f, "\n  ")?;
        for i in (1..8).step_by(2) {
            write!(f, "   {}  ", self.as_char(i))?;
        }
        write!(f, "\n  ")?;
        for i in (0..8).step_by(2) {
            write!(f, "   {}  ", self.as_char(i))?;
        }
        writeln!(f)
    }
}

impl Burrow {
    fn new(input: &str) -> Self {
        let value = input
            .bytes()
            .filter(|x| x.is_ascii_uppercase())
            .map(|x| 1 + x - b'A')
            .fold((1usize, 0u64), |accum, x| {
                let (mut index, mut value) = accum;
                if index > 7 {
                    index = 0;
                }
                value |= (x as u64 & 7) << (index * 4);
                (index + 2, value)
            })
            .1;
        Burrow { value }
    }

    fn set_val(&self, value: u8, position: u8) -> Self {
        let value = self.value | (value as u64 & 7) << (position * 4);
        Burrow { value }
    }

    fn get_val(&self, position: u8) -> u8 {
        ((self.value >> (position * 4)) & 7u64) as u8
    }

    fn as_char(&self, position: u8) -> char {
        let v = ((self.value >> (position * 4)) & 7u64) as u8;
        if v == 0 {
            '.'
        } else {
            (b'A' - 1 + v) as char
        }
    }

    fn done(&self) -> bool {
        for i in 0..8 {
            if self.get_val(i) != (i >> 1) + 1 {
                return false;
            }
        }
        true
    }

    fn move_to(&self, from: u8, to: u8) -> Option<u8> {
        if self.get_val(to) != 0 || (from > 7 && to > 7) {
            return None;
        }
        let (from, to) = (from.min(to), to.min(from));
        let mut steps = 0;

        let _row_f = if from < 8 {
            steps += 1;
            if from & 1 == 0 {
                if self.get_val(from + 1) != 0 {
                    return None;
                }
                steps = 1;
            }
            9 + (from >> 1)
        } else {
            from
        };

        let _row_t = if to < 8 {
            steps += 2;
            if to & 1 == 0 {
                if self.get_val(to + 1) != 0 {
                    return None;
                }
                steps += 1;
            }
            9 + (to >> 1)
        } else {
            to
        };

        Some(steps)
    }
}

pub fn solve(input: &str) -> usize {
    let value = Burrow::new(input);
    println!("{}", value);
    0
}
