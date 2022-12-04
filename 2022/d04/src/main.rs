use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn overlap(line: &str) -> usize {
    let nv:Vec<usize> = line.split(|c| c == '-' || c == ',').map(|x| usize::from_str_radix(x, 10).unwrap()).collect();
    let n: [usize; 4] = nv.try_into().unwrap();
    if n[0] >= n[2] && n[1] <= n[3] {
        return 1;
    }
    if n[2] >= n[0] && n[3] <= n[1] {
        1
    } else {
        0
    }
}

fn overlap_or_touch(line: &str) -> usize {
    let nv:Vec<usize> = line.split(|c| c == '-' || c == ',').map(|x| usize::from_str_radix(x, 10).unwrap()).collect();
    let n: [usize; 4] = nv.try_into().unwrap();
    if n[1] >= n[2] && n[1] <= n[3] {
        return 1;
    }
    if n[0] >= n[2] && n[0] <= n[3] {
        return 1
    }
    if n[2] >= n[0] && n[2] <= n[1] {
        return 1;
    }
    if n[3] >= n[0] && n[3] <= n[1] {
        return 1
    }
    0
}

fn solution_a(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| overlap(x.trim())).sum::<usize>())
}

fn solution_b(input: &str) -> Option<usize> {
    Some(input.lines().map(|x| overlap_or_touch(x.trim())).sum::<usize>())
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
        assert_eq!(solution_a(&data), Some(2));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(4));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(573));
    }

    #[test]
     fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(867));
    }
}
