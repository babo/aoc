use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn read_inst_a(input: &str) -> (u64, Vec<u64>) {
    let mut lines = input.lines();
    let ts = lines.next().unwrap().parse::<u64>().unwrap();
    let nums = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    (ts, nums)
}

fn read_inst_b(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    lines.next();
    let mut rtv = Vec::new();
    lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().ok())
        .fold(0, |acc, x| {
            match x {
                Some(num) => rtv.push((num, acc)),
                None => (),
            }
            acc + 1
        });
    rtv
}

fn validate(input: &[(u64, u64)], t: u64) -> bool {
    for x in input {
        if (t + x.1) % x.0 != 0 {
            return false;
        }
    }
    true
}

fn solution_a(input: &str) -> u64 {
    let inst = read_inst_a(input);
    let t0 = inst.0;
    let res = inst.1.iter().fold((t0, 0, 0), |acc, x| {
        let wait_time = x - (t0 % x);
        if acc.0 > wait_time {
            (wait_time, *x, wait_time * x)
        } else {
            acc
        }
    });
    res.2
}

fn solution_b_hardcoded(input: &str) -> Option<u64> {
    let inst = read_inst_b(input);
    let step =  4248711961u64;
    let mut t = 3513036027u64;
    for i in 1.. {
        if validate(&inst, t) {
            println!("Found in {}", i);
            return Some(t);
        }
        t += step;
    }
    None
}

// First solve an easier problem with brute force. Remove the two largest
// numbers and use that solution as a baseline and the multiplication
// of all these numbers as a step. From Chinese remainder theorem we know
// that if we have a solution, than adding the multiplication of all the
// modulos is also a solution.
// When solving the hard problem we know that the solution must fulfill the
// easy solution as well. With that we can use the calculated base and step
// from the easy solution and find the hard problem with brute force, but in
// a reasonable amount of time, faster than 500ms.

fn solution_b(input: &str) -> Option<u64> {
    let mut inst = read_inst_b(input);
    inst.sort_by_key(|x| x.0);
    let m1 = inst[inst.len() - 1];
    let m2 = inst[inst.len() - 2];
    let m3 = inst[inst.len() - 3];
    let easy: Vec<(u64, u64)> = inst.iter().filter(|x| x.0 != m1.0 && x.0 != m2.0).map(|x| *x).collect();
    let step = easy.iter().fold(1u64, |acc, x| acc * x.0);

    let mut base: Option<u64> = None;
    for i in 1.. {
        let t = i * m3.0 - (m3.1 % m3.0);
        if validate(&easy, t) {
            println!("Found in {}", i);
            base = Some(t);
            break;
        }
    }
    let mut t = base.unwrap();
    for i in 1.. {
        if validate(&inst, t) {
            println!("Found in {}", i);
            return Some(t);
        }
        t += step;
    }
    None
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);
    let c = solution_b_hardcoded(&c);

    println!("Step A: {}", a);
    println!("Step B: {}", b.unwrap());
    println!("Step C: {}", c.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_instructions_a() {
        let c = content().unwrap();
        let inst = read_inst_a(&c);

        assert_eq!(inst.0, 1000391);
        assert_eq!(inst.1.len(), 9);
    }

    #[test]
    fn test_rules_a() {
        let c = "939
        7,13,x,x,59,x,31,19";
        let res = solution_a(c);

        assert_eq!(res, 295);
    }

    #[test]
    fn test_rules_b() {
        let c = "939
        7,13,x,x,59,x,31,19";
        let res = solution_b(c);

        assert_eq!(res, Some(1068781));
    }

    #[test]
    fn test_smaller_b() {
        let c = "939
        19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,r383,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,x457,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,17";
        let res = solution_b(c);

        assert_eq!(res, Some(3513036027));
    }

    #[test]
    fn test_validate_b() {
        let c = "939
        7,13,x,x,59,x,31,19";
        let inst = read_inst_b(c);

        assert_eq!(validate(&inst, 1068781), true);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let res = solution_a(&c);

        assert_eq!(res, 1915);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let res = solution_b(&c);

        assert_eq!(res, Some(294354277694107));
    }

    #[test]
    fn test_sorted() {
        let c = content().unwrap();
        let res = solution_b_hardcoded(&c);

        assert_eq!(res, Some(294354277694107));
    }
}
