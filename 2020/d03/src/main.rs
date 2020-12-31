use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut pos: [usize; 5] = [0; 5];
    let mut count: [u32; 5] = [0; 5];
    let mut y = 0u32;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for raw in lines {
            let line = raw.unwrap();
            let ln = line.len();

            for i in 0..5 {
                if y % steps[i].1 == 0 {
                    let n = pos[i] % ln;
                    if line.get(n..n + 1) == Some("#") {
                        count[i] += 1;
                    }
                    pos[i] += steps[i].0;
                }
            }

            y += 1;
        }
    }
    let mut res = 1;
    for i in 0..5 {
        res *= count[i];
        println!("Count {} ({}, {}) {}", i, steps[i].0, steps[i].1, count[i])
    }
    println!("Result {}", res);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
