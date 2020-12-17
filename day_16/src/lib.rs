use regex::Regex;

struct Rule {
    name: String,
    low_range: (u32, u32),
    high_range: (u32, u32),
}

impl Rule {

    fn new(name: &str, low_range: (u32, u32), high_range: (u32, u32)) -> Rule {
        Rule {
            name: name.to_string(),
            low_range,
            high_range,
        }
    }
}

fn get_rules(text: &str) -> Vec<Rule> {
    text.lines().map(|line| {
        let re = regex::Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let cap = re.captures(line).unwrap();
        Rule::new(
            &cap[1],
            (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
            (cap[4].parse().unwrap(), cap[5].parse().unwrap()))
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::get_rules;
    use std::fs;

    #[test]
    fn test_get_rules() {
        let text = fs::read_to_string("data/example_rules.txt").unwrap();
        let rules = get_rules(&text);
        assert_eq!("class", rules[0].name);
        assert_eq!((1, 3), rules[0].low_range);
        assert_eq!((5, 7), rules[0].high_range);
    }
}
