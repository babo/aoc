use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum FileSystemEntry {
    FileKind(String, usize),
    DirKind(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Folder {
    name: String,
    id: usize,
    size: usize,
    parent: Option<usize>,
    content: Vec<FileSystemEntry>,
}

struct FileSystem {
    root: Option<usize>,
    folders: Vec<Folder>,
    directory: HashMap<String, usize>,
}

impl Folder {
    pub fn new(name: &str, parent: Option<usize>) -> Self {
        let name = name.to_string();
        let content = Vec::<FileSystemEntry>::new();
        let id = 0;
        let size = 0usize;

        Folder {
            name,
            id,
            size,
            parent,
            content,
        }
    }

    pub fn assign(&mut self, id: usize) {
        self.id = id;
    }

    pub fn add(&mut self, entry: FileSystemEntry) {
        match &entry {
            FileSystemEntry::FileKind(_, size) => self.size += size,
            FileSystemEntry::DirKind(_id) => (),
        }
        self.content.push(entry);
    }
}

fn calc_subdirs(input: &str, parent: &str, start_: usize) -> (Vec<usize>, usize, usize) {
    let mut dirs = Vec::new();
    let mut it = input.lines().skip(start_);
    let mut count = 0usize;
    let mut total = 0usize;

    loop {
        let adv = match it.next() {
            Some(line) => {
                count += 1;
                if line.starts_with("$ cd ..") {
                    break;
                } else if line.starts_with("$ cd ") {
                    let (subdirs, size, adv) = calc_subdirs(input, &line[4..], start_ + count);
                    dirs.extend(subdirs.iter());
                    total += size;
                    count += adv;
                    adv
                } else if line.starts_with("$ ls") {
                    0
                } else if line.starts_with("dir ") {
                    0
                } else {
                    let spc = line.find(" ").unwrap();
                    total += usize::from_str_radix(&line[..spc], 10).unwrap();
                    0
                }
            }
            None => break,
        };
        for _ in 0..adv {
            it.next();
        }
    }
    dirs.push(total);
    (dirs, total, count)
}

fn solution_a(input: &str) -> Option<usize> {
    let dirs = calc_subdirs(input, "/", 1).0;
    Some(dirs.into_iter().filter(|size| *size <= 100000).sum())
}

fn solution_b(input: &str) -> Option<usize> {
    let dirs: Vec<usize> = calc_subdirs(input, "/", 1)
        .0
        .iter()
        .sorted()
        .rev()
        .map(|x| *x)
        .collect();
    let used_space: usize = *dirs.get(0).unwrap();
    let free_space: usize = 70000000usize - used_space;
    let need = 30000000;
    if free_space > need {
        None
    } else {
        dirs.iter()
            .rev()
            .find(|x| *x + free_space >= need)
            .map(|x| *x)
    }
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
        assert_eq!(solution_a(&data), Some(95437));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(24933642));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(1583951));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(214171));
    }
}
