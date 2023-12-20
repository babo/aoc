use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Other(bool),
}

impl Module {
    fn new(name: &str) -> (Self, String) {
        match name.chars().next() {
            Some('b') => (Module::Broadcaster, name.to_string()),
            Some('%') => (Module::FlipFlop, name.chars().skip(1).join("")),
            Some('&') => (Module::Conjunction, name.chars().skip(1).join("")),
            Some(_) => (Module::Other(name == "rx"), name.to_string()),
            _ => unimplemented!("Not a valid module"),
        }
    }
}

type Network = HashMap<usize, (Module, Vec<usize>)>;
type NamedNetwork = HashMap<String, (Module, Vec<String>)>;

fn read_input(input: &str) -> Network {
    let names: HashMap<String, usize> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .fold((HashMap::new(), 2), |(mut map, mut accu), line| {
            let mut it = line.split(" -> ");
            let x = it.next().unwrap();
            if x == "broadcaster" {
                map.insert(x.to_string(), 1);
            } else {
                map.insert(x.chars().skip(1).join(""), accu);
                accu += 1;
            }
            it.next().unwrap().split(", ").for_each(|x| {
                if !map.contains_key(x) {
                    map.insert(x.to_string(), accu);
                    accu += 1;
                }
            });
            (map, accu)
        })
        .0;
    let network = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .fold(HashMap::new(), |mut map, line| {
            let mut parts = line.split(" -> ");
            let (tp, name) = Module::new(parts.next().unwrap());
            let sub = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|x| names[x])
                .collect_vec();
            map.insert(*names.get(&name).unwrap(), (tp, sub));
            map
        });
    println!("{:?}", names);
    names.iter().fold(network, |mut accu, x| {
        if !accu.contains_key(x.1) {
            accu.insert(*x.1, (Module::Other(x.0 == "rx"), Vec::new()));
        }
        accu
    })
}

fn read_named_input(input: &str) -> NamedNetwork {
    let names: HashSet<String> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .fold(HashSet::new(), |mut map, line| {
            let mut it = line.split(" -> ");
            let x = it.next().unwrap();
            if x == "broadcaster" {
                map.insert(x.to_string());
            } else {
                map.insert(x.chars().skip(1).join(""));
            }
            it.next().unwrap().split(", ").for_each(|x| {
                if !map.contains(x) {
                    map.insert(x.to_string());
                }
            });
            map
        });
    let network = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .fold(HashMap::new(), |mut map, line| {
            let mut parts = line.split(" -> ");
            let (tp, name) = Module::new(parts.next().unwrap());
            let sub = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|x| x.to_string())
                .collect_vec();
            map.insert(name.clone(), (tp, sub));
            map
        });
    names.iter().fold(network, |mut accu, x| {
        if !accu.contains_key(x) {
            accu.insert(x.clone(), (Module::Other(x == "rx"), Vec::new()));
        }
        accu
    })
}

