use std::fmt;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PacketLine {
    line: String,
    pos: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Number(i32),
    ListStart,
    ListEnd,
}

impl PacketLine {
    fn new(line: &str) -> Self {
        PacketLine { line: line.to_string(), pos: 0}
    }
}

impl Iterator for PacketLine {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.line.chars().nth(self.pos) {
            None => None,
            Some('[') => {
                self.pos += 1;
                Some(Token::ListStart)
            }
            Some(']') => {
                self.pos += 1;
                Some(Token::ListEnd)
            }
            Some(',') => {
                self.pos += 1;
                PacketLine::next(self)
            }
            Some(c) if c.is_ascii_digit() => {
                let mut num = 0i32;
                let mut c = Some(c);
                while c.map_or(false, |c| c.is_ascii_digit()) {
                    c.map(|c| num = num * 10 + c as i32 - '0' as i32);
                    self.pos += 1;
                    c = self.line.chars().nth(self.pos + 1);
                }
                Some(Token::Number(num))
            }
            _ => unimplemented!("What?"),
        }
    }
}

impl fmt::Display for PacketLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

fn solution_a(input: &str) -> usize {
    let lines:Vec<_> = input.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).map(|line|
        PacketLine::new(line)).collect();

    lines.iter().step_by(2).zip(
    lines.iter().skip(1).step_by(2)).map(|(a, b)| {
        println!("{a} {b}");
        0
    }).sum()
}

fn solution_b(_input: &str) -> usize {
    0
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
        assert_eq!(solution_a(&data), 13);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 140);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 5366);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 23391);
    }

    #[test]
    fn test_packet_parser() {
        let line = "[1,[2]]";
        let pl = PacketLine {
            line: line.to_string(),
            pos: 0,
        };

        let mut it = pl.into_iter();

        let token = it.next();
        assert!(token.is_some());
        assert_eq!(token.unwrap(), Token::ListStart);

        let token = it.next();
        assert!(token.is_some());
        assert_eq!(token.unwrap(), Token::Number(1));

        let token = it.next();
        assert!(token.is_some());
        assert_eq!(token.unwrap(), Token::ListStart);

        let token = it.next();
        assert!(token.is_some());
        assert_eq!(token.unwrap(), Token::Number(2));

        let token = it.next();
        assert!(token.is_some());
        assert_eq!(token.unwrap(), Token::ListEnd);

        let token = it.next();
        assert!(token.is_some());
        assert_eq!(token.unwrap(), Token::ListEnd);

        let token = it.next();
        assert!(token.is_none());
    }
}
