use std::fmt;
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Amphipod {
    A(u16),
    B(u16),
    C(u16),
    D(u16),
}

impl Amphipod {
    pub fn new(kind: char) -> Self {
        match kind {
            'A' => Amphipod::A(0),
            'B' => Amphipod::B(0),
            'C' => Amphipod::C(0),
            'D' => Amphipod::D(0),
            _ => unreachable!("never"),
        }
    }

    pub fn step(&self, n: u16) -> Self {
        match self {
            Amphipod::A(amount) => Amphipod::A(amount + n),
            Amphipod::B(amount) => Amphipod::B(amount + 10 * n),
            Amphipod::C(amount) => Amphipod::C(amount + 100 * n),
            Amphipod::D(amount) => Amphipod::D(amount + 1000 * n),
        }
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let na = match self {
            Amphipod::A(amount) => ("A", amount),
            Amphipod::B(amount) => ("B", amount),
            Amphipod::C(amount) => ("C", amount),
            Amphipod::D(amount) => ("D", amount),
        };
        write!(f, "{}=>{}", na.0, na.1)
    }
}

struct Burrow {
    base: String,
    positions: [Amphipod; 8],
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
        let positions: [Amphipod; 8] = amphipods.as_slice().try_into().unwrap();
        let base = initial_map
            .replace("A", ".")
            .replace("B", ".")
            .replace("C", ".")
            .replace("D", ".");
        Burrow { base, positions }
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rtv = write!(f, "{}", self.base);
        for a in self.positions {
            rtv = write!(f, "{}", a);
        }
        rtv
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
