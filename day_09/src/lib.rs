fn is_valid(preamble: &[u32], next: u32) -> bool {
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

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::is_valid;

    #[test]
    fn test_is_valid1() {
        let codes: Vec<u32> = fs::read_to_string("data/example.txt").unwrap().lines().map(|e| e.parse().unwrap()).collect();
        assert!(is_valid(&codes[0..5], codes[5]));
    }

    #[test]
    fn test_is_not_valid1() {
        let codes: Vec<u32> = fs::read_to_string("data/example.txt").unwrap().lines().map(|e| e.parse().unwrap()).collect();
        assert!(!is_valid(&codes[9..14], codes[14]));
    }
}
