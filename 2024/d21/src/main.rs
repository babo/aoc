use std::cmp::Ordering;
use std::collections::HashMap;
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

fn to_kp(symbol: char) -> (usize, usize) {
    match symbol {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!("Invalid symbol"),
    }
}

fn path_r_2(f: char, t: char) -> Vec<String> {
    let mut rtv = Vec::new();

    if f == t {
        return rtv;
    }
    let (fr, fc) = to_kp(f);
    let (tr, tc) = to_kp(t);

    let lr = if tc > fc {
        ">".repeat(tc - fc).to_string()
    } else {
        "<".repeat(fc - tc).to_string()
    };

    if tr == fr {
        rtv.push(lr);
    } else if tr < fr {
        let up = "^".repeat(fr - tr).to_string();

        rtv.push(format!("{}{}", lr, up));
        if !(fr == 1 && fc == 0) && !lr.is_empty() {
            rtv.push(format!("{}{}", up, lr));
        }
    } else {
        let down = "v".repeat(tr - fr).to_string();
        rtv.push(format!("{}{}", down, lr));
        if !(tr == 1 && tc == 0) && !lr.is_empty() {
            rtv.push(format!("{}{}", lr, down));
        }
    }

    rtv
}

fn path_n_2(f: char, t: char) -> Vec<String> {
    let mut rtv = Vec::new();

    if f == t {
        return rtv;
    }
    let (fr, fc) = to_rc(f);
    let (tr, tc) = to_rc(t);

    let lr = if tc > fc {
        ">".repeat(tc - fc).to_string()
    } else {
        "<".repeat(fc - tc).to_string()
    };

    if tr == fr {
        rtv.push(lr);
    } else if tr < fr {
        let up = "^".repeat(fr - tr).to_string();

        rtv.push(format!("{}{}", up, lr));
        if !(fr == 3 && tc == 0) && !lr.is_empty() {
            rtv.push(format!("{}{}", lr, up));
        }
    } else {
        let down = "v".repeat(tr - fr).to_string();
        rtv.push(format!("{}{}", lr, down));
        if fc != 0 && !lr.is_empty() {
            rtv.push(format!("{}{}", down, lr));
        }
    }

    rtv
}

fn path_n_1(f: char, t: char) -> String {
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

        if (f == 'A' && fc == tc + 2) || (f == '0') {
            format!("{}{}", up, lr)
        } else {
            format!("{}{}", lr, up)
        }
    } else if tr == fr + 1 && tr != 3 {
        format!("v{}", lr)
    } else {
        let down = "v".repeat(tr - fr).to_string();
        format!("{}{}", lr, down)
    }
}

fn path_2(f: char, t: char) -> String {
    if f == t || f.is_ascii_whitespace() || t.is_ascii_whitespace() {
        return "".to_string();
    }

    let (fr, fc) = to_kp(f);
    let (tr, tc) = to_kp(t);

    let lr = match tc.cmp(&fc) {
        Ordering::Equal => "".to_string(),
        Ordering::Greater => ">".repeat(tc - fc).to_string(),
        Ordering::Less => "<".repeat(fc - tc).to_string(),
    };

    if fr == tr {
        lr
    } else if tr > fr {
        format!("v{}", lr)
    } else {
        format!("{}^", lr)
    }
}

fn path_3(f: char, t: char) -> String {
    if f == t {
        return String::new();
    }
    match (f, t) {
        ('^', 'v') => "v",
        ('v', '^') => "^",

        ('<', 'A') => ">>^", // 4107605444

        ('<', '^') => ">^", // 1651216277

        ('<', 'v') => ">", // 928140922

        ('>', 'A') => "^", // 2383958546

        ('>', '^') => "<^", // 4549672927

        ('>', 'v') => "<", // 9009012838

        ('A', '<') => "v<<", // 4901407394

        ('A', '>') => "v", // 8357534516

        ('A', '^') => "<", // 3621532006

        ('A', 'v') => "<v", // 4463726859

        ('^', '<') => "v<", // 1970315727

        ('^', '>') => "v>", // 3680843457

        ('^', 'A') => ">", // 2308871594

        ('v', '<') => "<", // 1455817624

        ('v', '>') => ">", // 928140922

        ('v', 'A') => "^>", // 9686334009
        _ => {
            println!("{} -> {}", f, t);
            panic!("Invalid path");
        }
    }
    .to_string()
}

