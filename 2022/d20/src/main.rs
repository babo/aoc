use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str) -> i32 {
    let mut nums = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .enumerate()
        .map(|(i, n)| (i.max(1) - 1, i + 1, n))
        .collect_vec();
    let n = nums.len() - 1;
    nums.get_mut(0).map(|node| *node = (n, node.1, node.2));
    nums.get_mut(n).map(|node| *node = (node.0, 0, node.2));

    /*
    nums.iter()
        .for_each(|n| println!("{} <- {} -> {}", n.0, n.2, n.1));
    println!("----");
    */

    let mut zero = 0;
    for i in 0..=n {
        let (l, r, v) = nums[i];

        if v != 0 {
            let (ll, _, lv) = nums[l];
            nums[l] = (ll, r, lv);
            let (_, rr, rv) = nums[r];
            nums[r] = (l, rr, rv);

            let m = v.abs() % n as i32;
            if v > 0 {
                let np = (1..m).fold(r, |prev, _| nums[prev].1);
                let (ll, rr, lv) = nums[np];
                nums[i] = (np, rr, v);
                nums[np] = (ll, i, lv);
                let (_, r, rv) = nums[rr];
                nums[rr] = (i, r, rv);
            } else if v < 0 {
                let np = (1..m).fold(l, |prev, _| nums[prev].0);
                let (ll, rr, rv) = nums[np];
                nums[i] = (ll, np, v);
                nums[np] = (i, rr, rv);
                let (l, _, lv) = nums[ll];
                nums[ll] = (l, i, lv);
            }
        } else {
            zero = i;
        }
        /*
                println!(
                    "Moving {v} between {} and {}",
                    nums[nums[i].0].2, nums[nums[i].1].2
                );
                nums.iter().for_each(|n| println!("{} <- {} -> {}", n.0, n.2, n.1));
                println!("----");
        */
    }
    //nums.iter().for_each(|n| println!("{} <- {} -> {}", n.0, n.2, n.1));

    (0..3001)
        .fold((zero, 0), |prev, i| {
            let (_, r, v) = nums[prev.0];
            (
                r,
                if i != 0 && i % 1000 == 0 {
                    println!("{i} value: {v} {}", prev.1);
                    prev.1 + v
                } else {
                    prev.1
                },
            )
        })
        .1
}

fn solution_b(input: &str) -> i64 {
    let mut nums = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| i64::from_str_radix(x, 10).unwrap() * 811589153)
        .enumerate()
        .map(|(i, n)| (i.max(1) - 1, i + 1, n))
        .collect_vec();
    let n = nums.len() - 1;
    nums.get_mut(0).map(|node| *node = (n, node.1, node.2));
    nums.get_mut(n).map(|node| *node = (node.0, 0, node.2));

    let mut zero = 0;
    for _ in 0..10 {
        for i in 0..=n {
            let (l, r, v) = nums[i];
            if v != 0 {
                let (ll, _, lv) = nums[l];
                nums[l] = (ll, r, lv);
                let (_, rr, rv) = nums[r];
                nums[r] = (l, rr, rv);

                let m = v.abs() % n as i64;
                if v > 0 {
                    let np = (1..m).fold(r, |prev, _| nums[prev].1);
                    let (ll, rr, lv) = nums[np];
                    nums[i] = (np, rr, v);
                    nums[np] = (ll, i, lv);
                    let (_, r, rv) = nums[rr];
                    nums[rr] = (i, r, rv);
                } else if v < 0 {
                    let np = (1..m).fold(l, |prev, _| nums[prev].0);
                    let (ll, rr, rv) = nums[np];
                    nums[i] = (ll, np, v);
                    nums[np] = (i, rr, rv);
                    let (l, _, lv) = nums[ll];
                    nums[ll] = (l, i, lv);
                }
            } else {
                zero = i;
            }
        }
    }

    (0..3001)
        .fold((zero, 0), |prev, i| {
            let (_, r, v) = nums[prev.0];
            (
                r,
                if i != 0 && i % 1000 == 0 {
                    println!("{i} value: {v} {}", prev.1);
                    prev.1 + v
                } else {
                    prev.1
                },
            )
        })
        .1
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
        assert_eq!(solution_a(&data), 3);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 1623178306);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 7713);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 1664569352803);
    }
}
