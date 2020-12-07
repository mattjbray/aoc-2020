use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Passport<'a> = HashMap<&'a str, &'a str>;

fn parse_passport(input: &str) -> Passport {
    lazy_static! {
        static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
        static ref YEAR_RE: Regex = Regex::new(r"\d{4}").unwrap();
    }
    WHITESPACE_RE
        .split(input.trim())
        .map(|kv| {
            let mut iter = kv.splitn(2, ":");
            let k = iter.next().expect("no key!");
            let v = iter.next().expect("no value!");
            (k, v)
        })
        .collect::<HashMap<_, _>>()
}

fn parse(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|entry| parse_passport(entry))
        .collect::<Vec<_>>()
}

pub fn part_1(file: &str) {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let passports = parse(&contents);

    println!("{}", solve(&passports))
}

static REQUIRED_FIELDS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn is_valid(p: &Passport) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|field| *field == "cid" || p.contains_key(field))
}

fn solve(ps: &[Passport]) -> usize {
    ps.iter().filter(|p| is_valid(p)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_parse() {
        let passports: _ = parse(EXAMPLE);
        assert_eq!(passports[0]["ecl"], "gry");
        assert_eq!(passports[0]["pid"], "860033327");
    }

    #[test]
    fn test_is_valid() {
        let passports: _ = parse(EXAMPLE);
        assert!(is_valid(&passports[0]));
        assert!(!(is_valid(&passports[1])));
        assert!(is_valid(&passports[2]));
        assert!(!(is_valid(&passports[3])));
    }

    #[test]
    fn test_solve() {
        let passports: _ = parse(EXAMPLE);
        assert_eq!(solve(&passports), 2);
    }
}
