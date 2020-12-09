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

fn find_contiguous_sum(codes: &[u64], sum: u64) -> (usize, usize) {
    let mut low = 0;
    let mut high = 1;
    let mut current_sum = codes[low];
    while current_sum != sum {
        if current_sum < sum {
            current_sum += codes[high];
            high += 1;
        } else {
            current_sum -= codes[low];
            low += 1;
        }
    }
    (low, high)
}

pub fn get_min_max_sum(filename: &str, length: usize) -> u64 {
    let codes: Vec<u64> = fs::read_to_string(filename).unwrap().lines().map(|e| e.parse().unwrap()).collect();
    let invalid = get_invalid(&codes, length).unwrap();
    let (low, high) = find_contiguous_sum(&codes, invalid);
    codes[low..high].iter().min().unwrap() + codes[low..high].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{is_valid, find_invalid, find_contiguous_sum, get_min_max_sum};

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

    #[test]
    fn test_find_contiguous_sum() {
        let codes: Vec<u64> = fs::read_to_string("data/example.txt").unwrap().lines().map(|e| e.parse().unwrap()).collect();
        assert_eq!((2, 6), find_contiguous_sum(&codes, 127));
    }

    #[test]
    fn test_get_min_max_sum() {
        assert_eq!(62, get_min_max_sum("data/example.txt", 5));
    }
}
