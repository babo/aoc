use regex::Regex;
use std::fs::read_to_string;
use std::io::{self};
use std::collections::HashSet;
use std::iter::FromIterator;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_file_reading() -> Result<(), io::Error> {
        assert_ne!(content()?.len(), 0);
        Ok(())
    }

    #[test]
    fn test_processing() {
        let pairs = get_pairs();
        assert_ne!(pairs.len(), 0);
    }

    #[test]
    fn test_selection() {
        let pairs = get_pairs();
        let contain = select_bags("shiny gold", &pairs);

        assert_eq!(contain.len(), 0);
    }
}

fn main() {
    if let Ok(all_lines) = content() {
        let mut count = 0usize;
        for line in all_lines.lines() {
            if line.len() == 0 {
                count += 1;
            }
        }
        println!("Count: {}", count);
    }
}

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn get_pairs() -> Vec<(String, String)> {
    lazy_static! {
        static ref RE_SINGLE: Regex =
            Regex::new(r"^(\w+ \w+) bags contain no other bags.$").unwrap();
        static ref RE_MANY: Regex =
            Regex::new(r"^(\w+ \w+) bags contain (\d+ \w+ \w+ bags?, )*?(\d+) (\w+ \w+) bags?\.$")
                .unwrap();
        static ref RE_SUB: Regex = Regex::new(r" (\d+) (\w+ \w+) bags?,").unwrap();
    }

    let c = content().unwrap();
    let mut contain = Vec::new();
    for line in c.lines() {
        if RE_MANY.is_match(line) {
            let cap = RE_MANY.captures(line).unwrap();
            let left = cap.get(1).unwrap().as_str();
            let right = cap.get(4).unwrap().as_str();

            contain.push((String::from(left), String::from(right)));

            for cap in RE_SUB.captures_iter(line) {
                let right = cap.get(2).unwrap().as_str();
                contain.push((String::from(left), String::from(right)));
            }
        } else {
            // Not used for now
            let cap = RE_SINGLE.captures(line).unwrap();
            cap.get(1).unwrap().as_str();
        }
    }
    contain
}

fn select_bags(color: &str, pairs: &Vec<(String, String)>) -> Vec<String> {
    let direct: Vec<String> = pairs.iter().filter(|x| x.1 == color).map(|x| String::from(&x.0)).collect();
    let children = direct.iter().map(|x| select_bags(&x, pairs)).flatten();

    let mut result: HashSet<String> = HashSet::from_iter(children);
    for x in direct {
        result.insert(String::from(&x));
    }

    Vec::from_iter(result.into_iter())
}
