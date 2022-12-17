use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EvalState {
    valves: u64,
    time: u8,
    position: usize,
    pressure: u32,
    choices: [u8; 56],
}

struct Volcano<'a> {
    names: Vec<&'a str>,
    rates: HashMap<usize, u32>,
    tunnel: HashMap<usize, Vec<usize>>,
    start: usize,
}

impl<'a> Volcano<'a> {
    const TIME_LIMIT: u8 = 30;

    // Valve YJ has flow rate=15; tunnels lead to valves OC, PE, AC
    fn new(input: &'a str) -> Self {
        let names: Vec<_> = input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|line| line.get(6..8).unwrap())
            .collect_vec();
        let lookup: HashMap<&str, usize> =
            HashMap::from_iter(names.iter().enumerate().map(|a| (*a.1, a.0)));

        let mut rates: HashMap<usize, u32> = HashMap::new();
        let mut tunnel: HashMap<usize, Vec<usize>> = HashMap::new();

        input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .for_each(|line| {
                let node = *lookup.get(line.get(6..8).unwrap()).unwrap();
                let semi = line.find(';').unwrap();
                let rate = u32::from_str_radix(line.get(23..semi).unwrap(), 10).unwrap();
                let vp = line.find("valve").unwrap() + 5;
                if line.get(vp..vp + 1) == Some("s") {
                    let next: Vec<_> = line
                        .get(vp + 2..)
                        .unwrap()
                        .split(", ")
                        .map(|name| *lookup.get(name).unwrap())
                        .collect();
                    tunnel.insert(node, next);
                } else {
                    let mut next = Vec::new();
                    next.push(*lookup.get(line.get(vp + 1..).unwrap()).unwrap());
                    tunnel.insert(node, next);
                }
                rates.insert(node, rate);
            });

        let start = *lookup.get("AA").unwrap();
        Volcano {
            names,
            rates,
            tunnel,
            start,
        }
    }

    fn initial_state(&self) -> EvalState {
        let choices: Vec<_> = (0..56)
            .map(|index| {
                if index < self.names.len() {
                    let mut c = 0;
                    for i in 0..self.tunnel.get(&index).unwrap().len() {
                        c |= 1 << i;
                    }
                    c
                } else {
                    0
                }
            })
            .collect();
        let choices: [u8; 56] = choices.try_into().unwrap();
        EvalState {
            valves: 0,
            time: 1,
            position: self.start,
            pressure: 0,
            choices,
        }
    }

    fn take_action(&self, state: &EvalState) -> Vec<EvalState> {
        let mut next = Vec::new();
        let current_pos = state.position;
        let current_time = state.time;
        let current_pressure = state.pressure;
        let free_choices = state.choices[current_pos];
        let name = self.names[state.position];

        if current_time <= Self::TIME_LIMIT && free_choices != 0 {
            let rate = *self.rates.get(&current_pos).unwrap();
            let valve_closed = (state.valves & (1 << current_pos)) == 0;
            let mut new_choice = state.choices.clone();

            for (next_choice, next_position) in
                self.tunnel.get(&current_pos).unwrap().iter().enumerate()
            {
                let mask = 1u8 << next_choice;
                if (free_choices & mask) != 0 {
                    let next_position = *next_position;
                    let turned_off = free_choices & !mask;
                    new_choice[current_pos] = turned_off;
                    if rate != 0 && valve_closed && Volcano::TIME_LIMIT >= current_time {
                        let valve_open = state.valves | (1 << current_pos);
                        let increase = rate * (Volcano::TIME_LIMIT as u32 - current_time as u32);
                        println!(
                            "Open {name} at {current_time} with {increase} from {current_pressure}"
                        );
                        next.push(EvalState {
                            valves: valve_open,
                            time: current_time + 2,
                            position: next_position,
                            pressure: current_pressure + increase,
                            choices: new_choice,
                        });
                    }
                    next.push(EvalState {
                        valves: state.valves,
                        time: current_time + 1,
                        position: next_position,
                        pressure: current_pressure,
                        choices: new_choice,
                    });
                }
            }
        }
        next
    }

    fn show(&self, state: &EvalState) {
        println!(
            "Position: {} at {} with {}",
            self.names[state.position as usize], state.time, state.pressure
        );
    }
}

fn solution_a(input: &str) -> usize {
    let v = Volcano::new(input);

    let mut current_states = Vec::new();
    let mut future_states = Vec::new();
    let mut best_so_far = 0u32;
    current_states.push(v.initial_state());

    while !current_states.is_empty() {
        future_states.clear();
        current_states.iter().for_each(|state| {
            //v.show(state);
            v.take_action(state).iter().for_each(|next_state| {
                if next_state.pressure > best_so_far {
                    best_so_far = next_state.pressure;
                }

                if next_state.time < 3 || next_state.pressure > 100 {
                    future_states.push(*next_state);
                }
            });
        });
        current_states.clear();
        current_states.extend(future_states.iter());
    }

    best_so_far as usize
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
        assert_eq!(solution_a(&data), 1651);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 99999);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 99999);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 99999);
    }
}
