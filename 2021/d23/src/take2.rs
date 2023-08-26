use std::fmt;

struct Burrow {
    value: u64,
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 4..11 {
            let v = ((self.value >> (i & 7) * 3) & 7) as u8;
            write!(f, "{}{}", v, if i == 7 { '\n' } else { ' ' })?;
        }
        write!(f, "{}", self.value >> 9 & 7)
    }
}

impl Burrow {
    const DONE: u64 = 1 << 0 | 2 << 3 | 3 << 6 | 4 << 9 | 1 << 12 | 2 << 15 | 3 << 18 | 4 << 21;

    fn new(input: &str) -> Self {
        let (value, _) = input
            .bytes()
            .filter(|x| x.is_ascii_uppercase())
            .map(|x| 1 + x - b'A')
            .fold((0u64, 4u8), |acc, x| {
                (
                    acc.0 | ((x & 7u8) as u64) << (acc.1 * 3u8),
                    (acc.1 + 1u8) & 7u8,
                )
            });
        Burrow { value }
    }

    fn ready(&self) -> bool {
        self.value == Self::DONE
    }

    fn set(&mut self, pos: u8, val: u8) -> u64 {
        self.value = !(7u64 << (pos & 7) * 3) & self.value;
        self.value = self.value | ((val as u64 & 7) << (pos & 7) * 3);
        self.value
    }

    fn get(&self, pos: u8) -> u8 {
        ((self.value >> (pos & 7) * 3) & 7u64) as u8
    }

    fn free_between(&self, p1: u8, p2: u8) -> bool {
        let (p1, p2) = (10 + 5 * (p1.min(p2) % 4), 10 + 5 * (p1.max(p2) % 4));

        for p in p1..p2 {
            if self.get(p) != 0 {
                return false;
            }
        }
        true
    }

    fn distance(&self, p1: u8, p2: u8) -> u8 {
        let (p1, p2) = (p1.min(p2), p1.max(p2));
        assert!(p1 < 8);
        if p2 > 7 {
            let t1 = 10 + (p1 & 3) * 2;
            2 - p1 / 4 + if p2 < t1 { t1 - p2 } else { p2 - t1 }
        } else {
            4 - (p1 / 4) - (p2 / 4) + 2 * ((p2 % 4) - (p1 % 4))
        }
    }

    fn direct(&self, pos: u8) -> Option<u8> {
        let v = self.get(pos);
        let w = self.get((pos + 4) & 7);

        if pos < 4 && (v == pos + 1 || w != 0) {
            return None;
        }
        if pos >= 4 && (v == pos + 1 && w == pos + 1) {
            return None;
        }
        let dest = (v - 1) * 2;
        let dv = self.get(dest);
        if dv == 0 && self.free_between(pos, dest) {
            return Some(dest);
        }
        if dv == v && self.get(dest + 4) == 0 && self.free_between(pos, dest) {
            return Some(dest + 4);
        }
        None
    }

    fn where_to(&self, pos: u8) -> u8 {
        let v = self.get(pos);
        if v != 1 + (pos & 3) {
            if self.get(v - 1) == 0u8 {
                v - 1
            } else if self.get(v + 3) == 0u8 {
                v + 3
            } else {
                if v < (pos & 3) + 1 {
                    (v + 4) * 2 + 1
                } else {
                    (v + 4) * 2 - 1
                }
            }
        } else {
            let w = self.get(pos & 3);
            if pos >= 4 && v != w {
                let top = (pos + 1) * 2;
                if w < v {
                    top + 1
                } else {
                    top - 1
                }
            } else {
                pos
            }
        }
    }

    fn needs_to_move(&self, pos: u8) -> bool {
        let v = ((self.value >> pos * 3) & 7) as u8;
        let w = if pos >= 4 {
            ((self.value >> (pos - 4) * 3) & 7) as u8
        } else {
            ((self.value >> (pos + 4) * 3) & 7) as u8
        };
        let in_place =
            (pos < 4 && v == (pos + 1)) || (pos >= 4 && v == (pos - 3) && w == (pos - 3));
        !in_place
    }

    fn should_move(&self, pos: u8) -> bool {
        self.needs_to_move(pos) && (pos >= 4 || self.get(pos + 4) == 0u8)
    }
}

pub fn solve(input: &str) -> usize {
    println!("{input}");
    let b = Burrow::new(input);
    println!("{b}");
    for pos in 0u8..8 {
        println!("{pos} {}", if b.should_move(pos) { b.get(pos) } else { 0 });
    }
    for pos in 0..8 {
        if b.should_move(pos) {
            println!("{pos} ({}) => {}", b.get(pos), b.where_to(pos));
        }
    }

    b.value as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn simple() -> Option<String> {
        read_to_string("./simple.txt").ok()
    }

    #[test]
    fn test_2go() {
        let data = simple().unwrap();
        let b = Burrow::new(&data);

        assert_eq!(1, b.distance(4, 10));
        assert_eq!(2, b.distance(0, 10));
        assert_eq!(3, b.distance(0, 11));
        assert_eq!(3, b.distance(0, 9));
        assert_eq!(4, b.distance(4, 5));
        assert_eq!(5, b.distance(0, 5));
        assert_eq!(6, b.distance(0, 1));
        assert_eq!(10, b.distance(0, 3));
    }
}
