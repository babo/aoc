use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Fn;

fn to_seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

fn to_num(_low: char, high: char) -> impl Fn((i32, i32), char)  -> (i32, i32) {
    move |vn: (i32, i32), x: char| -> (i32, i32) {
        if x == high { (vn.0 + (1 << vn.1), vn.1 + 1) } else { (vn.0, vn.1 + 1) }
    }
}

fn to_location(boardingpass: &str) -> Option<(i32, i32, i32)> {
    if boardingpass.len() != 10 { return None; }

    let a = boardingpass.chars().rev().take(3);
    let b = boardingpass.chars().rev().skip(3).take(7);

    let col = a.fold((0, 0), to_num('L', 'R')).0;
    let row = b.fold((0, 0), to_num('F', 'B')).0;

    Some((row, col, to_seat_id(row, col)))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_num() {
        let tf = to_num('0', '1');

        assert_eq!(tf((0, 0), '0'), (0, 1));
        assert_eq!(tf((0, 0), '1'), (1, 1));
        assert_eq!(tf((0, 6), '1'), (64, 7));
        assert_eq!(tf((2, 0), '1'), (3, 1));
    }

    #[test]
    fn test_01() {
        assert_eq!(to_location("BFFFFFFRLL"), Some((64, 4, 516)));
        assert_eq!(to_location("BFFFFFBLRR"), Some((65, 3, 523)));

        assert_eq!(to_location("FBFBBFFRLR"), Some((44, 5, 357)));
        assert_eq!(to_location("BFFFBBFRRR"), Some((70, 7, 567)));
        assert_eq!(to_location("FFFBBBFRRR"), Some((14, 7, 119)));
        assert_eq!(to_location("BBFFBBFRLL"), Some((102, 4, 820)));
    }

    #[test]
    fn test_02() {
        assert_eq!(to_location("FL"), None);
        assert_eq!(to_location("BBFfBBFRLL"), None);
        assert_eq!(to_location("BBFFBBFRLa"), None);
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let mut maxi = 0;
        let mut occupied = [0; 1024];

        for raw in lines {
            let line = raw.unwrap();
            let seat = to_location(&line).take();
            if let Some((_, _, m)) = seat {
                occupied[m as usize] -= 2;
                occupied[(m + 1) as usize] += 1;
                occupied[(m - 1) as usize] += 1;

                if m > maxi {
                    maxi = m;
                }
            }
        }
        let mid = occupied.into_iter().position(|x| *x == 2);

        println!("Result {} {:?}", maxi, mid);
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
