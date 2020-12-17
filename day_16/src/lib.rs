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

    fn in_range(num: &u32, (low, high): &(u32, u32)) -> bool {
        low <= num && num <= high
    }

    fn valid_for_rule(&self, ticket_data: &u32) -> bool {
        Self::in_range(ticket_data, &self.low_range) || Self::in_range(ticket_data, &self.high_range)
    }
}

fn get_rules(text: &str) -> Vec<Rule> {
    text.lines().map(|line| {
        let re = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let cap = re.captures(line).unwrap();
        Rule::new(
            &cap[1],
            (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
            (cap[4].parse().unwrap(), cap[5].parse().unwrap()))
    }).collect()
}

fn get_ticket_data(text: &str) -> Vec<Vec<u32>> {
    text.lines().map(|line| {
        line.split(",").map(|n| n.parse().unwrap()).collect()
    }).collect()
}

fn get_valid_in_some(rules: &Vec<Rule>, ticket_data: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    ticket_data.iter().map(|data| {
        data.iter().map(|num| {
            rules.iter().fold(false, |acc, rule| {
                acc || rule.valid_for_rule(num)
            })
        }).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::{get_rules, get_ticket_data, get_valid_in_some};
    use std::fs;

    #[test]
    fn test_get_rules() {
        let text = fs::read_to_string("data/example_rules.txt").unwrap();
        let rules = get_rules(&text);
        assert_eq!("class", rules[0].name);
        assert_eq!((1, 3), rules[0].low_range);
        assert_eq!((5, 7), rules[0].high_range);
    }

    #[test]
    fn test_ticket_data() {
        let text = fs::read_to_string("data/example_nearby_tickets.txt").unwrap();
        let tickets = get_ticket_data(&text);
        assert_eq!(vec![7, 3, 47], tickets[0]);
    }

    #[test]
    fn test_valid_in_some() {
        let text = fs::read_to_string("data/example_rules.txt").unwrap();
        let rules = get_rules(&text);
        let text = fs::read_to_string("data/example_nearby_tickets.txt").unwrap();
        let tickets = get_ticket_data(&text);
        let valid = get_valid_in_some(&rules, &tickets);
        assert_eq!(vec![true, false, true], valid[1]);
    }
}
