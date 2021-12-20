use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str, days: usize) -> Option<usize> {
    let mut school: Vec<u8> = input
        .split(",")
        .map(|v| u8::from_str_radix(v.trim(), 10).unwrap())
        .collect();
    for _ in 0..days {
        let babies = school.iter().filter(|x| **x == 0).count();
        for x in school.iter_mut() {
            if *x > 0 {
                *x = *x - 1;
            } else {
                *x = 6
            }
        }
        for _ in 0..babies {
            school.push(8u8);
        }
    }

    Some(school.len())
}

fn solution_b(input: &str, days: usize) -> Option<usize> {
    let mut ranks = [0usize; 7];
    input
        .split(",")
        .for_each(|v| {
            let age = usize::from_str_radix(v.trim(), 10).unwrap();
            ranks[age] += 1;
        });

    let steps = (days / 7) + 1;
    let mut total = 0usize;
    let mut birth: Vec<usize> = vec![0usize; days];

    for i in 0..7 {
        total += ranks[i];
        for j in 0..steps {
            let d = i + j * 7;
            if d >= days {
                break;
            }
            birth[d] += ranks[i];
        }
    }

    for day in 0..days {
        total += birth[day];
        println!("total: {} {} {}", day, birth[day], total);
        for j in 0..steps {
            let d = day + 9 + j * 7;
            if d >= days {
                break;
            }
            birth[d] += birth[day];
        }
    }
    Some(total)
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c, 80);
    let b = solution_b(&c, 256);

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_simple_a_18() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_a(&data, 18), Some(26));
    }

    #[test]
    fn test_simple_a_80() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_a(&data, 80), Some(5934));
    }

    #[test]
    fn test_simple_b_18() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_b(&data, 18), Some(26));
    }

    #[test]
    fn test_simple_b_80() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_b(&data, 80), Some(5934));
    }

    #[test]
    fn test_simple_b_256() {
        let data = read_to_string("./simple.txt").ok().unwrap();
        assert_eq!(solution_b(&data, 256), Some(26984457539));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c, 80), Some(376194));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c, 256), Some(1693022481538));
    }
}