fn path_4(f: char, t: char) -> String {
    if f == t {
        return String::new();
    }
    match (f, t) {
        ('A', '0') => "<",
        ('A', '1') => "^<<",
        ('A', '2') => "<^",
        ('A', '3') => "^",
        ('A', '4') => "^^<<",
        ('A', '5') => "<^^",
        ('A', '6') => "^^",
        ('A', '7') => "^^^<<",
        ('A', '8') => "<^^^",
        ('A', '9') => "^^^",
        ('0', 'A') => ">",
        ('0', '1') => "^<",
        ('0', '2') => "^",
        ('0', '3') => "^>",
        ('0', '4') => "^^<",
        ('0', '5') => "^^",
        ('0', '6') => "^^>",
        ('0', '7') => "^^^<",
        ('0', '8') => "^^^",
        ('0', '9') => "^^^>",
        ('1', 'A') => ">>v",
        ('1', '0') => ">v",
        ('1', '2') => ">",
        ('1', '3') => ">>",
        ('1', '4') => "^",
        ('1', '5') => "^>",
        ('1', '6') => "^>>",
        ('1', '7') => "^^",
        ('1', '8') => "^^>",
        ('1', '9') => "^^>>",
        ('2', 'A') => "v>",
        ('2', '0') => "v",
        ('2', '1') => "<",
        ('2', '3') => ">",
        ('2', '4') => "<^",
        ('2', '5') => "^",
        ('2', '6') => "^>",
        ('2', '7') => "<^^",
        ('2', '8') => "^^",
        ('2', '9') => "^^>",
        ('3', 'A') => "v",
        ('3', '0') => "<v",
        ('3', '1') => "<<",
        ('3', '2') => "<",
        ('3', '4') => "<<^",
        ('3', '5') => "<^",
        ('3', '6') => "^",
        ('3', '7') => "<<^^",
        ('3', '8') => "<^^",
        ('3', '9') => "^^",
        ('4', 'A') => ">>vv",
        ('4', '0') => ">vv",
        ('4', '1') => "v",
        ('4', '2') => ">v",
        ('4', '3') => ">>v",
        ('4', '5') => ">",
        ('4', '6') => ">>",
        ('4', '7') => "^",
        ('4', '8') => "^>",
        ('4', '9') => "^>>",
        ('5', 'A') => "vv>",
        ('5', '0') => "vv",
        ('5', '1') => "<v",
        ('5', '2') => "v",
        ('5', '3') => "v>",
        ('5', '4') => "<",
        ('5', '6') => ">",
        ('5', '7') => "<^",
        ('5', '8') => "^",
        ('5', '9') => "^>",
        ('6', 'A') => "vv",
        ('6', '0') => "<vv",
        ('6', '1') => "<<v",
        ('6', '2') => "<v",
        ('6', '3') => "v",
        ('6', '4') => "<<",
        ('6', '5') => "<",
        ('6', '7') => "<<^",
        ('6', '8') => "<^",
        ('6', '9') => "^",
        ('7', 'A') => ">>vvv",
        ('7', '0') => ">vvv",
        ('7', '1') => "vv",
        ('7', '2') => ">vv",
        ('7', '3') => ">>vv",
        ('7', '4') => "v",
        ('7', '5') => ">v",
        ('7', '6') => ">>v",
        ('7', '8') => ">",
        ('7', '9') => ">>",
        ('8', 'A') => "vvv>",
        ('8', '0') => "vvv",
        ('8', '1') => "<vv",
        ('8', '2') => "vv",
        ('8', '3') => "vv>",
        ('8', '4') => "<v",
        ('8', '5') => "v",
        ('8', '6') => "v>",
        ('8', '7') => "<",
        ('8', '9') => ">",
        ('9', 'A') => "vvv",
        ('9', '0') => "<vvv",
        ('9', '1') => "<<vv",
        ('9', '2') => "<vv",
        ('9', '3') => "vv",
        ('9', '4') => "<<v",
        ('9', '5') => "<v",
        ('9', '6') => "v",
        ('9', '7') => "<<",
        ('9', '8') => "<",
        _ => panic!("Invalid path"),
    }
    .to_string()
}
fn l1(code: &str) -> String {
    code.chars()
        .fold(('A', "".to_string()), |(prev, path), c| {
            (c, format!("{}{}A ", path, path_4(prev, c)))
        })
        .1
}

