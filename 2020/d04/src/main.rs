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

type State = (bool, String, String, u16, u32);

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // is_outside, collected, current_fact, current_facts, valid

        let valid_fact = |fact: &Facts, data: &String| -> bool {
            /*
                byr (Birth Year) - four digits; at least 1920 and at most 2002.
                iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                hgt (Height) - a number followed by either cm or in:
                If cm, the number must be at least 150 and at most 193.
                If in, the number must be at least 59 and at most 76.
                hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                pid (Passport ID) - a nine-digit number, including leading zeroes.
                cid (Country ID) - ignored, missing or not.
            */
            match fact {
                Facts::Cid => true,
                Facts::Byr => {
                    if data.len() != 4 {
                        false
                    } else {
                        if let Ok(n) = data.parse::<u16>() {
                            n >= 1920 && n <= 2002
                        } else {
                            false
                        }
                    }
                }
                Facts::Iyr => {
                    if data.len() != 4 {
                        false
                    } else {
                        if let Ok(n) = data.parse::<u16>() {
                            n >= 2010 && n <= 2020
                        } else {
                            false
                        }
                    }
                }
                Facts::Eyr => {
                    if data.len() != 4 {
                        false
                    } else {
                        if let Ok(n) = data.parse::<u16>() {
                            n >= 2020 && n <= 2030
                        } else {
                            false
                        }
                    }
                }
                Facts::Hgt => {
                    if data.ends_with("cm") && data.len() == 5 {
                        match data.get(..3) {
                            Some(three) => {
                                if let Ok(n) = three.parse::<u16>() {
                                    return n >= 150 && n <= 193;
                                }
                            }
                            None => return false,
                        }
                    }
                    if data.ends_with("in") && data.len() == 4 {
                        match data.get(..2) {
                            Some(two) => {
                                if let Ok(n) = two.parse::<u16>() {
                                    return n >= 59 && n <= 76;
                                }
                            }
                            None => return false,
                        }
                    }
                    return false;
                }
                Facts::Hcl => {
                    if data.len() == 7 {
                        let mut it = data.chars();
                        return it.nth(0) == Some('#')
                            && it.all(|x| (x >= '0' && x <= '9') || x >= 'a' && x <= 'f');
                    }
                    return false;
                }
                Facts::Ecl => match &data as &str {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false,
                },
                Facts::Pid => data.len() == 9 && data.chars().all(|x| (x >= '0' && x <= '9')),
            }
        };
        let start_state = |num: u32| -> State { (true, "".to_string(), "".to_string(), 0u16, num) };
        let mut state = start_state(0);
        let eval_state = |acc: State| -> State {
            let fact = Facts::from(&acc.2 as &str);
            if valid_fact(&fact, &acc.1) {
                let cf = 1u16 << fact as u8;
                if acc.3 & cf != 0u16 {
                    panic!("Fact already set");
                }
                if acc.3 | cf & 254 >= 254 {
                    start_state(acc.4 + 1)
                } else {
                    (false, "".to_string(), "".to_string(), acc.3 | cf, acc.4)
                }
            } else {
                println!("Invalid fact: {} {}", acc.2, acc.1);
                (true, "".to_string(), "".to_string(), acc.3, acc.4)
            }
        };

        let validate = |acc: State, e: char| -> State {
            // println!("{} {:?}", e, acc);
            match e {
                ':' => (false, "".to_string(), acc.1, acc.3, acc.4),

                ' ' => eval_state(acc),
                _ => {
                    let ext = format!("{}{}", acc.1, e);
                    (acc.0, ext, acc.2, acc.3, acc.4)
                }
            }
        };

        for raw in lines {
            let line = raw.unwrap();

            if line == "" {
                state = start_state(state.4);
                continue;
            }

            let it = line.chars();
            state = eval_state(it.fold(state, validate));
        }

        println!("Result {}", state.4);
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
