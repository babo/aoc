use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn read_inst(input: &str) -> (i64, Vec<i64>) {
    let mut lines = input.lines();
    let ts = lines.next().unwrap().parse::<i64>().unwrap();
    let nums = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    (ts, nums)
}

fn solution_a(input: &str) -> i64 {
    let inst = read_inst(input);
    let t0 = inst.0;
    let res = inst.1.iter().fold((t0, 0, 0), |acc, x| {
        let wait_time = x - (t0 % x);
        if acc.0 > wait_time {
            (wait_time, *x, wait_time * x)
        } else {
            acc
        }
    });
    println!("res {:?}", res);
    for x in inst.1.iter() {
        println!("{} {} {}", (x - (inst.0 % x)) * x, x, x - (inst.0 % x));
    }
    res.2
}

fn solution_b(_input: &str) -> i32 {
    0
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {}", a);
    println!("Step B: {}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_instructions() {
        let c = content().unwrap();
        let inst = read_inst(&c);

        assert_eq!(inst.0, 1000391);
        assert_eq!(inst.1.len(), 9);
    }

    #[test]
    fn test_rules() {
        let c = "939
        7,13,x,x,59,x,31,19";
        let res = solution_a(c);

        assert_eq!(res, 295);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let res = solution_a(&c);

        assert_eq!(res, 1915);
    }
}
