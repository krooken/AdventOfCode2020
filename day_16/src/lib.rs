use regex::Regex;
use std::fs;

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

pub fn count_valid_tickets(rules_file: &str, ticket_file: &str) -> u32 {
    let text = fs::read_to_string(rules_file).unwrap();
    let rules = get_rules(&text);
    let text = fs::read_to_string(ticket_file).unwrap();
    let tickets = get_ticket_data(&text);
    let valid = get_valid_in_some(&rules, &tickets);
    valid.iter().flatten().zip(tickets.iter().flatten()).fold(0, |acc, (b, num)| {
        acc + if *b {0} else {*num}
    })
}

fn find_assignment(possible_rules: &Vec<Vec<usize>>, used_rules: &mut Vec<usize>, index: usize) -> Option<Vec<usize>> {
    let mut result = None;
    if index == possible_rules.len() {
        result = Some(Vec::new());
    } else {
        for id in &possible_rules[index] {
            if used_rules.contains(&id) {
                continue;
            }
            let mut new_used = used_rules.clone();
            new_used.push(*id);
            result = match find_assignment(possible_rules, &mut new_used, index + 1) {
                Some(mut vec) => {
                    vec.push(*id);
                    Some(vec)
                },
                None => None,
            };
        }
    }
    result
}

fn get_field_names(rules: &Vec<Rule>, ticket_data: &Vec<Vec<u32>>) -> Vec<String> {
    let mut res = Vec::new();
    let mut possible_rules = Vec::new();
    for i in 0..ticket_data[0].len() {
        let mut possible = Vec::new();
        for (j, rule) in rules.iter().enumerate() {
            let mut valid = true;
            for row in ticket_data {
                if !rule.valid_for_rule(&row[i]) {
                    valid = false;
                    break;
                }
            }
            if valid {
                possible.push(j);
            }
        }
        possible_rules.push(possible);
    }
    let assignment = find_assignment(&possible_rules, &mut Vec::new(), 0).unwrap();
    for id in assignment.iter().rev() {
        res.push(rules[*id].name.to_string());
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::{get_rules, get_ticket_data, get_valid_in_some, count_valid_tickets, get_field_names};
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

    #[test]
    fn test_count_valid_tickets() {
        assert_eq!(71, count_valid_tickets("data/example_rules.txt", "data/example_nearby_tickets.txt"));
    }

    #[test]
    fn test_get_field_names() {
        let text = fs::read_to_string("data/example_rules.txt").unwrap();
        let rules = get_rules(&text);
        let text = fs::read_to_string("data/example_nearby_tickets.txt").unwrap();
        let tickets = get_ticket_data(&text);
        let valid = get_valid_in_some(&rules, &tickets);
        let tickets = valid.iter().zip(tickets.iter()).filter(|(valid_row, ticket_row)| {
            valid_row.iter().zip(ticket_row.iter()).fold(true, |acc, e| {
                acc && *e.0
            })
        }).map(|row| {
            row.1.clone()
        }).collect();
        assert_eq!(vec!["row", "class", "seat"], get_field_names(&rules, &tickets));
    }
}
