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

    pub fn as_num(&self) -> u32 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 2,
            Amphipod::C => 3,
            Amphipod::D => 4,
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

fn free_to(pos_f: u32, pos_t: u32, state: &[u32; 15]) -> Option<usize> {
    if state[pos_t as usize] != 0 {
        return None;
    }
    if pos_f == pos_t {
        return Some(0);
    }
    let mut pos_t = pos_t;
    let mut pos_f = pos_f;
    if pos_t < pos_f {
        let tmp = pos_t;
        pos_t = pos_f;
        pos_f = tmp;
    }
    let mut cost = 0usize;
    if pos_f < 4 {
        pos_f += 4u32;
        if state[pos_f as usize] != 0 {
            return None;
        }
        cost += 1;
    }
    if pos_f < 8 {
        cost += 1;
        pos_f += 6;
    }
    if pos_f > pos_t {
        let tmp = pos_f;
        pos_f = pos_t;
        pos_t = tmp;
    }
    if state[pos_f as usize..pos_t as usize].iter().sum::<u32>() != 0 {
        return None;
    }

    Some(cost + (pos_t - pos_f) as usize)
}

fn solve(s: [u32; 15], energy: usize) -> Option<usize> {
    if s[0] == 1
        && s[4] == 1
        && s[1] == 2
        && s[5] == 2
        && s[2] == 3
        && s[6] == 3
        && s[3] == 4
        && s[7] == 4
    {
        return Some(energy);
    }
    for j in 8..15 {
        if s[j] != 0 {
            let i = (s[j] - 1) as usize;
            if s[i] == 0 {
                match free_to(j as u32, i as u32, &s) {
                    Some(cost) => {
                        let mut sub = s;
                        sub[j] = 0;
                        sub[i] = (i + 1) as u32;
                        return solve(sub, energy + cost);
                    }
                    None => (),
                }
            } else if s[i + 4] == 0 && s[i] == (i + 1) as u32 {
                let t = i + 4;
                match free_to(j as u32, t as u32, &s) {
                    Some(cost) => {
                        let mut sub = s;
                        sub[j] = 0;
                        sub[t] = (i + 1) as u32;
                        return solve(sub, energy + cost);
                    }
                    None => (),
                }
            }
        }
    }

    let mut m = None;
    for i in 0u32..4 {
        let f = if s[(i + 4) as usize] == 0 && s[i as usize] != i + 1 {
            Some(i)
        } else if s[(i + 4) as usize] != 0 && s[(i + 4) as usize] != i + 1 {
            Some(i + 4)
        } else {
            None
        };
        match f {
            Some(f) => {
                for t in 8..15 {
                    match free_to(f, t, &s) {
                        Some(cost) => {
                            let cost = cost * 10usize.pow(s[f as usize]);
                            let mut sub = s;
                            sub[t as usize] = sub[f as usize];
                            sub[f as usize] = 0;
                            match solve(sub, energy + cost) {
                                Some(total) => {
                                    if m.map_or(true, |x| x > total) {
                                        m = Some(total);
                                    }
                                }
                                None => (),
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
    m
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
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
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

    pub fn solve(&self) -> Option<usize> {
        let state = self.positions.map(|x| match x {
            None => 0u32,
            Some(m) => self.amphipods[m].as_num(),
        });
        solve(state, 0)
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base)?;
        for i in 8..15 {
            match self.positions[i] {
                None => {
                    write!(f, ".")?;
                }
                Some(i) => {
                    let c = self.amphipods[i].to_string();
                    write!(f, "{}", c.as_str())?;
                }
            }
            if i > 8 && i < 13 {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        for i in 0..8 {
            if i % 4 == 0 {
                write!(f, " ")?;
            }
            match self.positions[i] {
                None => {
                    write!(f, " .")?;
                }
                Some(i) => {
                    let c = self.amphipods[i].to_string();
                    write!(f, " {}", c.as_str())?;
                }
            }
            if i == 3 {
                writeln!(f)?;
            }
        }
        write!(f, "\nEnergy: {}", self.energy)
    }
}

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let b = Burrow::new(input);
    println!("{}", b);
    b.solve()
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
