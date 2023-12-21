use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

fn solution_a(input: &str, steps: usize) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let w = input.lines().next().map(|x| x.len()).unwrap();
    let input = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect_vec();
    let h = input.len() / w;
    let at = |(x, y): &(usize, usize)| (x + y * w);
    let s = input.iter().position(|x| *x == 'S').unwrap();
    let s = (s % w, s / w);

    let hs: HashSet<(usize, usize)> = (0..steps).fold(
        HashSet::from_iter(std::iter::repeat(s).take(1)),
        |current, _step| {
            HashSet::from_iter(
                current
                    .iter()
                    .map(|(x, y)| {
                        vec![
                            (*x as i64 - 1, *y as i64),
                            (*x as i64 + 1, *y as i64),
                            (*x as i64, *y as i64 + 1),
                            (*x as i64, *y as i64 - 1),
                        ]
                    })
                    .flatten()
                    .filter(|xy| xy.0 >= 0 && xy.0 < w as i64 && xy.1 >= 0 && xy.1 < h as i64)
                    .map(|xy| (xy.0 as usize, xy.1 as usize))
                    .filter(|xy| input[at(xy)] != '#'),
            )
        },
    );
    Some(hs.len())
}

fn solution_b(input: &str, steps: usize) -> Option<usize> {
    let input = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .join("\n");
    let w = input.lines().next().map(|x| x.len()).unwrap();
    let input = input
        .chars()
        .filter(|x| !x.is_ascii_whitespace())
        .collect_vec();
    let h = input.len() / w;
    let s = input.iter().position(|x| *x == 'S').unwrap();
    let s = ((s % w) as i64, (s / w) as i64);

    let at = |(x, y): &(i64, i64)| {
        ((w as i64 + x % w as i64).abs() as usize) % w
            + (((h as i64 + y % h as i64).abs() as usize) % h) * w
    };

    println!("w {w} h {h}");

    let hs: HashSet<(i64, i64)> = (0..steps).fold(
        HashSet::from_iter(std::iter::repeat(s).take(1)),
        |current, step| {
            let hs = HashSet::from_iter(
                current
                    .iter()
                    .map(|(x, y)| vec![(*x - 1, *y), (*x + 1, *y), (*x, *y + 1), (*x, *y - 1)])
                    .flatten()
                    .filter(|xy| input[at(xy)] != '#'),
            );
            /*
            let mut count = 0;
            for y in 0..h as i64 {
                for x in 0..w as i64{
                    let h = hs.contains(&(x as i64, y as i64));
                    if h {
                        count += 1;
                    }
                    print!("{}", if input[at(&(x, y))] == '#' { '#' } else if h { 'O' } else {'.'});
                }
                println!()
            }
            */
            let mul = 5;
            let count = hs.iter().filter(|x| x.0 >= (mul -1) * w as i64 && x.0 < mul*w as i64 && x.1 >= (mul - 1) * h as i64 && x.1 < mul * h as i64).count();
            println!("{step} {count}");
            println!("min x {:?} max x {:?}", hs.iter().map(|x| x.0).min().unwrap(), hs.iter().map(|x| x.0).max().unwrap());
            println!("min y {:?} max y {:?}", hs.iter().map(|x| x.1).min().unwrap(), hs.iter().map(|x| x.1).max().unwrap());
            //let found = (0..w).map(|x| (0..h).filter(|y| input[x + y * w] != '#' && !hs.contains(&(x as i64, *y as i64))).count()).sum::<usize>();
            //println!("{step} {}", found);
            hs
        },
    );

    Some(hs.len())
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c, 64);
    //let b = solution_b(&c, 26501365);
    let b = solution_b(&c, 100);

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
        assert_eq!(solution_a(&data, 6), Some(16));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        let steps = vec![
            (6, 16),
            (10, 50),
            (50, 1594),
            (100, 6536),
            (500, 167004),
            (1000, 668697),
            (5000, 16733044),
        ];
        steps
            .iter()
            .for_each(|(steps, result)| assert_eq!(solution_b(&data, *steps), Some(*result)));
    }

    #[test]
    fn test_draw() {
        let data = simple().unwrap();
        let data = content().unwrap();
        assert_eq!(solution_b(&data, 131 * 100), Some(1594+1));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c, 64), Some(3699));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c, 26501365), Some(0));
    }
}
