//use itertools::Itertools;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input").ok()
}

fn to_rc(digit: char) -> (usize, usize) {
    match digit {
        'a' => (3, 2),
        'A' => (3, 2),
        '0' => (3, 1),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        _ => panic!("Invalid digit"),
    }
}

fn from_rc(xy: (usize, usize)) -> char {
    match xy {
        (3, 2) => 'A',
        (3, 1) => '0',
        (2, 0) => '1',
        (2, 1) => '2',
        (2, 2) => '3',
        (1, 0) => '4',
        (1, 1) => '5',
        (1, 2) => '6',
        (0, 0) => '7',
        (0, 1) => '8',
        (0, 2) => '9',
        _ => panic!("Invalid digit"),
    }
}

fn to_kp(symbol: char) -> (usize, usize) {
    match symbol {
        '^' => (0, 1),
        'a' => (0, 2),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!("Invalid symbol"),
    }
}

fn from_kp(xy: (usize, usize)) -> char {
    match xy {
        (0, 1) => '^',
        (0, 2) => 'A',
        (1, 0) => '<',
        (1, 1) => 'v',
        (1, 2) => '>',
        _ => panic!("Invalid symbol"),
    }
}

fn path_r_1(path: &str) -> String {
    path.chars()
        .fold((to_rc('A'), String::new()), |((x, y), out), c| match c {
            '^' => ((x - 1, y), out),
            'v' => ((x + 1, y), out),
            '<' => ((x, y - 1), out),
            '>' => ((x, y + 1), out),
            'A' => ((x, y), format!("{}{}", out, from_rc((x, y)))),
            _ => panic!("Invalid path"),
        })
        .1
}

fn path_r_2(path: &str) -> String {
    path.chars()
        .fold((to_kp('A'), String::new()), |((x, y), out), c| match c {
            '^' => ((x - 1, y), out),
            'v' => ((x + 1, y), out),
            '<' => ((x, y - 1), out),
            '>' => ((x, y + 1), out),
            'A' => ((x, y), format!("{}{}", out, from_kp((x, y)))),
            _ => panic!("Invalid path"),
        })
        .1
}

fn path_1(f: char, t: char) -> String {
    if f == t {
        return "".to_string();
    }
    let (fr, fc) = to_rc(f);
    let (tr, tc) = to_rc(t);

    let lr = if tc > fc {
        ">".repeat(tc - fc).to_string()
    } else {
        "<".repeat(fc - tc).to_string()
    };

    if fr == tr {
        lr
    } else if tr < fr {
        let up = "^".repeat(fr - tr).to_string();

        if f == 'A' && fc == tc + 2 {
            format!("{}{}", up, lr)
        } else {
            format!("{}{}", lr, up)
        }
    } else {
        if tr == fr + 1 {
            format!("v{}", lr)
        } else {
            let down = "v".repeat(tr - fr).to_string();
            format!("{}{}", lr, down)
        }
    }
}

fn path_2(f: char, t: char) -> String {
    if f == t {
        return "".to_string();
    }

    let (fr, fc) = to_kp(f);
    let (tr, tc) = to_kp(t);

    let lr = if tc == fc {
        "".to_string()
    } else if tc > fc {
        ">".repeat(tc - fc).to_string()
    } else {
        "<".repeat(fc - tc).to_string()
    };

    if fr == tr {
        lr
    } else if tr > fr {
        format!("v{}", lr)
    } else {
        format!("{}^", lr)
    }
}

fn l1(code: &str) -> String {
    code.chars()
        .fold(('A', "".to_string()), |(prev, path), c| {
            (c, format!("{}{}A", path, path_1(prev, c)))
        })
        .1
}

fn l2(code: &str) -> String {
    code.chars()
        .fold(('A', "".to_string()), |(prev, path), c| {
            (c, format!("{}{}A", path, path_2(prev, c)))
        })
        .1
}

fn solve_a(code: &str) -> usize {
    let la = l1(code);
    let lb = l2(la.as_str());
    let lc = l2(lb.as_str());

    let n = code
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    println!("{} {} * {} = {}", code, lc.len(), n, lc.len() * n);
    lc.len() * n
}

fn solve_b(code: &str) -> usize {
    let lc = (0..25).fold(l1(code), |acc, _| l2(acc.as_str()));

    let n = code
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    println!("{} {} * {} = {}", code, lc.len(), n, lc.len() * n);
    lc.len() * n
}

fn solution_a(input: &str) -> usize {
    input.lines().map(|x| solve_a(x.trim())).sum::<usize>()
}

