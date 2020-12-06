use std::collections::HashSet;

fn get_group_answers(text: &str) -> HashSet<String> {
    let re = regex::Regex::new(r"[a-z]").unwrap();
    let mut answers = HashSet::new();
    for cap in re.captures_iter(text) {
        answers.insert(cap[0].to_string());
    }
    answers
}

#[cfg(test)]
mod tests {
    use crate::get_group_answers;
    use std::collections::HashSet;

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
}
