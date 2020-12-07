
struct Rule {
    bag_name: String,
    content_strings: Vec<(u32, String)>,
}

fn get_rule(text: &str) -> Rule {
    let re_name = regex::Regex::new(r"(^[a-z\s]+) bags contain ").unwrap();
    let re_nothing = regex::Regex::new(r"([a-z\s]+) bags contain no other bags\.").unwrap();
    if re_nothing.is_match(text) {
        let cap = re_nothing.captures(text).unwrap();
        Rule {
            bag_name: cap[1].to_string(),
            content_strings: Vec::new(),
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_rule;

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
}
