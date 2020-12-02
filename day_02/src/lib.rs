use regex;
use std::fs;

struct Entry {
    bounds: (u32, u32),
    character: String,
    password: String,
}

impl Entry {
    fn new(bounds: (u32, u32), character: String, password: String) -> Entry {
        if character.len() < 1 || character.len() > 1 {
            panic!("Character is a word!");
        }
        Entry {
            bounds,
            character,
            password,
        }
    }
}

fn get_entry_from_text(text: &str) -> Entry {
    let re = regex::Regex::new(r"^(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)$").unwrap();
    let cap = re.captures(text).unwrap();
    let bounds = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
    Entry::new(bounds, cap[3].to_string(), cap[4].to_string())
}

fn get_entries_from_text(filename: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    for line in fs::read_to_string(filename).unwrap().lines() {
        entries.push(get_entry_from_text(line));
    }
    entries
}

fn check_valid(entry: Entry) -> bool {
    let re = regex::Regex::new(&format!("[{}]", entry.character)).unwrap();
    let mut sum = 0;
    for _cap in re.captures_iter(&entry.password) {
        sum += 1;
    }
    entry.bounds.0 <= sum && sum <= entry.bounds.1
}

pub fn count_valid_entries(filename: &str) -> u32 {
    count_valid(filename, |e|{check_valid(e)})
}

pub fn count_new_valid_entries(filename: &str) -> u32 {
    count_valid(filename, |e|{check_new_valid(e)})
}

fn count_valid<F>(filename: &str, f: F) -> u32
    where F: Fn(Entry) -> bool {
    let entries = get_entries_from_text(filename);
    let mut nr_valid = 0;
    for entry in entries {
        if f(entry) {
            nr_valid += 1;
        }
    }
    nr_valid
}

fn check_new_valid(entry: Entry) -> bool {
    let i = (entry.bounds.0 - 1) as usize;
    let j = (entry.bounds.1 - 1) as usize;
    let password = entry.password;
    let first_match = password[i..i+1] == entry.character;
    let second_match = password[j..j+1] == entry.character;
    (first_match || second_match) && !(first_match && second_match)
}

#[cfg(test)]
mod tests {
    use crate::{get_entry_from_text, get_entries_from_text, check_valid, Entry, count_valid_entries, check_new_valid, count_new_valid_entries};

    #[test]
    fn one_entry() {
        let test_str = "1-3 a: abc";
        let entry = get_entry_from_text(test_str);
        assert_eq!(entry.bounds, (1, 3));
        assert_eq!(entry.character, String::from("a"));
        assert_eq!(entry.password, String::from("abc"));
    }

    #[test]
    fn task_example_from_file() {
        let entry = &get_entries_from_text("data/example.txt")[1];
        assert_eq!(entry.bounds, (1, 3));
        assert_eq!(entry.character, String::from("b"));
        assert_eq!(entry.password, String::from("cdefg"));
    }

    #[test]
    fn valid_entry() {
        let entry = Entry::new(
            (1, 3),
            "a".to_string(),
            "abcde".to_string());
        assert!(check_valid(entry));
    }

    #[test]
    fn invalid_entry() {
        let entry = Entry::new(
            (1, 3),
            "b".to_string(),
            "cdfg".to_string());
        assert!(!check_valid(entry));
    }

    #[test]
    fn nr_valid() {
        assert_eq!(2, count_valid_entries("data/example.txt"));
    }

    #[test]
    fn new_valid_entry() {
        let entry = Entry::new(
            (1, 3),
            "a".to_string(),
            "abcde".to_string()
        );
        assert!(check_new_valid(entry));
    }

    #[test]
    fn new_invalid_entry() {
        let entry = Entry::new(
            (1, 3),
            "b".to_string(),
            "cdefg".to_string()
        );
        assert!(!check_new_valid(entry));
    }

    #[test]
    fn nr_new_valid() {
        assert_eq!(1, count_new_valid_entries("data/example.txt"));
    }
}
