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
}

fn move_to(pos_f: usize, pos_t: usize, state: &[u32; 15]) -> Option<usize> {
    println!("move_to {pos_f} {pos_t}");

    if state[pos_t] != 0 || state[pos_f] != 0 {
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

    println!("ft {pos_f} {pos_t} {cost}");
    if pos_t < 8 {
        cost += 3;
        let d = pos_t - pos_f;
        if d < 4 {
            cost += d * 2;
            if pos_t < 4 {
                cost += 1;
            }
        } else {
            cost += (d - 4) * 2;
        }
    } else {
        if pos_f < 4 {
            if state[pos_f + 4] != 0 {
                return None;
            }
            pos_f += 9;
            cost += 3;
        } else {
            pos_f += 5;
            cost += 2;
        }
        if pos_t <= pos_f {
            cost += (pos_f - pos_t) * 2;
            if pos_t == 8 {
                cost -= 1;
            }
            if state[pos_t..pos_f + 1].iter().sum::<u32>() != 0 {
                return None;
            }
        } else {
            cost += (pos_t - pos_f - 1) * 2;
            if pos_t == 14 {
                cost -= 1;
            }
            if state[pos_f + 1..pos_t + 1].iter().sum::<u32>() != 0 {
                return None;
            }
        }
    }
    Some(cost)
}

fn ready(s: &[u32; 15]) -> bool {
    s[0] == 1
        && s[4] == 1
        && s[1] == 2
        && s[5] == 2
        && s[2] == 3
        && s[6] == 3
        && s[3] == 4
        && s[7] == 4
}

fn solve(s: [u32; 15], energy: usize, calls: usize) -> Option<usize> {
    //println!("Calls {calls} {:?}", s);
    if ready(&s) {
        println!("Solved {energy}");
        return Some(energy);
    }
    if calls > 15 {
        return None;
    }

    let mut s = s;
    let mut energy = energy;
    let mut modified = true;
    while modified {
        modified = false;
        for from in 8..15 {
            if s[from] != 0 {
                let val = s[from];
                let goal = (val - 1) as usize;
                let above = goal + 4;
                if s[goal] == 0 {
                    match move_to(from, goal, &s) {
                        Some(cost) => {
                            s[from] = 0;
                            s[goal] = val;
                            modified = true;
                            energy += cost * 10usize.pow(val - 1);
                            //println!("Move to home: {val} from {from}");
                        }
                        None => (),
                    }
                }
                if s[above] == 0 && s[goal] == val {
                    match move_to(from, above, &s) {
                        Some(cost) => {
                            s[from] = 0;
                            s[above] = val;
                            modified = true;
                            energy += cost * 10usize.pow(val - 1);
                            //println!("Move to home above: {val} from {from}");
                        }
                        None => (),
                    }
                }
            }
        }
    }
    if ready(&s) {
        println!("Solved {energy}");
        return Some(energy);
    }

    let mut m = None;
    for from in 0usize..4 {
        let above = from + 4;
        let val = (from + 1) as u32;
        let f = if s[above] == 0 && s[from] != val {
            Some(from)
        } else if s[above] != 0 && s[above] != val {
            Some(above)
        } else if s[above] != 0 && s[above] == val && s[from] != val {
            Some(above)
        } else {
            None
        };
        //println!("A {from} {above} {:?}", f);
        match f {
            Some(f) => {
                let val = s[f];
                for t in 8..15 {
                    match move_to(f, t, &s) {
                        Some(cost) => {
                            let mut sub = s.clone();
                            sub[f] = 0;
                            sub[t] = val;
                            match solve(sub, energy + cost * 10usize.pow(val - 1), calls + 1) {
                                Some(total) => {
                                    println!("Total {total}");
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
        let amphipods: Vec<Amphipod> = initial_map
            .replace(" ", "")
            .replace("#", "")
            .replace(".", "")
            .replace("\n", "")
            .chars()
            .map(|c| Amphipod::new(c))
            .collect();
        let amphipods: [Amphipod; 8] = amphipods.as_slice().try_into().unwrap();
        let base = initial_map.to_string();
        let positions = [
            Some(4),
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
        }
    }

    pub fn solve(&self) -> Option<usize> {
        let state = self.positions.map(|x| match x {
            Some(m) => self.amphipods[m].as_num(),
            None => 0u32,
        });
        solve(state, 0, 0)
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.base)
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
    fn test_moves() {
        let state = [0u32; 15];

        assert_eq!(move_to(0, 0, &state), Some(0));
        assert_eq!(move_to(0, 8, &state), Some(4));
        assert_eq!(move_to(0, 9, &state), Some(3));
        assert_eq!(move_to(0, 10, &state), Some(3));
        assert_eq!(move_to(0, 11, &state), Some(5));
        assert_eq!(move_to(0, 14, &state), Some(10));
        assert_eq!(move_to(0, 1, &state), Some(6));
        assert_eq!(move_to(1, 0, &state), Some(6));
        assert_eq!(move_to(1, 4, &state), Some(5));
        assert_eq!(move_to(0, 5, &state), Some(5));
        assert_eq!(move_to(3, 8, &state), Some(10));
        assert_eq!(move_to(3, 0, &state), Some(10));
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
