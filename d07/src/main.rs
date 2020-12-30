use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::fs::{read_to_string};
    use regex::Regex;

    fn content() -> Result<String, io::Error> {
        Ok(read_to_string("./input.txt")?)
    }

    #[test]
    fn test_file_reading() -> Result<(), io::Error> {
        assert_ne!(content()?.len(), 0);
        Ok(())
    }

    #[test]
    fn test_line_reading() {
        let c = content().unwrap();
        let re = Regex::new(r"^((\w+) (\w+)) bags contain (\d+ \w+ \w+) .+? bags?\.$").unwrap();

        let mut count = 0usize;
        for line in c.lines() {
            if ! re.is_match(line) {
                println!("{}", line);
                count += 1;
            }
        }
        assert_eq!(count, 0);
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut count = 0usize;
        for raw in lines {
            let line = raw.unwrap();
            if line.len() > 0 {
                count += 1;
            }
        }
        println!("Count: {}", count);
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
