use std::fs;
use regex;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

fn parse_passport(text: &str) -> Passport {
    let mut passport = Passport::new();
    let re_byr = regex::Regex::new(r"(byr):(\S*)").unwrap();
    match re_byr.captures(text) {
        Some(caps) => passport.byr = Some(caps[2].to_string()),
        None => (),
    }
    let re_iyr = regex::Regex::new(r"(iyr):(\S*)").unwrap();
    match re_iyr.captures(text) {
        Some(caps) => passport.iyr = Some(caps[2].to_string()),
        None => (),
    }
    let re_eyr = regex::Regex::new(r"(eyr):(\S*)").unwrap();
    match re_eyr.captures(text) {
        Some(caps) => passport.eyr = Some(caps[2].to_string()),
        None => (),
    }
    let re_hgt = regex::Regex::new(r"(hgt):(\S*)").unwrap();
    match re_hgt.captures(text) {
        Some(caps) => passport.hgt = Some(caps[2].to_string()),
        None => (),
    }
    let re_hcl = regex::Regex::new(r"(hcl):(\S*)").unwrap();
    match re_hcl.captures(text) {
        Some(caps) => passport.hcl = Some(caps[2].to_string()),
        None => (),
    }
    let re_ecl = regex::Regex::new(r"(ecl):(\S*)").unwrap();
    match re_ecl.captures(text) {
        Some(caps) => passport.ecl = Some(caps[2].to_string()),
        None => (),
    }
    let re_pid = regex::Regex::new(r"(pid):(\S*)").unwrap();
    match re_pid.captures(text) {
        Some(caps) => passport.pid = Some(caps[2].to_string()),
        None => (),
    }
    let re_cid = regex::Regex::new(r"(cid):(\S*)").unwrap();
    match re_cid.captures(text) {
        Some(caps) => passport.cid = Some(caps[2].to_string()),
        None => (),
    }
    passport
}

fn check_valid(passport: &Passport) -> bool {
    passport.byr != None &&
        passport.eyr != None &&
        passport.ecl != None &&
        passport.hcl != None &&
        passport.hgt != None &&
        passport.iyr != None &&
        passport.pid != None
}

fn parse_passports(text: &str) -> Vec<Passport> {
    let re = regex::Regex::new(r"^\W*$").unwrap();
    let mut text_with_delim = String::new();
    for line in text.lines() {
        if re.is_match(line) {
            text_with_delim.push_str(";;");
        } else {
            text_with_delim.push_str(&format!("{} ", line));
        }
    }
    let passports = text_with_delim.split(";;").collect::<Vec<&str>>();
    passports.iter().map(|entry| parse_passport(entry)).collect()
}

pub fn nr_valid_passports(filename: &str) -> u32 {
    let text = fs::read_to_string(filename).unwrap();
    let passports = parse_passports(&text);
    passports.iter()
        .filter(|passport| check_valid(passport))
        .fold(0, |acc, _| acc + 1)
}


#[cfg(test)]
mod tests {
    use crate::{parse_passport, check_valid, parse_passports, nr_valid_passports};
    use std::fs;

    #[test]
    fn test_parse_passport() {
        let text = "eyr:eyr_text";
        assert_eq!(Some("eyr_text".to_string()), parse_passport(text).eyr);
        assert_eq!(None, parse_passport(text).cid);
    }

    #[test]
    fn test_parse_multi_passport() {
        let text = "eyr:eyr_text cid:123\n\rbyr:test";
        assert_eq!(Some("eyr_text".to_string()), parse_passport(text).eyr);
        assert_eq!(Some("123".to_string()), parse_passport(text).cid);
        assert_eq!(Some("test".to_string()), parse_passport(text).byr);
        assert_eq!(None, parse_passport(text).pid);
    }

    #[test]
    fn test_valid_passport() {
        let text = "eyr:eyr_text cid:123\n\rbyr:test";
        assert!(!check_valid(&parse_passport(text)));
    }

    #[test]
    fn test_parse_passports() {
        let text = fs::read_to_string("data/example.txt").unwrap();
        let passports = parse_passports(&text);
        assert_eq!(Some("gry".to_string()), passports.first().unwrap().ecl);
        assert_eq!(Some("59in".to_string()), passports.last().unwrap().hgt);
    }

    #[test]
    fn test_nr_valid_passports() {
        assert_eq!(2, nr_valid_passports("data/example.txt"));
    }
}
