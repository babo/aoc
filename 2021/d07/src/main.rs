use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> Option<usize> {
    let mut pos: Vec<usize> = input
        .split(",")
        .map(|x| usize::from_str_radix(x.trim(), 10).unwrap())
        .collect();
    pos.sort();

    let median = pos[pos.len() / 2];
    let fuel = pos.iter().fold(0, |acc, x| {
        acc + if *x < median { median - x } else { x - median }
    });

    Some(fuel)
}

fn solution_b(input: &str) -> Option<usize> {
    let mut pos: Vec<usize> = input
        .split(",")
        .map(|x| usize::from_str_radix(x.trim(), 10).unwrap())
        .collect();
    pos.sort();

    let min = pos.first().unwrap();
    let max = pos.last().unwrap();
    let range = max - min;
    let mut precalc = vec![0usize; range + 1];
    precalc.iter_mut().enumerate().fold(0usize, |acc, x| {
        *x.1 = acc + x.0;
        *x.1
    });
    let calc = |p: &usize| {
        pos.iter().fold(0usize, |acc, x| {
            acc + precalc[if x >= p { x - p } else { p - x }]
        })
    };

    let mean = (min + max) / 2;
    let median = pos[pos.len() / 2];
    let mm = if mean > median {
        median..mean
    } else {
        mean..median
    };

    let mut fuel: Option<usize> = None;
    for x in mm {
        let v = calc(&x);
        if fuel.map_or(true, |p| v < p) {
            fuel = Some(v);
        }
    }

    fuel
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
        assert_eq!(solution_a(&data), Some(37));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(168));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(340987));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(96987874));
    }
}
