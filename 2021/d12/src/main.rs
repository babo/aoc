use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn walk(
    node: &String,
    seen: &HashSet<String>,
    connections: &HashMap<String, Vec<String>>,
) -> usize {
    if seen.contains(node) {
        return 0;
    }
    if node.eq("end") {
        return 1;
    }
    let mut seen = seen.clone();
    if node.chars().next().map_or(false, |x| x.is_lowercase()) {
        seen.insert(node.clone());
    }
    connections
        .get(node)
        .unwrap()
        .iter()
        .map(|next| walk(next, &seen, connections))
        .sum()
}

fn walk2(
    node: &String,
    seen: &HashSet<String>,
    solution: &mut HashSet<String>,
    doubled: bool,
    path: &String,
    connections: &HashMap<String, Vec<String>>,
) {
    if seen.contains(node) {
        return;
    }
    let mut path = path.clone();
    path.push(',');
    path.push_str(node);

    if node.eq("end") {
        solution.insert(path);
        return;
    }

    let mut seen = seen.clone();
    if node.find(char::is_uppercase).is_none() {
        if doubled == false {
            connections
                .get(node)
                .unwrap()
                .iter()
                .for_each(|next| walk2(next, &seen, solution, true, &path, connections));
        }

        seen.insert(node.clone());
    }
    connections
        .get(node)
        .unwrap()
        .iter()
        .for_each(|next| walk2(next, &seen, solution, doubled, &path, connections));
}

fn get_connections(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let mut pair = line.trim().split("-");
            let a = pair.next().unwrap();
            let b = pair.next().unwrap();
            assert_eq!(pair.next(), None);
            (a, b)
        })
        .fold(HashMap::new(), |mut acc, x| {
            if acc.contains_key(x.0) {
                acc.get_mut(x.0).map(|v| v.push(x.1.to_string()));
            } else {
                acc.insert(x.0.to_string(), vec![x.1.to_string()]);
            }
            if x.0 != "start" {
                if acc.contains_key(x.1) {
                    acc.get_mut(x.1).map(|v| v.push(x.0.to_string()));
                } else {
                    acc.insert(x.1.to_string(), vec![x.0.to_string()]);
                }
            }
            acc
        })
}

fn solution_a(input: &str) -> Option<usize> {
    let connections = get_connections(input);

    let seen: HashSet<String> = HashSet::new();
    Some(walk(&"start".to_string(), &seen, &connections))
}

fn solution_b(input: &str) -> Option<usize> {
    let connections = get_connections(input);

    let start = "start".to_string();
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(start.clone());
    let mut solution: HashSet<String> = HashSet::new();
    connections
        .get(&start)
        .unwrap()
        .iter()
        .for_each(|next| walk2(next, &seen, &mut solution, false, &start, &connections));
    Some(solution.len())
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
    fn test_simple_a_10() {
        let data = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        assert_eq!(solution_a(&data), Some(10));
    }

    #[test]
    fn test_simple_a_19() {
        let data = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";
        assert_eq!(solution_a(&data), Some(19));
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(226));
    }

    #[test]
    fn test_simple_b_36() {
        let data = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        assert_eq!(solution_b(&data), Some(36));
    }

    #[test]
    fn test_simple_b_103() {
        let data = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";
        assert_eq!(solution_b(&data), Some(103));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(3509));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(3298));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(93572));
    }
}
