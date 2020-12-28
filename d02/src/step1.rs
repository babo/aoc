use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let sl = |ip: &String| -> Option<bool> {
        let dash = ip.chars().position(|c| c == '-').unwrap();
        let space = ip.chars().position(|c| c == ' ').unwrap();
        let mini = ip.get(0..dash).unwrap().parse::<usize>().unwrap();
        let maxi = ip.get(dash + 1..space).unwrap().parse::<usize>().unwrap();
        let to_find = ip.chars().nth(space + 1).unwrap();
        let count = ip.chars().skip(space + 3).filter(|c| *c == to_find).count();
        let cond = count >= mini && count <= maxi;
        return Some(cond);
    };

    let mut sledge = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if sl(&line.unwrap()) == Some(true) {
                sledge += 1;
            }
        }
        println!("Match {}", sledge);
    } else {
        println!("Sucked")
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
