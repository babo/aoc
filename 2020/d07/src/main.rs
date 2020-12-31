use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self};
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
    fn test_processing_step_one() {
        let pairs = get_pairs();
        assert_ne!(pairs.len(), 0);
    }

    #[test]
    fn test_processing_step_two() {
        let pairs = vec![
            (String::from("green"), String::from("blue"), 3),
            (String::from("green"), String::from("red"), 2),
            (String::from("red"), String::from("blue"), 5),
            (String::from("blue"), String::from(""), 0),
        ];
        assert_eq!(bags_in("red", 1, &pairs), 6);
        assert_eq!(bags_in("red", 3, &pairs), 18);
        assert_eq!(bags_in("green", 1, &pairs), 16);
        assert_eq!(bags_in("green", 5, &pairs), 80);
    }

    #[test]
    fn test_part_a() {
        let pairs = get_pairs();
        let contain = bags_out("shiny gold", &pairs);

        assert_eq!(contain.len(), 287);
    }

    #[test]
    fn test_empty_bags() {
        let pairs = get_pairs();

        assert_eq!(bags_in("shiny gold", 0, &pairs), 0);
        assert_eq!(bags_in("plaid teal", 1, &pairs), 1);
        assert_eq!(bags_in("drab magenta", 99, &pairs), 99);
    }

    #[test]
    fn test_part_b() {
        let pairs = get_pairs();

        // For the final, you don'n need to count the shiny gold;
        assert_eq!(bags_in("shiny gold", 1, &pairs) - 1, 48160);
    }
}

fn main() {
    let pairs = get_pairs();
    let contain = bags_out("shiny gold", &pairs);
    let bags = bags_in("shiny gold", 1, &pairs) - 1;

    println!("Part A: {}", contain.len());
    println!("Part B: {}", bags);
}

fn content() -> Result<String, io::Error> {
    Ok(read_to_string("./input.txt")?)
}

fn get_pairs() -> Vec<(String, String, usize)> {
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
            let n = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let right = cap.get(4).unwrap().as_str();

            contain.push((String::from(left), String::from(right), n));

            for cap in RE_SUB.captures_iter(line) {
                let right = cap.get(2).unwrap().as_str();
                let n = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                contain.push((String::from(left), String::from(right), n));
            }
        } else {
            // Not used for now
            let cap = RE_SINGLE.captures(line).unwrap();
            let left = cap.get(1).unwrap().as_str();
            contain.push((String::from(left), String::from(""), 0));
        }
    }
    contain
}

fn bags_out(color: &str, pairs: &Vec<(String, String, usize)>) -> Vec<String> {
    let direct: Vec<String> = pairs
        .iter()
        .filter(|x| x.1 == color)
        .map(|x| String::from(&x.0))
        .collect();
    let children = direct.iter().map(|x| bags_out(&x, pairs)).flatten();

    let mut result: HashSet<String> = HashSet::from_iter(children);
    for x in direct {
        result.insert(String::from(&x));
    }

    Vec::from_iter(result.into_iter())
}

fn bags_in(color: &str, count: usize, pairs: &Vec<(String, String, usize)>) -> usize {
    if count == 0 {
        return 0usize;
    }
    println!("{} {}", count, color);
    let direct = pairs.iter().filter(|x| x.0 == color);
    direct.fold(count, |acc, x| acc + bags_in(&x.1, count * x.2, pairs))
}
