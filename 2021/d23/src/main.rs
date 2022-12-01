use std::fmt;
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    pub fn new(kind: char) -> Self {
        match kind {
            'A' => Amphipod::A,
            'B' => Amphipod::B,
            'C' => Amphipod::C,
            'D' => Amphipod::D,
            _ => unreachable!("never"),
        }
    }

    pub fn energy(&self, n: u16) -> u16 {
        match self {
            Amphipod::A => n,
            Amphipod::B => 10 * n,
            Amphipod::C => 100 * n,
            Amphipod::D => 1000 * n,
        }
    }

    pub fn room(&self) -> u16 {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 2,
            Amphipod::C => 4,
            Amphipod::D => 6,
        }
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let na = match self {
            Amphipod::A => "A",
            Amphipod::B => "B",
            Amphipod::C => "C",
            Amphipod::D => "D",
        };
        write!(f, "{}", na)
    }
}

struct Burrow {
    base: String,
    amphipods: [Amphipod; 8],
    positions: [Option<usize>; 15],
    energy: usize,
}

impl Burrow {
    pub fn new(initial_map: &str) -> Self {
        let dots = initial_map.chars().filter(|x| *x == '.').count();
        println!("Count: {dots}");
        let amphipods: Vec<Amphipod> = initial_map
            .replace(" ", "")
            .replace("#", "")
            .replace(".", "")
            .replace("\n", "")
            .chars()
            .map(|c| Amphipod::new(c))
            .collect();
        let amphipods: [Amphipod; 8] = amphipods.as_slice().try_into().unwrap();
        let base = initial_map
            .replace("A", ".")
            .replace("B", ".")
            .replace("C", ".")
            .replace("D", ".");
        let positions = [
            Some(4usize),
            Some(5),
            Some(6),
            Some(7),
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        Burrow {
            base,
            amphipods,
            positions,
            energy: 0usize,
        }
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rtv = write!(f, "{}", self.base);
        for a in self.amphipods {
            rtv = writeln!(f, "{}", a);
        }
        write!(f, "Energy: {}", self.energy)
    }
}

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let b = Burrow::new(input);
    println!("{}", b);
    None
}

fn solution_b(_input: &str) -> Option<usize> {
    None
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
        assert_eq!(solution_a(&data), Some(12521));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(0));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