fn solve_a(cycles: usize, input: &str) -> usize {
    let network = read_input(input);
    println!("{:?}", network);

    let mut flips: HashMap<usize, bool> = network
        .iter()
        .filter(|x| x.1 .0 == Module::FlipFlop)
        .fold(HashMap::new(), |mut map, x| {
            map.insert(*x.0, false);
            map
        });
    let conj: HashMap<usize, HashMap<usize, bool>> = network
        .iter()
        .filter(|x| x.1 .0 == Module::Conjunction)
        .fold(HashMap::new(), |mut map, x| {
            map.insert(*x.0, HashMap::new());
            map
        });
    let mut conj = network.iter().fold(conj, |c, x| {
        let source = x.0;
        x.1 .1.iter().fold(c, |mut c, name| {
            if let Some(n) = c.get_mut(name) {
                n.insert(*source, false);
            }
            c
        })
    });

    let mut lows = 0;
    let mut highs = 0;
    for _cycle in 0..cycles {
        let mut active = vec![(0usize, 1usize, false)];
        while !active.is_empty() {
            lows += active.iter().filter(|x| !x.2).count();
            highs += active.iter().filter(|x| x.2).count();
            active = active
                .iter()
                .fold(Vec::new(), |mut next_round, (source, dest, signal)| {
                    match network[dest].0 {
                        Module::Broadcaster => network[dest].1.iter().for_each(|connected| {
                            next_round.push((*dest, *connected, *signal));
                        }),
                        Module::FlipFlop => {
                            if !signal {
                                if let Some(state) = flips.get_mut(dest) {
                                    *state = !*state;
                                    network[dest].1.iter().for_each(|connected| {
                                        next_round.push((*dest, *connected, *state));
                                    })
                                }
                            }
                        }
                        Module::Conjunction => {
                            if let Some(parents) = conj.get_mut(dest) {
                                if let Some(state) = parents.get_mut(source) {
                                    *state = *signal;
                                }
                                let state = parents.iter().any(|x| !x.1);
                                network[dest].1.iter().for_each(|connected| {
                                    next_round.push((*dest, *connected, state));
                                })
                            }
                        }
                        Module::Other(_) => (),
                    }
                    next_round
                })
        }
    }
    println!("{lows} {highs}");
    lows * highs
}

fn solution_b(input: &str) -> Option<usize> {
    let network = read_input(input);

    let mut flips: HashMap<usize, bool> = network
        .iter()
        .filter(|x| x.1 .0 == Module::FlipFlop)
        .fold(HashMap::new(), |mut map, x| {
            map.insert(*x.0, false);
            map
        });
    let conj: HashMap<usize, HashMap<usize, bool>> = network
        .iter()
        .filter(|x| x.1 .0 == Module::Conjunction)
        .fold(HashMap::new(), |mut map, x| {
            map.insert(*x.0, HashMap::new());
            map
        });
    let mut conj = network.iter().fold(conj, |c, x| {
        let source = x.0;
        x.1 .1.iter().fold(c, |mut c, name| {
            if let Some(n) = c.get_mut(name) {
                n.insert(*source, false);
            }
            c
        })
    });

    let intr = vec![73,36,79,84];

    let mut cycle = 0usize;
    loop {
        let mut active = vec![(0usize, 1usize, false)];
        let mut count_l = 0usize;
        let mut count_h = 0usize;
        while !active.is_empty() {
            active = active
                .iter()
                .fold(Vec::new(), |mut next_round, (source, dest, signal)| {
                    match network[dest].0 {
                        Module::Broadcaster => network[dest].1.iter().for_each(|connected| {
                            next_round.push((*dest, *connected, *signal));
                        }),
                        Module::FlipFlop => {
                            if !signal {
                                if let Some(state) = flips.get_mut(dest) {
                                    *state = !*state;
                                    network[dest].1.iter().for_each(|connected| {
                                        next_round.push((*dest, *connected, *state));
                                    })
                                }
                            }
                        }
                        Module::Conjunction => {
                            if let Some(parents) = conj.get_mut(dest) {
                                if let Some(state) = parents.get_mut(source) {
                                    *state = *signal;
                                }
                                let state = parents.iter().any(|x| !x.1);

                                if !state && intr.contains(dest) {
                                    println!("{cycle} {dest}");
                                }

                                network[dest].1.iter().for_each(|connected| {
                                    next_round.push((*dest, *connected, state));
                                })
                            }
                        }
                        Module::Other(true) => {
                            if *signal {
                                count_h += 1;
                            } else {
                                count_l += 1;
                            }
                        }
                        Module::Other(false) => (),
                    }
                    next_round
                })
        }
        cycle += 1;
        if count_l == 1 {
            println!("{count_h}");
            return Some(cycle);
        }
    }
}