fn num_code(code: &str) -> usize {
    code.chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn l2(code: &str, tagged: bool) -> String {
    let tag = if tagged { " " } else { "" };
    code.chars()
        .fold(('A', "".to_string()), |(prev, path), c| {
            if c.is_ascii_whitespace() {
                (prev, path + " ")
            } else {
                (c, format!("{}{}A{}", path, path_3(prev, c), tag))
            }
        })
        .1
}

fn solve_a(code: &str) -> usize {
    let la = l1(code);
    let lb = l2(la.as_str(), true);
    let lc = l2(lb.as_str(), true);

    let length = lc.replace(" ", "").len();
    let num = num_code(code);

    // println!("{}", la);
    // println!("{}", lb);
    // println!("{}", lc);
    println!("{} {} * {} = {}", code, length, num, length * num);
    length * num
}

fn solve_b(code: &str) -> usize {
    let length = probe(l1(code).replace(" ", "").as_str(), 25);
    let num = num_code(code);

    println!("{} {} * {} = {}", code, length, num, length * num);
    length * num
}

fn probe2(code: &str, max_level: usize) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();

    fn run_level(
        prev: char,
        curr: char,
        level: usize,
        max_level: usize,
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        path_r_2(prev, curr)
            .iter()
            .map(|path| {
                let path = path.clone() + "A";
                if level < max_level {
                    let key = format!("{:02}| ('{}', '{}') | {}", level, prev, curr, path);
                    if let Some(v) = cache.get(&key) {
                        *v
                    } else {
                        let v = path
                            .chars()
                            .fold(('A', 0), |(prev, acc), curr| {
                                (
                                    curr,
                                    acc + run_level(prev, curr, level + 1, max_level, cache),
                                )
                            })
                            .1;
                        cache.insert(key, v);
                        v
                    }
                } else {
                    path.len()
                }
            })
            .min()
            .unwrap_or(1)
    }

    let rtv = code
        .chars()
        .fold(('A', 0), |(prev, acc), curr| {
            (curr, acc + run_level(prev, curr, 1, max_level, &mut cache))
        })
        .1;
    if cache.len() > 100 {
        std::fs::write(
            "cache.txt",
            cache
                .iter()
                .map(|(k, v)| format!("{} | {}\n", k, v))
                .collect::<String>(),
        )
        .unwrap();
    }
    rtv
}

