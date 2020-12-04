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

fn check_stricter_valid(passport: &Passport) -> bool {
    let mut valid = true;
    match &passport.byr {
        Some(year) => {
            if year.as_bytes().len() != 4 {
                valid = false;
            } else {
                match year.parse() {
                    Ok(n) => if 1920 > n || n > 2002 {valid = false},
                    Err(_) => valid = false,
                }
            }
        },
        None => valid = false,
    }
    match &passport.ecl {
        Some(color) => {
            let re = regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            if !re.is_match(&color) {
                valid = false;
            }
        },
        None => valid = false,
    }
    match &passport.eyr {
        Some(year) => {
            if year.as_bytes().len() != 4 {
                valid = false;
            } else {
                match year.parse() {
                    Ok(n) => if 2020 > n || n > 2030 {valid = false},
                    Err(_) => valid = false,
                }
            }
        },
        None => valid = false,
    }
    match &passport.hcl {
        Some(color) => {
            let re = regex::Regex::new(r"#[0-9a-f]{6}").unwrap();
            if !re.is_match(&color) {
                valid = false;
            }
        }
        None => valid = false,
    }
    match &passport.hgt {
        Some(height) => {
            let re = regex::Regex::new(r"(\d+)(cm|in)").unwrap();
            if !re.is_match(&height) {
                valid = false;
            } else {
                match re.captures(&height) {
                    Some(cap) => {
                        let hgt: u32 = cap[1].parse().unwrap();
                        match &cap[2] {
                            "cm" => {
                                if hgt < 150 || hgt > 193 {
                                    valid = false;
                                }
                            },
                            "in" => {
                                if hgt < 59 || hgt > 76 {
                                    valid = false;
                                }
                            },
                            _ => valid = false,
                        }
                    },
                    None => valid = false,
                }
            }
        },
        None => valid = false,
    }
    match &passport.iyr {
        Some(year) => {
            if year.as_bytes().len() != 4 {
                valid = false;
            } else {
                match year.parse() {
                    Ok(n) => if 2010 > n || n > 2020 {valid = false},
                    Err(_) => valid = false,
                }
            }
        },
        None => valid = false,
    }
    match &passport.pid {
        Some(id) => {
            let re = regex::Regex::new(r"^\d{9}$").unwrap();
            if !re.is_match(&id) {
                valid = false;
            }
        },
        None => valid = false,
    }
    valid
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
    use crate::{parse_passport, check_valid, parse_passports, nr_valid_passports, check_stricter_valid};
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

    #[test]
    fn test_strict_valid() {
        let text = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\rhcl:#623a2f";
        assert!(check_stricter_valid(&parse_passport(&text)));
    }

    #[test]
    fn test_strict_invalid() {
        let text = "eyr:1972 cid:100\n\rhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        assert!(!check_stricter_valid(&parse_passport(&text)));
    }
}
