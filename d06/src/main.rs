use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut groups = 1;
        let mut answers = [0u32; 26];
        let ascii_a = 'a' as u8;
        let mut count = 0u32;

        for raw in lines {
            let line = raw.unwrap();
            if line == "" {
                groups += 1;
                count += answers.iter().sum::<u32>();
                answers = [0u32; 26];
            }
            answers = line.chars().fold(answers, |mut acc, x| {
                if x.is_ascii_lowercase() {
                    let pos = x as u8 - ascii_a;
                    acc[ pos as usize ] |= 1;
                    acc
                } else {
                    acc
                }
            });
        }
        count += answers.iter().sum::<u32>();
        println!("{} {}", groups, count);
    }
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