fn probe(code: &str, max_level: usize) -> usize {
    const LEVELS: usize = 25;
    assert!(max_level > 1);
    assert!(max_level <= LEVELS);

    let mut pos = [0usize; LEVELS];
    let mut prev_c = ['A'; LEVELS];
    let mut stack = [const { String::new() }; LEVELS];
    let mut prev_l = [0usize; LEVELS];
    stack[0] = code.to_string();
    pos[0] = 0;

    let mut cache = HashMap::new();

    let mut length = 0;
    let mut level = 0;
    while pos[0] < stack[0].len() || level != 0 {
        let key = format!("{}{}A", level, stack[level]);

        if level + 1 >= max_level {
            if let Some(v) = cache.get(&key) {
                length += *v;
                stack[level] = String::new();
                level -= 1;
            } else {
                prev_l[level] = length;
                for c in stack[level].chars() {
                    let tr = path_3(prev_c[level], c) + "A";
                    length += tr.len();
                    prev_c[level] = c;
                }
                cache.insert(key, length - prev_l[level]);

                pos[level] = 0;
                stack[level] = String::new();
                level -= 1;
            }
        } else if pos[level] < stack[level].len() {
            if pos[level] == 0 {
                if let Some(v) = cache.get(&key) {
                    length += *v;
                    pos[level] = stack[level].len();
                    continue;
                }
                prev_l[level] = length;
            }
            let c = stack[level].chars().nth(pos[level]).unwrap();
            let tr = path_3(prev_c[level], c) + "A";

            prev_c[level] = c;
            pos[level] += 1;
            level += 1;

            stack[level] += tr.as_str();
        } else {
            cache.entry(key).or_insert(length - prev_l[level]);

            pos[level] = 0;
            stack[level] = String::new();
            level -= 1;
        }
    }

    length
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
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), 163086);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), 198466286401228);
    }

    #[test]
    fn test_numbers_1() {
        let c = content().unwrap();

        let mut path = HashMap::new();
        let res = c
            .lines()
            .map(|line| line.trim())
            .map(|code| {
                let num = num_code(code);
                let length = code
                    .chars()
                    .fold(('A', 0), |(prev, total), curr| {
                        let v = path_n_2(prev, curr)
                            .iter()
                            .map(|p| {
                                let press = p.clone() + "A";
                                let v1 = probe2(press.as_str(), 25);
                                let v2 = probe2(press.as_str(), 2);
                                println!("{} -> {}: {} {} {}", prev, curr, v1, v2, press);
                                let key = format!("('{}','{}')|{}", prev, curr, press);
                                path.insert(key, v1);
                                v1
                            })
                            .min()
                            .unwrap();
                        (curr, total + v)
                    })
                    .1;

                println!("{} {} * {} = {}", code, length, num, length * num);
                println!("l1: {}", l1(code));
                length * num
            })
            .sum::<usize>();

        std::fs::write(
            "path.txt",
            path.iter()
                .map(|(k, v)| format!("{} | {}\n", k, v))
                .collect::<String>(),
        )
        .unwrap();

        println!("Result: {}", res);
        assert_eq!(res, 198466286401228);
    }

    #[test]
    fn test_numbers_2() {
        let symb = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        let mut count = 0;
        for f in symb.iter() {
            for t in symb.iter() {
                if f == t {
                    continue;
                }
                let (k, _) = path_n_2(*f, *t)
                    .iter()
                    .map(|p| {
                        let press = p.clone() + "A";
                        (p.clone(), probe2(press.as_str(), 25))
                    })
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .unwrap();
                //println!("{} -> {}: {}", f, t, k);
                println!("('{}', '{}') => {:?},", f, t, k);

                if path_n_1(*f, *t) != *k {
                    //println!("('{}', '{}') => {:?}", f, t, k);
                    count += 1;
                }
            }
        }
        assert_eq!(count, 0);
    }

    #[test]
    fn test_arrows_1() {
        let symb = ['^', 'A', '<', 'v', '>'];

        for f in symb.iter() {
            for t in symb.iter() {
                if f == t {
                    continue;
                }
                let choices = path_r_2(*f, *t);
                let path = path_2(*f, *t);
                if choices.len() > 1 {
                    println!("{} .. {} := {} {:?}", f, t, path, choices);
                }

                assert!(choices.contains(&path));
            }
        }
    }

    #[test]
    fn test_arrows_2() {
        let symb = ['^', 'A', '<', 'v', '>'];

        for f in symb.iter() {
            for t in symb.iter() {
                if f == t {
                    continue;
                }
                assert_eq!(path_2(*f, *t), path_3(*f, *t));
            }
        }
    }
}
