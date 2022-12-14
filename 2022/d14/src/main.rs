use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Cave {
    rows: usize,
    cols: usize,
    orig: (usize, usize),
    data: Vec<u8>,
}

impl Cave {
    fn new(input: &str) -> Self {
        let read_line = |line: &str| -> Vec<(usize, usize)> {
            line.split(" -> ")
                .map(|coord| {
                    let p: [usize; 2] = coord
                        .split(',')
                        .map(|num| usize::from_str_radix(num, 10).unwrap())
                        .collect::<Vec<usize>>()
                        .try_into()
                        .unwrap();
                    (p[1], p[0])
                })
                .collect()
        };

        let pouring = (0, 500);
        let mut mini = pouring;
        let mut maxi = pouring;
        input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .for_each(|line| {
                read_line(line).iter().for_each(|p| {
                    let (r, c) = *p;
                    if r > maxi.0 {
                        maxi = (r, maxi.1)
                    }
                    if c > maxi.1 {
                        maxi = (maxi.0, c)
                    }
                    if r < mini.0 {
                        mini = (r, mini.1)
                    }
                    if c < mini.1 {
                        mini = (mini.0, c)
                    }
                })
            });
        println!("{:?} {:?}", mini, maxi);
        let rows = maxi.0 - mini.0 + 1;
        let cols = maxi.1 - mini.1 + 1;
        let n = rows * cols;
        let mut data: Vec<u8> = Vec::from_iter((0..=n).map(|_| b'.'));

        input
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .for_each(|line| {
                read_line(line)
                    .iter()
                    .map(|p| (p.0 - mini.0, p.1 - mini.1))
                    .reduce(|(pr, pc), (cr, cc)| {
                        if pr == cr {
                            let r = pr;
                            let c1 = pc.min(cc);
                            let c2 = pc.max(cc);

                            for c in c1..=c2 {
                                data.get_mut(r * cols + c).map(|x| *x = b'C');
                            }
                        } else if pc == cc {
                            let c = pc;
                            let r1 = pr.min(cr);
                            let r2 = pr.max(cr);

                            for r in r1..=r2 {
                                data.get_mut(r * cols + c).map(|x| *x = b'R');
                            }
                        }

                        (cr, cc)
                    });
            });
        Cave {
            rows,
            cols,
            orig: (pouring.0 - mini.0, pouring.1 - mini.1),
            data,
        }
    }

    fn draw(&self) {
        println!("dim: {} {}", self.rows, self.cols);
        println!("orig: {:?}", self.orig);
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{}", (self.data[self.pos(r, c)]) as char)
            }
            println!();
        }
    }

    fn pos(&self, r: usize, c: usize) -> usize {
        assert!(r < self.rows || c < self.cols, "Out!");
        r * self.cols + c
    }

    fn drops(&mut self) -> usize {
        let n = self.data.len();
        let start = self.orig;

        let mut rest = |rc: (usize, usize)| -> bool {
            let (mut r, mut c) = rc;
            while r < self.rows && c < self.cols {
                if self.data[self.pos(r, c)] != b'.' {
                    break;
                } else if r + 1 < self.rows && self.data[self.pos(r + 1, c)] == b'.' {
                    r += 1;
                } else if c == 0 {
                    break;
                } else if r + 1 < self.rows && self.data[self.pos(r + 1, c - 1)] == b'.' {
                    r += 1;
                    c -= 1;
                } else if c + 1 >= self.cols {
                    break;
                } else if r + 1 < self.rows && self.data[self.pos(r + 1, c + 1)] == b'.' {
                    r += 1;
                    c += 1;
                } else if r + 1 >= self.rows {
                    break;
                } else {
                    let p = self.pos(r, c);
                    self.data.get_mut(p).map(|x| *x = b'o');
                    return true;
                }
            }
            false
        };
        (0..n).take_while(|_| rest(start)).count()
    }
}

fn solution_a(input: &str) -> Option<usize> {
    let mut cave = Cave::new(input);
    let count = cave.drops();
    cave.draw();

    Some(count)
}

fn solution_b(input: &str) -> Option<usize> {
    Cave::new(input);
    None
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
        assert_eq!(solution_a(&data), Some(24 + 1));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(0));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(0));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(0));
    }
}