fn solution_b(input: &str) -> usize {
    input.lines().map(|x| solve_b(x.trim())).sum::<usize>()
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
        assert_eq!(solution_a(&data), 126384);
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), 1);
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        let n = solution_a(&c);
        assert_eq!(n, 163086);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 4);
    }

    #[test]
    fn test_path_1() {
        for f in 0..=10 {
            for t in 0..=10 {
                let fc = std::char::from_digit(f, 16).unwrap();
                let tc = std::char::from_digit(t, 16).unwrap();
                let path = path_1(fc, tc);

                if f == t {
                    assert_eq!(path.len(), 0);
                }
                if f == 0 && t == 10 {
                    assert_eq!(path, ">");
                }
                if f == 10 && t == 0 {
                    assert_eq!(path, "<");
                }
                path.chars().fold(to_rc(fc), |(x, y), c| {
                    let xy = match c {
                        '^' => (x - 1, y),
                        'v' => (x + 1, y),
                        '<' => (x, y - 1),
                        '>' => (x, y + 1),
                        'A' => (x, y),
                        _ => panic!("Invalid path"),
                    };
                    assert_ne!(xy, (3, 0));
                    assert!(xy.0 <= 3);
                    assert!(xy.1 <= 2);
                    xy
                });
            }
        }
    }

    #[test]
    fn test_path_2() {
        for f in "<^>vA".chars() {
            for t in "<^>vA".chars() {
                let path = path_2(f, t);

                if f == t {
                    assert_eq!(path.len(), 0);
                }
                if f == '<' && t == '>' {
                    assert_eq!(path, ">>");
                }
                if f == '>' && t == '<' {
                    assert_eq!(path, "<<");
                }
                if f == '^' && t == 'v' {
                    assert_eq!(path, "v");
                }
                if f == 'v' && t == '^' {
                    assert_eq!(path, "^");
                }
                if f == '<' && t == 'A' {
                    assert_eq!(path, ">>^");
                }
                path.chars().fold(to_kp(f), |(x, y), c| {
                    let xy = match c {
                        '^' => (x - 1, y),
                        'v' => (x + 1, y),
                        '<' => (x, y - 1),
                        '>' => (x, y + 1),
                        'A' => (x, y),
                        _ => panic!("Invalid path"),
                    };
                    assert_ne!(xy, (0, 0));
                    assert!(xy.0 <= 1);
                    assert!(xy.1 <= 2);
                    xy
                });
            }
        }
    }

    #[test]
    fn test_path_3() {
        assert_eq!(l1("0"), "<A");
        assert_eq!(l1("029A"), "<A^A>^^AvvvA");
    }

    #[test]
    fn test_reverse_path_3() {
        assert_eq!(path_r_1(l1("029A").as_str()), "029A");
        assert_eq!(path_r_1(l1("286A").as_str()), "286A");
        assert_eq!(path_r_1(l1("964A").as_str()), "964A");
    }

    #[test]
    fn test_reverse_path_4() {
        assert_eq!(path_r_1(path_r_2("v<<A>>^A<A>AvA<^AA>A").as_str()), "029");
        assert_eq!(
            path_r_1(path_r_2("v<<A>>^A<A>AvA<^AA>A<vAAA>^A").as_str()),
            "029A"
        );
    }

    #[test]
    fn test_path_4() {
        assert_eq!(l2("<"), "v<<A");
        assert_eq!(l2("<A"), "v<<A>>^A");
        assert_eq!(l2(l1("02").as_str()), "v<<A>>^A<A>A");

        assert_eq!(
            path_r_1(path_r_2(l2(l1("029A").as_str()).as_str()).as_str()),
            "029A"
        );
    }

    #[test]
    fn test_path_5() {
        assert_eq!(
            path_r_1(
                path_r_2(path_r_2(l2(l2(l1("029A").as_str()).as_str()).as_str()).as_str()).as_str()
            ),
            "029A"
        );

        let f = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
        let step_1 = path_r_2(f);
        let step_2 = path_r_2(step_1.as_str());
        let step_3 = path_r_1(step_2.as_str());
        assert_eq!(step_3, "029A");
    }

    #[test]
    fn test_path_6() {
        let code = "029A";
        let la = l1(code);
        let lb = l2(la.as_str());
        let lc = l2(lb.as_str());

        assert_eq!(la, "<A^A^^>AvvvA");
        assert_eq!(lb, "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
        assert_eq!(
            lc,
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
        );
    }

    #[test]
    fn test_path_7() {
        let code = "379A";

        let la = l1(code);
        let lb = l2(la.as_str());
        let lc = l2(lb.as_str());

        let f = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        let g = "v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A";
        println!("f  {}", f);
        println!("rf {}", path_r_2(f));
        println!("g  {}", g);
        println!("rg {}", path_r_2(g));
        println!();

        let step_1 = path_r_2(f);
        let step_2 = path_r_2(step_1.as_str());
        let step_3 = path_r_1(step_2.as_str());
        assert_eq!(step_3, "379A");

        println!("f");
        println!("{}", f);
        println!("step 1\n{}", step_1);
        // println!("{}", lb);
        println!("step 2\n{}", step_2);
        // println!("{}", la);

        println!("\nlc");
        println!("{}", lc);
        println!("{}", lb);
        println!("{}", la);
        println!("{}", code);
        assert_eq!(lc.len(), 64);
    }

    #[test]
    fn test_path_8() {
        let code = "286A";

        let la = l1(code);
        let lb = l2(la.as_str());
        let lc = l2(lb.as_str());

        println!("{}", la);
        println!("{}", lb);
        println!("{}", lc);

        let f = "<^A^^Av>AvvA";
        let fb = l2(f);
        let fc = l2(fb.as_str());

        println!("f");
        println!("{}", f);
        println!("{}", fb);
        println!("{}", fc);

        let step_1 = path_r_2(fc.as_str());
        let step_2 = path_r_2(step_1.as_str());
        let step_3 = path_r_1(step_2.as_str());
        assert_eq!(step_3, code);

        assert_eq!(fc.len(), lc.len());
    }
}
