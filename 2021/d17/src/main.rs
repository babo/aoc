use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

// target area: x=20..30, y=-10..-5
// target area: x=257..286, y=-101..-57
fn read_target(input: &str) -> Option<((i32, i32), (i32, i32))> {
    input.lines().next().map(|line| {
        let xp = line.find('=').map(|x| x + 1).unwrap();
        let yp = line.rfind('=').map(|x| x + 1).unwrap();
        let comma = line.find(',').unwrap();

        let x = line
            .get(xp..comma)
            .map(|s| {
                s.split("..")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .unwrap();
        let y = line
            .get(yp..)
            .map(|s| {
                s.split("..")
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .unwrap();

        ((x[0], x[1]), (y[0], y[1]))
    })
}

fn trajectory_x(velocity_x: i32, target: (i32, i32)) -> (Vec<usize>, i32, bool) {
    let mut x = 0;
    let mut velocity = velocity_x;
    let mut hits: Vec<usize> = Vec::new();
    for i in 0.. {
        if x >= target.0 && x <= target.1 {
            hits.push(i)
        }
        if x > target.1 || velocity == 0 {
            break;
        }
        x += velocity;
        if velocity > 0 {
            velocity -= 1;
        } else if velocity < 0 {
            velocity += 1;
        }
    }
    (hits, velocity_x, velocity == 0)
}

fn trajectory_y(velocity_y: i32, target: (i32, i32)) -> (Vec<usize>, i32, i32) {
    let mut y = 0;
    let mut my = 0;
    let mut velocity = velocity_y;
    let mut hits: Vec<usize> = Vec::new();

    for i in 0.. {
        my = my.max(y);
        if y >= target.0 && y <= target.1 {
            hits.push(i);
        }
        if y < target.0 {
            break;
        }
        y += velocity;
        velocity -= 1;
    }
    return (hits, velocity_y, my);
}

fn solution_a(input: &str) -> Option<i32> {
    read_target(input).map(|(_target_x, target_y)| {
        let mut max_y = -1;
        for i in 0..1000 {
            let step = trajectory_y(i, target_y);
            if !step.0.is_empty() {
                if step.2 >= max_y {
                    max_y = step.2;
                } else {
                    break;
                }
            };
        }
        max_y
    })
}

fn solution_b(input: &str) -> Option<usize> {
    read_target(input).map(|(target_x, target_y)| {
        let y: Vec<(usize, i32)> = (target_y.0..=-target_y.0)
            .map(|v| trajectory_y(v, target_y))
            .filter(|res| !res.0.is_empty())
            .map(|r| r.0.iter().map(move |i| (*i, r.1)).collect::<Vec<_>>())
            .flatten()
            .collect();

        let mut all = HashSet::new();
        let x: usize = (1..=target_x.1)
            .map(|v| trajectory_x(v, target_x))
            .filter(|res| !res.0.is_empty())
            .map(|(hits, velocity_x, stop)| {
                let combo: Vec<(i32, i32)> = hits
                    .iter()
                    .map(|i| {
                        y.iter()
                            .filter(|j| {
                                if stop {
                                    j.0 >= *hits.get(0).unwrap()
                                } else {
                                    j.0 == *i
                                }
                            })
                            .map(|j| (velocity_x, j.1))
                            .collect::<Vec<(i32, i32)>>()
                    })
                    .flatten()
                    .collect(); //collect::<Vec<(i32, i32)>>()
                combo.iter().for_each(|xy| {
                    all.insert(*xy);
                });
                combo.iter().count()
            })
            .sum();

        all.iter()
            .sorted()
            .for_each(|xy| println!("{},{}", xy.0 + 1, xy.1 + 1));
        all.iter().count()
    })
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
        assert_eq!(solution_a(&data), Some(45));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(112));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(5050));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(2223));
    }

    fn trajectory(velocity: (i32, i32), target: ((i32, i32), (i32, i32))) -> i32 {
        let (mut vx, mut vy) = velocity;
        let mut x = 0;
        let mut y = 0;
        let mut max_y = 0;

        for step in 0..100 {
            println!("step: {}", step);
            if x >= target.0 .0 && x <= target.0 .1 && y >= target.1 .0 && y <= target.1 .1 {
                println!("hit: {} {}", x, y);
                break;
            }
            if x > target.0 .1.max(target.0 .0) || y < target.1 .0.min(target.1 .1) {
                println!("over: {} {}", x, y);
                break;
            }
            x += vx;
            y += vy;
            max_y = max_y.max(y);
            if vx > 0 {
                vx -= 1;
            }
            if vx < 0 {
                vx += 1;
            }
            vy -= 1;
        }
        max_y
    }

    #[test]
    fn test_height() {
        let velocity = (23, 100);
        let target = ((257, 286), (-101, -57));
        let height = trajectory(velocity, target);
        println!("{:?} {}", velocity, height);
        assert_eq!(height, 5050);
    }
}
