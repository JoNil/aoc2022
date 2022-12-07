use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/7.txt");
pub static TEST_INPUT: &str = include_str!("../input/7_test.txt");

#[derive(Debug)]
enum Entry {
    File(i32),
    Dir(HashMap<String, Entry>),
}

impl Entry {
    fn insert_with_path(&mut self, path: &[&str], filename: String, size: i32) {
        match self {
            Entry::File(_) => panic!(),
            Entry::Dir(entries) => {
                if path.is_empty() {
                    entries.insert(filename, Entry::File(size));
                } else {
                    let (first, rest) = path.split_first().unwrap();

                    let entry = entries
                        .entry(first.to_string())
                        .or_insert_with(|| Entry::Dir(HashMap::new()));

                    entry.insert_with_path(rest, filename, size);
                }
            }
        }
    }

    fn all_dirs(&self) -> Vec<&Entry> {
        match self {
            Entry::File(_) => Vec::new(),
            Entry::Dir(content) => {
                let mut dirs = Vec::new();
                dirs.push(self);
                for entry in content.values() {
                    dirs.extend(entry.all_dirs());
                }
                dirs
            }
        }
    }

    fn size(&self) -> i32 {
        match self {
            Entry::File(size) => *size,
            Entry::Dir(content) => content.values().map(|e| e.size()).sum(),
        }
    }
}

fn parse(input: &str) -> Entry {
    let mut entry = Entry::Dir(HashMap::new());
    let mut current_path = Vec::new();

    for line in input.lines() {
        if line.starts_with('$') {
            let line = line.trim_start_matches('$').trim_start();

            if line.starts_with("cd") {
                let line = line.trim_start_matches("cd").trim_start();

                match line {
                    ".." => {
                        current_path.pop();
                    }
                    "/" => {
                        current_path = Vec::new();
                    }
                    segment => {
                        current_path.push(segment);
                    }
                }
            }
        } else if !line.starts_with("dir") {
            let (size, filename) = line.split_once(' ').unwrap();
            entry.insert_with_path(&current_path, filename.to_string(), size.parse().unwrap());
        }
    }

    entry
}

pub fn a(input: &str) -> i32 {
    let root = parse(input);

    root.all_dirs()
        .iter()
        .filter_map(|e| {
            let size = e.size();
            if size > 100_000 {
                None
            } else {
                Some(size)
            }
        })
        .sum::<i32>()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 95437);
    assert_eq!(a(INPUT), 2104783);
}

pub fn b(input: &str) -> i32 {
    let root = parse(input);

    let space_left = 70_000_000 - root.size();

    root.all_dirs()
        .iter()
        .filter_map(|e| {
            let size = e.size();
            if space_left + size >= 30_000_000 {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 24933642);
    assert_eq!(b(INPUT), 5883165);
}
