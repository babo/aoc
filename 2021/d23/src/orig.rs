use std::fmt;

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
    if state[pos_t] != 0 {
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

    if pos_t < 8 {
        if pos_f & 4 == 0 && state[pos_f + 4] != 0 {
            return None;
        }
        if pos_t & 4 == 0 && state[pos_t + 4] != 0 {
            return None;
        }
        cost += if pos_f & 4 == 4 { 1 } else { 2 };
        cost += if pos_t & 4 == 4 { 1 } else { 2 };
        let mut a_f = pos_f & 3;
        let mut a_t = pos_t & 3;
        if a_f > a_t {
            let tmp = a_f;
            a_f = a_t;
            a_t = tmp;
        }
        cost += 2 * (a_t - a_f);
        for i in (10 + a_f)..(10 + a_t) {
            if state[i] != 0 {
                return None;
            }
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
    println!("Calls {calls} {:?}", s);
    if ready(&s) {
        println!("Solved {energy}");
        return Some(energy);
    }
    if calls > 255 {
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
                            println!("F");
                        }
                        None => (),
                    }
                } else if s[above] == 0 && s[goal] == val {
                    match move_to(from, above, &s) {
                        Some(cost) => {
                            s[from] = 0;
                            s[above] = val;
                            modified = true;
                            energy += cost * 10usize.pow(val - 1);
                            println!("A");
                        }
                        None => (),
                    }
                }
            }
        }
        for from in 0..4 {
            let top = from + 4;
            let val = (from + 1) as u32;
            if s[top] != 0 && s[top] != val {
                let val = s[top];
                let pos = (val - 1) as usize;
                if s[pos] == 0 {
                    match move_to(top, pos, &s) {
                        Some(cost) => {
                            s[top] = 0;
                            s[pos] = val;
                            modified = true;
                            energy += cost * 10usize.pow(val - 1);
                        }
                        None => (),
                    }
                } else if s[pos] == val && s[pos + 1] == 0 {
                    let above = pos + 4;
                    match move_to(top, above, &s) {
                        Some(cost) => {
                            s[top] = 0;
                            s[above] = val;
                            modified = true;
                            energy += cost * 10usize.pow(val - 1);
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

    for from in (1usize..4).rev() {
        let val = (from + 1) as u32;
        if s[from] == 0 && s[from + 4] == 0 {
            for f in 0..from {
                if s[f] == val && s[f + 4] != 0 {
                    let f = f + 4;
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
            }
        }
    }

    for from in (0usize..4).rev() {
        let above = from + 4;
        let val = (from + 1) as u32;
        let f = if s[above] == 0 && s[from] != val && s[from] != 0 {
            from
        } else if s[above] != 0 && (s[above] != val || s[from] != val) {
            above
        } else {
            continue;
        };

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


#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_blocks() {
        let mut state = [0u32; 15];

        state[4] = 1;
        assert_eq!(move_to(0, 8, &state), None);
        state[4] = 0;
        state[9] = 1;
        assert_eq!(move_to(0, 8, &state), None);
        state[9] = 0;
        state[8] = 1;
        assert_eq!(move_to(0, 8, &state), None);
        state[8] = 0;

        state[5] = 1;
        assert_eq!(move_to(0, 1, &state), None);
        state[5] = 0;
        state[10] = 1;
        assert_eq!(move_to(0, 1, &state), None);
        state[10] = 0;

        state[11] = 1;
        assert_eq!(move_to(0, 3, &state), None);
        state[11] = 0;
        state[12] = 1;
        assert_eq!(move_to(0, 3, &state), None);
        state[12] = 0;
        state[7] = 1;
        assert_eq!(move_to(0, 3, &state), None);
        state[7] = 0;

        state[13] = 1;
        assert_eq!(move_to(0, 14, &state), None);
        state[13] = 0;
    }
}
