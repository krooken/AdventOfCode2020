use std::collections::HashSet;
use std::fs;

fn get_group_answers(text: &str) -> HashSet<String> {
    let re = regex::Regex::new(r"[a-z]").unwrap();
    let mut answers = HashSet::new();
    for cap in re.captures_iter(text) {
        answers.insert(cap[0].to_string());
    }
    answers
}

fn get_flight_answers(text: &str) -> Vec<HashSet<String>> {
    let re = regex::Regex::new(r"(?m)^\W*$").unwrap();
    re.split(text).map(|group| get_group_answers(group)).collect()
}

pub fn count_flight_answers(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    get_flight_answers(&text).iter().fold(0, |acc, set| acc + set.len() as u32)
}

#[cfg(test)]
mod tests {
    use crate::{get_group_answers, get_flight_answers, count_flight_answers};
    use std::collections::HashSet;
    use std::fs;

    #[test]
    fn test_group_answers() {
        let text = "abcx\n\rabcy\n\rabcz";
        let mut set = HashSet::new();
        set.insert("a".to_string());
        set.insert("b".to_string());
        set.insert("c".to_string());
        set.insert("x".to_string());
        set.insert("y".to_string());
        set.insert("z".to_string());
        assert_eq!(set, get_group_answers(text));
    }

    #[test]
    fn test_flight_answers() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let flight = get_flight_answers(&text);
        assert_eq!(3, flight[0].len());
        assert_eq!(3, flight[1].len());
        assert_eq!(3, flight[2].len());
        assert_eq!(1, flight[3].len());
        assert_eq!(1, flight[4].len());
    }

    #[test]
    fn test_get_flight_count() {
        assert_eq!(11, count_flight_answers("data/example.txt"));
    }
}
