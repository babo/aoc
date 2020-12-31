use std::convert::From;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Facts {
    Cid = 0, // (Country ID)
    Byr,     // (Birth Year)
    Iyr,     // (Issue Year)
    Eyr,     // (Expiration Year)
    Hgt,     // (Height)
    Hcl,     // (Hair Color)
    Ecl,     // (Eye Color)
    Pid,     // (Passport ID)
}

impl From<&str> for Facts {
    fn from(item: &str) -> Self {
        return match item {
            "byr" => Facts::Byr,
            "iyr" => Facts::Iyr,
            "eyr" => Facts::Eyr,
            "hgt" => Facts::Hgt,
            "hcl" => Facts::Hcl,
            "ecl" => Facts::Ecl,
            "pid" => Facts::Pid,
            "cid" => Facts::Cid,
            _ => panic!("Invalid fact |{}|", item),
        };
    }
}

type State =  (bool, String, u16, u32);

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // is_outside, current_fact, current_facts, valid
        let start_state = |num: u32| -> State { (true, "".to_string(), 0u16, num) };
        let mut state = start_state(0);

        let validate = |acc: State, e: char| -> State {
            match e {
                ':' => {
                    let cf = 1u16 << Facts::from(&acc.1 as &str) as u8;
                    if acc.2 & cf != 0u16 {
                        panic!("Fact already set");
                    }
                    if acc.2 | cf & 254 >= 254 {
                        start_state(acc.3 + 1)
                    } else {
                        (false, "".to_string(), acc.2 | cf, acc.3)
                    }
                }
                ' ' => (true, "".to_string(), acc.2, acc.3),
                _ => {
                    if acc.0 {
                        let ext = format!("{}{}", acc.1, e);
                        (acc.0, ext, acc.2, acc.3)
                    } else {
                        acc
                    }
                }
            }
        };

        for raw in lines {
            let line = raw.unwrap();

            if line == "" {
                state = start_state(state.3);
                continue;
            }

            let it = line.chars();
            state = it.fold(state, validate);
            state = (true, "".to_string(), state.2, state.3);
        }

        println!("Result {}", state.3);
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
