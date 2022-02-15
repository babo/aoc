use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Cuboid {
    on_off: bool,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cuboid {
    fn new(line: &str) -> Self {
        let line = String::from(line) + " ";
        let on_off = match line.get(0..3) {
            Some("on ") => true,
            Some("off") => false,
            _ => unreachable!("Should start with on or off"),
        };
        let coordinates: Vec<i32> = line
            .chars()
            .fold((Vec::new(), String::new()), |mut acc, c| {
                if char::is_numeric(c) || (c == '-' && acc.1.is_empty()) {
                    acc.1.push(c);
                    acc
                } else if !acc.1.is_empty() {
                    let n = i32::from_str_radix(acc.1.as_str(), 10).unwrap();
                    acc.0.push(n);
                    (acc.0, String::new())
                } else {
                    acc
                }
            })
            .0;

        Cuboid {
            on_off,
            x1: *coordinates.get(0).unwrap(),
            x2: *coordinates.get(1).unwrap(),
            y1: *coordinates.get(2).unwrap(),
            y2: *coordinates.get(3).unwrap(),
            z1: *coordinates.get(4).unwrap(),
            z2: *coordinates.get(5).unwrap(),
        }
    }

    fn small(&self) -> bool {
        self.x1 > -50
            && self.x2 < 50
            && self.y1 > -50
            && self.y2 < 50
            && self.z1 > -50
            && self.z2 < 50
    }

    fn cubes(&self) -> usize {
        ((self.x2 - self.x1) * (self.y2 - self.y1) * (self.z2 - self.z1)) as usize
    }

    fn union(&self, other: &Self) -> Option<Self> {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);

        if x1 >= x2 || y1 >= y2 || z1 >= z2 {
            None
        } else {
            Some(Cuboid {
                on_off: true,
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        }
    }
}

fn read_input(input: &str, full_size: bool) -> Vec<Cuboid> {
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(Cuboid::new)
        .filter(|c| full_size || c.small())
        .collect::<Vec<Cuboid>>()
}

fn solution_a(input: &str) -> Option<usize> {
    read_input(input, false);
    None
}

fn solution_b(input: &str) -> Option<usize> {
    read_input(input, true);
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
    fn test_mini_a() {
        let data = "on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13
        off x=9..11,y=9..11,z=9..11
        on x=10..10,y=10..10,z=10..10";
        assert_eq!(solution_a(&data), Some(39));
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(590784));
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
