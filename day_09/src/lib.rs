use std::fs;

fn is_valid(preamble: &[u64], next: u64) -> bool {
    let mut result = false;
    for i in 0..preamble.len() {
        for j in i+1..preamble.len() {
            if preamble[i] + preamble[j] == next {
                result = true;
                break;
            }
        }
    }
    result
}

fn get_invalid(codes: &[u64], length: usize) -> Option<u64> {
    let mut result = None;
    for i in length..codes.len() {
        if !is_valid(&codes[i-length..i], codes[i]) {
            result = Some(codes[i]);
            break;
        }
    }
    result
}

pub fn find_invalid(filename: &str, length: usize) -> u64 {
    let codes: Vec<u64> = fs::read_to_string(filename).unwrap().lines().map(|e| e.parse().unwrap()).collect();
    get_invalid(&codes, length).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{is_valid, find_invalid};

    #[test]
    fn test_is_valid1() {
        let codes: Vec<u64> = fs::read_to_string("data/example.txt").unwrap().lines().map(|e| e.parse().unwrap()).collect();
        assert!(is_valid(&codes[0..5], codes[5]));
    }

    #[test]
    fn test_is_not_valid1() {
        let codes: Vec<u64> = fs::read_to_string("data/example.txt").unwrap().lines().map(|e| e.parse().unwrap()).collect();
        assert!(!is_valid(&codes[9..14], codes[14]));
    }

    #[test]
    fn test_find_invalid() {
        assert_eq!(127, find_invalid("data/example.txt", 5));
    }

    #[test]
    fn test_is_valid2() {
        let codes: Vec<u64> = fs::read_to_string("data/code.txt").unwrap().lines().map(|e| e.parse().unwrap()).collect();
        assert!(is_valid(&codes[8..33], codes[33]));
    }
}
