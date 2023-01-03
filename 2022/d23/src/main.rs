use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Magma {
    elves: HashSet<(usize, usize)>,
    direction: usize,
}

impl Magma {
    fn new(input: &str) -> Self {
        const SHIFT: usize = 100;
        let mut elves = HashSet::new();

        input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .enumerate()
            .for_each(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .for_each(|(col, _)| {
                        elves.insert((row + SHIFT, col + SHIFT));
                    });
            });

        Magma {
            elves,
            direction: 0,
        }
    }

    fn ready(&self) -> bool {
        self.elves
            .iter()
            .map(|rc| {
                let (r, c) = *rc;
                let nw = self.elves.contains(&(r - 1, c - 1));
                let n = self.elves.contains(&(r - 1, c));
                let ne = self.elves.contains(&(r - 1, c + 1));
                let e = self.elves.contains(&(r, c + 1));
                let se = self.elves.contains(&(r + 1, c + 1));
                let s = self.elves.contains(&(r + 1, c));
                let sw = self.elves.contains(&(r + 1, c - 1));
                let w = self.elves.contains(&(r, c - 1));

                n || ne || e || se || s || sw || w || nw
            })
            .find(|x| *x)
            .is_none()
    }

    fn round(&self) -> Self {
        const DIR: [(i32, i32); 5] = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut cntrl = HashMap::<(usize, usize), usize>::new();

        let it = self.elves.iter().map(|rc| {
            let (r, c) = *rc;
            let nw = self.elves.contains(&(r - 1, c - 1));
            let n = self.elves.contains(&(r - 1, c));
            let ne = self.elves.contains(&(r - 1, c + 1));
            let e = self.elves.contains(&(r, c + 1));
            let se = self.elves.contains(&(r + 1, c + 1));
            let s = self.elves.contains(&(r + 1, c));
            let sw = self.elves.contains(&(r + 1, c - 1));
            let w = self.elves.contains(&(r, c - 1));

            let mut new_pos = 0;
            if n || ne || e || se || s || sw || w || nw {
                for i in self.direction..self.direction + 4 {
                    let dir = i % 4;
                    let occupied = match dir {
                        0 => nw || n || ne,
                        1 => se || s || sw,
                        2 => nw || w || sw,
                        _ => ne || e || se,
                    };
                    if !occupied {
                        new_pos = dir + 1;
                        break;
                    }
                }
            }

            let (dr, dc) = DIR[new_pos];
            let (nr, nc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);

            if let Some(p) = cntrl.get_mut(&(nr, nc)) {
                *p += 1;
            } else {
                cntrl.insert((nr, nc), 1);
            }
            ((r, c), (nr, nc))
        });
        let plan = HashMap::<(usize, usize), (usize, usize)>::from_iter(it);

        let real = self.elves.iter().map(|orig| {
            let next = plan.get(orig).unwrap();
            if cntrl.get(next) == Some(&1) {
                plan.get(next).map_or(*next, |other| {
                    if cntrl.get(other) == Some(&1) {
                        *next
                    } else {
                        *orig
                    }
                })
            } else {
                *orig
            }
        });
        let elves = HashSet::from_iter(real);
        assert_eq!(self.elves.len(), elves.len());

        Magma {
            elves,
            direction: (self.direction + 1) % 4,
        }
    }

    fn bounding_box(&self) -> ((usize, usize), (usize, usize)) {
        let (rmin, rmax) = self
            .elves
            .iter()
            .map(|(r, _)| r)
            .minmax()
            .into_option()
            .unwrap();
        let (cmin, cmax) = self
            .elves
            .iter()
            .map(|(_, c)| c)
            .minmax()
            .into_option()
            .unwrap();
        ((*rmin, *rmax), (*cmin, *cmax))
    }

    fn count_empty(&self) -> usize {
        let ((rmin, rmax), (cmin, cmax)) = self.bounding_box();
        (rmax + 1 - rmin) * (cmax + 1 - cmin) - self.elves.len()
    }

    fn display(&self) {
        let ((rmin, rmax), (cmin, cmax)) = self.bounding_box();

        for r in rmin..rmax + 1 {
            for c in cmin..cmax + 1 {
                print!(
                    "{}",
                    if self.elves.contains(&(r, c)) {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!()
        }
        println!()
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let m = Magma::new(input);
    //println!("== Initial State ==");
    //m.display();
    let f = (0..10).fold(m, |prev, i| {
        //println!("== End of Round {} ==", i + 1);
        let n = prev.round();
        //n.display();
        n
    });

    Some(f.count_empty())
}

fn solution_b(input: &str) -> Option<usize> {
    let mut m = Magma::new(input);
    for i in 1..1000 {
        if m.ready() {
            return Some(i);
        }
        m = m.round();
    }
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
        assert_eq!(solution_a(&data), Some(110));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(20));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(3757));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(918));
    }
}
