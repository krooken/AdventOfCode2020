use regex;

struct Entry {
    bounds: (u32, u32),
    character: String,
    password: String,
}

impl Entry {
    fn new(bounds: (u32, u32), character: String, password: String) -> Entry {
        if character.len() < 1 || character.len() > 1 {
            panic!("Character is a word!");
        }
        Entry {
            bounds,
            character,
            password,
        }
    }
}

fn get_entry_from_text(text: &str) -> Entry {
    let re = regex::Regex::new(r"^(\d+)-(\d+) ([[:alpha:]]): ([[:alpha:]]+)$").unwrap();
    let cap = re.captures(text).unwrap();
    let bounds = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
    Entry::new(bounds, cap[3].to_string(), cap[4].to_string())
}

#[cfg(test)]
mod tests {
    use crate::get_entry_from_text;

    #[test]
    fn one_entry() {
        let test_str = "1-3 a: abc";
        let entry = get_entry_from_text(test_str);
        assert_eq!(entry.bounds, (1, 3));
        assert_eq!(entry.character, String::from("a"));
        assert_eq!(entry.password, String::from("abc"));
    }
}
