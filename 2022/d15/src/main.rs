use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

struct Sensor {
    sensor: (i64, i64),
    dist: usize,
}

struct Cave {
    sensors: Vec<Sensor>,
    mini: (i64, i64),
    maxi: (i64, i64),
}

impl Cave {
    fn new(input: &str) -> Self {
        let mut sensors = Vec::new();
        let mut coord = Vec::new();

        input
            .lines()
            .map(|line| line.trim())
            .filter(|x| !x.is_empty())
            .for_each(|line| {
                // Sensor at x=12, y=14: closest beacon is at x=10, y=16
                let nums: Vec<(i64, i64)> = String::from_iter(
                    line.chars()
                        .filter(|c| c.is_ascii_digit() || *c == ',' || *c == ':' || *c == '-'),
                )
                .split(":")
                .map(|xy| {
                    let p: Vec<i64> = xy
                        .split(",")
                        .map(|n| i64::from_str_radix(n, 10).unwrap())
                        .collect();
                    coord.push((p[0], p[1]));
                    (p[0], p[1])
                })
                .collect();
                let dist = Cave::manhattan(nums[0], nums[1]);
                sensors.push(Sensor {
                    sensor: nums[0],
                    dist,
                });
            });

        let x = match coord.iter().map(|xy| xy.0).minmax() {
            itertools::MinMaxResult::MinMax(a, b) => (a, b),
            _ => unreachable!("What?"),
        };
        let y = match coord.iter().map(|xy| xy.1).minmax() {
            itertools::MinMaxResult::MinMax(a, b) => (a, b),
            _ => unreachable!("What?"),
        };

        Cave {
            sensors,
            mini: (x.0, y.0),
            maxi: (x.1, y.1),
        }
    }

    fn hidden(&self, row: i64) -> usize {
        if row < self.mini.1 || row > self.maxi.1 {
            return (self.maxi.0 - self.mini.0) as usize;
        }
        let coverage: Vec<(i64, i64)> = self
            .sensors
            .iter()
            .map(|sb| {
                let distance = sb.dist as i64;
                if (row + distance < sb.sensor.1) || (row > sb.sensor.1 + distance) {
                    None
                } else {
                    let dy = (sb.sensor.1 - row).abs();
                    let delta = distance - dy;
                    Some((sb.sensor.0 - delta, sb.sensor.0 + delta))
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .sorted_by_key(|x| x.0)
            .collect();

        // [(-2, 2), (2, 14), (2, 2), (12, 12), (14, 18), (16, 24)]
        let (_, count) = coverage.iter().fold((self.mini.0, 0i64), |acc, element| {
            let (prev, count) = acc;

            if element.1 <= prev {
                acc
            } else if element.0 < prev {
                (element.1, count + (element.1 - prev))
            } else {
                (element.1, count + (element.1 - element.0))
            }
        });
        count as usize
    }

    fn not_covered(&self, row: i64, maxim: i64) -> Option<usize> {
        let coverage: Vec<(i64, i64)> = self
            .sensors
            .iter()
            .map(|sb| {
                let distance = sb.dist as i64;
                if (row + distance < sb.sensor.1) || (row > sb.sensor.1 + distance) {
                    None
                } else {
                    let dy = (sb.sensor.1 - row).abs();
                    let delta = distance - dy;
                    Some((sb.sensor.0 - delta, sb.sensor.0 + delta))
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .sorted_by_key(|x| x.0)
            .collect();

        // [(-2, 2), (2, 14), (2, 2), (12, 12), (14, 18), (16, 24)]
        let x = coverage.iter().fold(0i64, |prev, element| {
            if prev < element.0 || prev > element.1 {
                prev
            } else {
                maxim.min(element.1 + 1)
            }
        });
        if x < maxim {
            println!("{x} {row}");
            Some(row as usize + 4000000usize * x as usize)
        } else {
            None
        }
    }

    fn manhattan(a: (i64, i64), b: (i64, i64)) -> usize {
        let mx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
        let my = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
        (mx + my) as usize
    }
}
fn solution_a(input: &str, row: i64) -> Option<usize> {
    let cave = Cave::new(input);
    Some(cave.hidden(row))
}

fn solution_b(input: &str, maxim: i64) -> Option<usize> {
    let cave = Cave::new(input);
    for row in 0..maxim {
        let curr = cave.not_covered(row, maxim);
        if curr.is_some() {
            return curr;
        }
    }
    None
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c, 2000000);
    let b = solution_b(&c, 4000000);

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
        assert_eq!(solution_a(&data, 10), Some(26));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data, 20), Some(56000011));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c, 2000000).unwrap(), 5040643);
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c, 4000000), Some(11016575214126));
    }
}
