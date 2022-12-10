use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .fold((0i64, 1i64, 1i64), |accum, instr| {
            let (mut ret, val, mut cycle) = accum;

            if cycle == 20 || ((cycle - 20) % 40 == 0 && cycle < 230) {
                ret += val * cycle;
            }
            if instr == "noop" {
                (ret, val, cycle + 1)
            } else {
                cycle += 1;
                if cycle == 20 || ((cycle - 20) % 40 == 0 && cycle < 230) {
                    ret += val * cycle;
                }
                let n = instr
                    .get(5..)
                    .map(|x| i64::from_str_radix(x, 10).unwrap())
                    .unwrap();
                (ret, val + n, cycle + 1)
            }
        })
        .0
}

fn solution_b(input: &str) -> String {
    let mut rtv = String::new();
    let mut draw = |cycle: i64, val: i64| {
        let cycle = (cycle-1) % 40;
        rtv.push(if cycle >= val-1 && cycle <= val+1 { '#' } else { '.' });
        if cycle == 39 {
            rtv.push('\n');
        }
    };

    input
        .lines()
        .filter(|line| !line.is_empty())
        .fold((1i64, 1i64), |accum, instr| {
            let (val, mut cycle) = accum;
            draw(cycle, val);
            if instr == "noop" {
                (val, cycle + 1)
            } else {
                cycle += 1;
                draw(cycle, val);
                let n = instr
                    .get(5..)
                    .map(|x| i64::from_str_radix(x, 10).unwrap())
                    .unwrap();
                (val + n, cycle + 1)
            }
        });

    println!("{rtv}");
    rtv
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
        assert_eq!(solution_a(&data), 13140);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        let out = read_to_string("./simple_out.txt").unwrap();
        assert_eq!(solution_b(&data), out);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 13720);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        let out = read_to_string("./out.txt").unwrap();
        assert_eq!(solution_b(&c), out);
    }
}
