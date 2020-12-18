use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Passport<'a> = HashMap<&'a str, &'a str>;

fn parse_passport(input: &str) -> Passport {
    lazy_static! {
        static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
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

fn yr_valid(v: &str, min: u16, max: u16) -> bool {
    v.parse::<u16>()
        .map(|yr| yr >= min && yr <= max)
        .unwrap_or(false)
}

fn byr_valid(v: &str) -> bool {
    yr_valid(v, 1920, 2002)
}

fn iyr_valid(v: &str) -> bool {
    yr_valid(v, 2010, 2020)
}

fn eyr_valid(v: &str) -> bool {
    yr_valid(v, 2020, 2030)
}

fn hgt_valid(v: &str) -> bool {
    lazy_static! {
        static ref HGT_RE: Regex = Regex::new(r"^(?P<n>\d+)(?P<units>cm|in)$").unwrap();
    }
    HGT_RE
        .captures(v)
        .and_then(|caps| {
            caps["n"].parse::<u16>().ok().map(|n| match &caps["units"] {
                "cm" => n >= 150 && n <= 193,
                "in" => n >= 59 && n <= 76,
                _ => false,
            })
        })
        .unwrap_or(false)
}

fn hcl_valid(v: &str) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    HCL_RE.is_match(v)
}

fn ecl_valid(v: &str) -> bool {
    lazy_static! {
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }
    ECL_RE.is_match(v)
}

fn pid_valid(v: &str) -> bool {
    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    PID_RE.is_match(v)
}

fn is_valid2(p: &Passport) -> bool {
    p.iter().all(|(k, v)| match *k {
        "byr" => byr_valid(v),
        "iyr" => iyr_valid(v),
        "eyr" => eyr_valid(v),
        "hgt" => hgt_valid(v),
        "hcl" => hcl_valid(v),
        "ecl" => ecl_valid(v),
        "pid" => pid_valid(v),
        _ => true,
    })
}

fn solve_2(ps: &[Passport]) -> usize {
    ps.iter().filter(|p| is_valid(p) && is_valid2(p)).count()
}

pub fn part_2(file: &str) {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let passports = parse(&contents);

    println!("{}", solve_2(&passports))
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

    #[test]
    fn test_is_valid2() {
        assert_eq!(byr_valid("2002"), true);
        assert_eq!(byr_valid("2003"), false);

        assert_eq!(hgt_valid("60in"), true);
        assert_eq!(hgt_valid("190cm"), true);
        assert_eq!(hgt_valid("190in"), false);
        assert_eq!(hgt_valid("190"), false);

        assert_eq!(hcl_valid("#123abc"), true);
        assert_eq!(hcl_valid("#123abz"), false);
        assert_eq!(hcl_valid("123abc"), false);

        assert_eq!(ecl_valid("brn"), true);
        assert_eq!(ecl_valid("wat"), false);

        assert_eq!(pid_valid("000000001"), true);
        assert_eq!(pid_valid("0123456789"), false);

        let ex_invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        for p in parse(ex_invalid) {
            assert_eq!(is_valid2(&p), false);
        }

        let ex_valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        for p in parse(ex_valid) {
            assert_eq!(is_valid2(&p), true);
        }
    }
}