fn analyze(input: &str) -> Option<usize> {
    let network = read_named_input(input);
    let reverse: HashMap<String, Vec<String>> =
        network.iter().fold(HashMap::new(), |mut accu, x| {
            let name = x.0;
            let (_module, items) = x.1;
            items.iter().for_each(|val| {
                if let Some(a) = accu.get_mut(val) {
                    a.push(name.clone());
                } else {
                    accu.insert(val.clone(), vec![name.clone()]);
                }
            });
            accu
        });
    let mut expected = vec![("rx", "-", false)];
    let mut round = 0;
    while round < 1 && !expected.is_empty() {
        expected = expected
            .iter()
            .fold(Vec::new(), |mut accu, (sender, parent, signal)| {
                match network[*sender].0 {
                    Module::Broadcaster => {
                        println!("Broadcaster round: {round} calling: {parent} sending {signal}");
                        round += 1;
                    }
                    Module::Conjunction => {
                        println!(
                            "Conjunction: {} with state {} calling: {parent} sending {signal}",
                            sender,
                            if *signal { "any" } else { "all high" }
                        );
                        if *signal {
                            reverse[*sender]
                                .iter()
                                .for_each(|name| accu.push((name, sender, *signal)));
                        } else {
                            reverse[*sender]
                                .iter()
                                .for_each(|name| accu.push((name, sender, true)));
                        }
                    }
                    Module::FlipFlop => {
                        println!(
                            "FlipFlop: {} with state: {} calling: {parent} sending false",
                            sender, !signal
                        );
                        reverse[*sender]
                            .iter()
                            .for_each(|name| accu.push((name, sender, false)));
                    }
                    Module::Other(_) => {
                        println!("Other: {sender} called receiving {signal}");
                        reverse[*sender]
                            .iter()
                            .for_each(|name| accu.push((name, sender, *signal)));
                    }
                    _ => unimplemented!("Not possible"),
                }
                accu
            });
        println!("---");
    }
    println!("Normal flow");
    let mut flow = vec![("broadcaster", false)];
    round = 0;
    let mut i = 0;
    while i < 2 && round < 2 && !flow.is_empty() {
        i += 1;
        flow = flow.iter().fold(Vec::new(), |mut accu, (sender, signal)| {
            match network[*sender].0 {
                Module::Broadcaster => {
                    println!("Broadcaster round: {round}");
                    network[*sender]
                        .1
                        .iter()
                        .for_each(|name| accu.push((name, *signal)));
                    round += 1;
                }
                Module::Conjunction => {
                    println!(
                        "Conjunction: {} with state {} receiving {}",
                        sender,
                        if *signal { "any" } else { "all high" },
                        signal
                    );
                    if *signal {
                        network[*sender]
                            .1
                            .iter()
                            .for_each(|name| accu.push((name, *signal)));
                    } else {
                        network[*sender]
                            .1
                            .iter()
                            .for_each(|name| accu.push((name, true)));
                    }
                }
                Module::FlipFlop => {
                    println!(
                        "FlipFlop: {} with state: {} receiving false",
                        sender, !signal
                    );
                    network[*sender]
                        .1
                        .iter()
                        .for_each(|name| accu.push((name, false)));
                }
                Module::Other(_) => {
                    println!("Other: {} {}", sender, signal);
                    network[*sender]
                        .1
                        .iter()
                        .for_each(|name| accu.push((name, *signal)));
                }
                _ => unimplemented!("Not possible"),
            };
            accu
        });
    }
    None
}

fn solution_a(input: &str) -> Option<usize> {
    Some(solve_a(1000, input))
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
        let data = "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a";
        assert_eq!(solution_a(&data), Some(32000000));
    }

    #[test]
    fn test_complex_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(11687500));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(929810733));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(231657829136023));
    }
    #[test]
    fn test_analyze() {
        let c = content().unwrap();

        assert_eq!(analyze(&c), Some(1));
    }

    #[test]
    fn test_mini() {
        let c = "broadcaster -> rev
            %rev -> bt, gp, ml, rb
            &lg -> rx
            &vg -> lg
            &ls -> lg
            &nb -> lg
            &vc -> lg
            &bt -> vg
            &gp -> ls
            &ml -> nb
            &rb -> vc";
        assert_eq!(solution_b(&c), Some(1));
    }
}
