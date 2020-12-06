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
    get_flight_answers_generic(text, |group| get_group_answers(group))
}

fn get_flight_answers_generic<F>(text: &str, f: F) -> Vec<HashSet<String>>
where
    F: Fn(&str) -> HashSet<String>
{
    let re = regex::Regex::new(r"(?m)^\W*$").unwrap();
    re.split(text).map(|group| f(group)).collect()
}

pub fn count_flight_answers(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    get_flight_answers(&text).iter().fold(0, |acc, set| acc + set.len() as u32)
}

fn get_group_all(text: &str) -> HashSet<String> {
    let re = regex::Regex::new(r"[a-z]").unwrap();
    let answers: Vec<HashSet<String>> = text.lines().map(|line| {
        re.captures_iter(line).fold(HashSet::new(), |mut acc, cap| {
            acc.insert(cap[0].to_string());
            acc
        })
    }).filter(|set| set.len() > 0).collect();
    answers.iter().fold(answers[0].clone(), |acc, set| {
        acc.intersection(set).map(|elem| elem.to_string()).collect()
    })
}

fn get_flight_answers_all(text: &str) -> Vec<HashSet<String>> {
    get_flight_answers_generic(text, |elem| get_group_all(elem))
}

pub fn count_flight_answers_all(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    get_flight_answers_all(&text).iter().fold(0, |acc, set| acc + set.len() as u32)
}

#[cfg(test)]
mod tests {
    use crate::{get_group_answers, get_flight_answers, count_flight_answers, get_group_all, get_flight_answers_all, count_flight_answers_all};
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

    #[test]
    fn test_get_group_all_1() {
        let text = "abc";
        assert_eq!(3, get_group_all(&text).len());
    }

    #[test]
    fn test_get_group_all_2() {
        let text = "abcx\n\rabcy\n\rabcz";
        assert_eq!(3, get_group_all(&text).len());
    }

    #[test]
    fn test_get_group_all_3() {
        let text = "ab\n\rac";
        assert_eq!(1, get_group_all(&text).len());
    }

    #[test]
    fn test_get_group_all_4() {
        let text = "a\n\rb\n\rc";
        assert_eq!(0, get_group_all(&text).len());
    }

    #[test]
    fn test_get_group_all_5() {
        let text = "a\n\ra\n\ra";
        assert_eq!(1, get_group_all(&text).len());
    }

    #[test]
    fn test_flight_answers_all() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let flight = get_flight_answers_all(&text);
        assert_eq!(3, flight[0].len());
        assert_eq!(0, flight[1].len());
        assert_eq!(1, flight[2].len());
        assert_eq!(1, flight[3].len());
        assert_eq!(1, flight[4].len());
    }

    #[test]
    fn test_get_flight_count_all() {
        assert_eq!(6, count_flight_answers_all("data/example.txt"));
    }
}
