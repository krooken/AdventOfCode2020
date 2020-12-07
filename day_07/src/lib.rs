use std::collections::{HashMap, HashSet};
use std::fs;

struct Rule {
    bag_name: String,
    content_strings: Vec<(u32, String)>,
    contained_in: Vec<String>,
}

fn get_rule(text: &str) -> Rule {
    let re_name = regex::Regex::new(r"(^[a-z\s]+) bags contain ").unwrap();
    let re_nothing = regex::Regex::new(r"([a-z\s]+) bags contain no other bags\.").unwrap();
    if re_nothing.is_match(text) {
        let cap = re_nothing.captures(text).unwrap();
        Rule {
            bag_name: cap[1].to_string(),
            content_strings: Vec::new(),
            contained_in: Vec::new(),
        }
    } else {
        let re_contents = regex::Regex::new(r" (\d+) ([a-z\s]+) bags?(,|\.)").unwrap();
        let mut contents = Vec::new();
        for cap in re_contents.captures_iter(text) {
            contents.push((cap[1].parse().unwrap(), cap[2].to_string()));
        }
        let cap = re_name.captures(text).unwrap();
        Rule {
            bag_name: cap[1].to_string(),
            content_strings: contents,
            contained_in: Vec::new(),
        }
    }
}

pub fn count_bags_carrying_bag(filename: &str, bag: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    let map = construct_map(&text);
    let vec = go_up_in_map(&map, bag);
    let set: HashSet<_> = vec.iter().collect();
    set.len() as u32
}

fn go_up_in_map(map: &HashMap<String, Rule>, name: &str) -> Vec<String> {
    let next_rule = map.get(name).unwrap();
    if next_rule.contained_in.is_empty() {
        Vec::new()
    } else {
        let mut ancestors = Vec::new();
        for rule in next_rule.contained_in.iter() {
            ancestors.push(rule.to_string());
            let mut vec = go_up_in_map(map, rule);
            ancestors.append(&mut vec);
        }
        ancestors
    }
}

fn construct_map(text: &str) -> HashMap<String, Rule> {
    let mut map = HashMap::new();
    for line in text.lines() {
        let rule = get_rule(line);
        add_rule_to_map(&mut map, rule);
    };
    map
}

fn add_rule_to_map(map: &mut HashMap<String, Rule>, rule: Rule) {
    let mut current_rule: Rule;
    match map.remove(&rule.bag_name) {
        Some(found_rule) => {
            current_rule = found_rule;
            current_rule.content_strings = rule.content_strings;
        },
        None => current_rule = rule,
    };
    for (_, content_rule_string) in current_rule.content_strings.iter() {
        let mut content_rule: Rule;
        match map.remove(content_rule_string) {
            Some(found_rule) => {
                content_rule = found_rule;
                content_rule.contained_in.push(current_rule.bag_name.to_string());
            },
            None => {
                content_rule = Rule {
                    bag_name: content_rule_string.to_string(),
                    content_strings: Vec::new(),
                    contained_in: vec![current_rule.bag_name.to_string()],
                };
            }
        }
        map.insert(content_rule_string.to_string(), content_rule);
    }
    map.insert(current_rule.bag_name.to_string(), current_rule);
}

fn count_required_bags(map: &HashMap<String, Rule>, name: &str) -> u32 {
    let mut sum = 1;
    let rule = map.get(name).unwrap();
    for (num, bag) in rule.content_strings.iter() {
        sum += num*count_required_bags(map, bag);
    }
    sum
}

pub fn count_individual_bags(filename: &str, bag: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    let map = construct_map(&text);
    count_required_bags(&map, bag) - 1
}

#[cfg(test)]
mod tests {
    use crate::{get_rule, count_bags_carrying_bag, count_individual_bags};

    #[test]
    fn test_get_full_bag() {
        let text = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let rule = get_rule(text);
        assert_eq!("light red".to_string(), rule.bag_name);
        assert_eq!(1, rule.content_strings[0].0);
        assert_eq!(2, rule.content_strings[1].0);
        assert_eq!("bright white".to_string(), rule.content_strings[0].1);
        assert_eq!("muted yellow".to_string(), rule.content_strings[1].1);
    }

    #[test]
    fn test_get_empty_bag() {
        let text = "dotted black bags contain no other bags.";
        let rule = get_rule(text);
        assert_eq!("dotted black".to_string(), rule.bag_name);
        assert!(rule.content_strings.is_empty());
    }

    #[test]
    fn test_count_bags_carrying_bag() {
        let name = "shiny gold";
        assert_eq!(4, count_bags_carrying_bag("data/example.txt", name));
    }

    #[test]
    fn test_count_individual_bags() {
        assert_eq!(32, count_individual_bags("data/example.txt", "shiny gold"));
    }

    #[test]
    fn test_task1() {
        let name = "shiny gold";
        assert_eq!(335, count_bags_carrying_bag("data/bag_rules.txt", name));
    }

    #[test]
    fn test_task2() {
        assert_eq!(2431, count_individual_bags("data/bag_rules.txt", "shiny gold"));
    }
}
