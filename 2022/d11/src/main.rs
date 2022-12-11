use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct MonkeyInput {
    source: String,
    pos: usize,
}

impl MonkeyInput {
    fn new(input: &str) -> Self {
        MonkeyInput {
            source: input.to_string(),
            pos: 0,
        }
    }
}

struct Monkey {
    items: Vec<u128>,
    operation: (Option<usize>, char, Option<usize>),
    div: u128,
    partners: (usize, usize),
    inspected: usize,
}

impl Iterator for MonkeyInput {
    type Item = Monkey;

    fn next(&mut self) -> Option<Monkey> {
        if self.pos >= self.source.len() {
            return None;
        }
        let mut it = self.source.lines().skip(self.pos * 7);
        let name = it.next();
        if name.is_none() {
            return None;
        }
        let items: Vec<u128> = Vec::from_iter(
            it.next()
                .unwrap()
                .get(18..)
                .unwrap()
                .split(", ")
                .map(|x| u128::from_str_radix(x, 10).map(|x| x).unwrap()),
        );
        let operation: Vec<&str> = it.next().unwrap().get(19..).unwrap().split(' ').collect();
        let div = it
            .next()
            .unwrap()
            .get(21..)
            .map(|x| u128::from_str_radix(x, 10).ok().unwrap())
            .unwrap();
        let m1 = it
            .next()
            .map(|x| {
                let n = x.find(|c: char| c.is_ascii_digit()).unwrap();
                x.get(n..).map(|x| usize::from_str_radix(x, 10).unwrap())
            })
            .unwrap()
            .unwrap();
        let m2 = it
            .next()
            .map(|x| {
                let n = x.find(|c: char| c.is_ascii_digit()).unwrap();
                x.get(n..).map(|x| usize::from_str_radix(x, 10).unwrap())
            })
            .unwrap()
            .unwrap();
        self.pos += 1;
        Some(Monkey {
            operation: (
                usize::from_str_radix(operation.get(0).unwrap(), 10).ok(),
                operation.get(1).unwrap().chars().next().unwrap(),
                usize::from_str_radix(operation.get(2).unwrap(), 10).ok(),
            ),
            items,
            div,
            partners: (m1, m2),
            inspected: 0,
        })
    }
}

impl Monkey {
    fn next(&self, worry: &u128, d: u128) -> (u128, usize) {
        //println!("  Monkey inspects an item with a worry level of {worry}.");
        let p1 = self.operation.0.map_or(worry.clone(), |x| x as u128);
        let p2 = self.operation.2.map_or(worry.clone(), |x| x as u128);
        let mut worry = if self.operation.1 == '+' {
            //println!("    Worry level increases by {p2} to {}.", p1 + p2);
            p1 + p2
        } else {
            //println!("    Worry level multiplied by {p2} to {}.", p1 * p2);
            let r = p1 * p2;
            r % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 29)
        };
        if d != 1 {
            worry /= d;
        }
        (
            worry.clone(),
            if worry % self.div == 0 {
                self.partners.0
            } else {
                self.partners.1
            },
        )
    }
}

fn read_desc(input: &str) -> Vec<Monkey> {
    let mi = MonkeyInput::new(input);
    Vec::from_iter(mi)
}

fn solution_a(input: &str) -> usize {
    let mut monkeys = read_desc(input);
    let n = monkeys.len();

    for _round in 0..20 {
        //println!("Round {round}");
        let mut next = Vec::from_iter((0..n).map(|_| Vec::<u128>::new()));
        let mut prev = Vec::from_iter((0..n).map(|_| Vec::<u128>::new()));

        (0..n).for_each(|i| {
            //println!("Monkey {i}:");
            let m = monkeys.get(i).unwrap();

            let mut l = Vec::from_iter(m.items.iter().map(|x| x.clone()));
            l.extend_from_slice(next.get(i).unwrap());

            l.iter().for_each(|item| {
                let (worry, partner) = m.next(&item, 3);
                //println!("    Item with worry level {worry} is thrown to monkey {partner}.");
                if partner < i {
                    prev.get_mut(partner).map(|p| p.push(worry));
                } else {
                    next.get_mut(partner).map(|p| p.push(worry));
                }
            });
            monkeys.get_mut(i).map(|m| m.inspected += l.len());
        });

        (0..n - 1).for_each(|i| {
            monkeys
                .get_mut(i)
                .map(|m| m.items = Vec::from_iter(prev.get(i).unwrap().iter().map(|x| x.clone())));
        });
        monkeys.iter_mut().last().map(|m| m.items.clear());
    }

    let b: Option<(usize, usize)> = monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple();
    b.map_or(0, |b| b.0 * b.1)
}

fn solution_b(input: &str) -> usize {
    let mut monkeys = read_desc(input);
    let n = monkeys.len();

    let start = std::time::Instant::now();

    for round in 0..10000 {
        if round > 0 && round % 1000 == 0 {
            let d = start.elapsed().as_secs();
            println!("Round {round} in {d}s");
        }
        let mut next = Vec::from_iter((0..n).map(|_| Vec::<u128>::new()));
        let mut prev = Vec::from_iter((0..n).map(|_| Vec::<u128>::new()));

        (0..n).for_each(|i| {
            let m = monkeys.get(i).unwrap();

            let mut l = Vec::from_iter(m.items.iter().map(|x| x.clone()));
            l.extend_from_slice(next.get(i).unwrap());

            l.iter().for_each(|item| {
                let (worry, partner) = m.next(item, 1);
                if partner < i {
                    prev.get_mut(partner).map(|p| p.push(worry));
                } else {
                    next.get_mut(partner).map(|p| p.push(worry));
                }
            });
            monkeys.get_mut(i).map(|m| m.inspected += l.len());
        });

        (0..n - 1).for_each(|i| {
            monkeys
                .get_mut(i)
                .map(|m| m.items = Vec::from_iter(prev.get(i).unwrap().iter().map(|x| x.clone())));
        });
        monkeys.iter_mut().last().map(|m| m.items.clear());
    }

    (0..n).for_each(|i| {
        let m = monkeys.get(i).unwrap();
        println!("Monkey {i}: {}", m.inspected);
    });

    let d = start.elapsed().as_secs();
    println!("Total {d}s");

    let b: Option<(usize, usize)> = monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple();
    println!("{:?}", b);
    b.map_or(0, |b| b.0 * b.1)
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
        assert_eq!(solution_a(&data), 10605);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 2713310158);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 78678);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 15333249714);
    }
}
