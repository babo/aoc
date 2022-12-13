use std::fs::read_to_string;
use std::fmt;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct PacketInput {
    lines: String,
    counter: usize,
}

impl PacketInput {
    fn new(lines: &str) -> Self {
        let lines = lines.to_string();
        PacketInput { lines, counter: 0 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PacketLine {
    line: String,
    pos: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Packet {
    left: PacketLine,
    right: PacketLine,
}

impl fmt::Display for PacketLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

impl Packet {
    fn validate(&self) -> bool {
        let mut left: Vec<u8> = self.left.line.bytes().collect();
        let mut right: Vec<u8> = self.right.line.bytes().collect();
        let mut l = 0;
        let mut r = 0;

        loop {
            let a = left.get(l);
            let b = right.get(r);

            if a.is_none() {
                return true;
            }
            if b.is_none() {
                return false;
            }
            let a = a.unwrap();
            let b = b.unwrap();
            if a == b {
                l += 1;
                r += 1;
                continue;
            }

            let mut inc_l = 0;
            let l_num = if a.is_ascii_digit() {
                let mut n = 0;
                while l + inc_l < left.len() {
                    match left.get(l + inc_l) {
                        Some(c) if *c >= b'0' && *c <= b'9' => n = c - b'0' + n * 10,
                        _ => break,
                    }
                    inc_l += 1;
                }
                Some(n)
            } else {
                None
            };
            let mut inc_r = 0;
            let r_num = if b.is_ascii_digit() {
                let mut n = 0;
                while r + inc_r < right.len() {
                    match right.get(r + inc_r) {
                        Some(c) if *c >= b'0' && *c <= b'9' => n = c - b'0' + n * 10,
                        _ => break,
                    }
                    inc_r += 1;
                }
                Some(n)
            } else {
                None
            };
            if l_num.is_some() && r_num.is_some() {
                let ll = l_num.unwrap();
                let rr = r_num.unwrap();
                if ll != rr {
                    return ll < rr;
                }
                l += inc_l;
                r += inc_r;
                continue;
            }
            if *a == b']'  {
                return true;
            }
            if *a == b',' && *b == b']' && r + 1 == right.len() {
                return true;
            }
            if l_num.is_some() {
                left.insert(l + inc_l + 1, b']');
                left.insert(l, b'[');
                println!("Rearrange left {:?}", r_num);
                left.iter().take(l+inc_l+1).for_each(|c| {
                    print!("{}", *c as char);
                });
                println!();
                continue;
            }
            if r_num.is_some() {
                right.insert(r + inc_r + 1, b']');
                right.insert(r, b'[');
                println!("Rearrange right {:?}", l_num);
                right.iter().take(r+inc_r+1).for_each(|c| {
                    print!("{}", *c as char);
                });
                println!();
                continue;
            }

            println!("Between v");
            left.iter().take(l).for_each(|c| {
                print!("{}", *c as char);
            });
            println!();
            right.iter().take(r).for_each(|c| {
                print!("{}", *c as char);
            });
            println!();
            println!("Here ^");

            return false;
        }
    }
}

enum Token {
    Number(i32),
    ListStart,
    ListEnd,
    EOL
}

impl Iterator for PacketLine {
    type Item = (Token, Token);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iterator for PacketInput {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
        let mut lines = self.lines.lines().skip(self.counter * 3);
        let left = lines.next();
        let right = lines.next();

        right
            .map(|r| {
                left.map(|l| {
                    self.counter += 1;
                    let left = PacketLine { line: l.trim().to_string(), pos: 0};
                    let right = PacketLine { line: r.trim().to_string(), pos: 0};
                    Packet {
                        left,
                        right,
                    }
                })
            })
            .flatten()
    }
}

fn solution_a(input: &str) -> usize {
    let packets = PacketInput::new(input);
    packets
        .enumerate()
        .map(|ip| {
            if ip.1.validate() {
                println!("valid {}", ip.0);
                ip.0
            } else {
                println!("invalid {}", ip.0);
                println!("{}", ip.1.left);
                println!("{}", ip.1.right);
                0
            }
        })
        .sum()
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
        assert_eq!(solution_b(&c), Some(0));
    }
}
