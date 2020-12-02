use regex::Regex;
use std::fs;

struct Policy {
    char: char,
    min: usize,
    max: usize,
}

impl Policy {
    fn new(char: char, min: usize, max: usize) -> Self {
        Policy { char, min, max }
    }

    // The password policy indicates the lowest and highest number of times a
    // given letter must appear for the password to be valid. For example, 1-3 a
    // means that the password must contain a at least 1 time and at most 3
    // times.
    fn validate(&self, password: &str) -> bool {
        let occurrences = password.chars().filter(|&c| c == self.char).count();
        occurrences >= self.min && occurrences <= self.max
    }

    // Each policy actually describes two positions in the password, where 1
    // means the first character, 2 means the second character, and so on.
    // Exactly one of these positions must contain the given letter. Other
    // occurrences of the letter are irrelevant for the purposes of policy
    // enforcement.
    fn validate_2(&self, password: &str) -> bool {
        let x = password.chars().nth(self.min - 1).unwrap();
        let y = password.chars().nth(self.max - 1).unwrap();
        (x == self.char && y != self.char) || (x != self.char && y == self.char)
    }
}

impl std::str::FromStr for Policy {
    type Err = std::num::ParseIntError;

    // "1-3 a"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref POLICY_RE: Regex =
                Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>[a-z])$").unwrap();
        }
        let caps = POLICY_RE.captures(s).expect("Regex didn't match");
        let char: char = caps["char"].chars().next().unwrap();
        let min: usize = caps["min"].parse::<usize>()?;
        let max: usize = caps["max"].parse::<usize>()?;
        Ok(Policy::new(char, min, max))
    }
}

struct Row {
    policy: Policy,
    password: String,
}

impl Row {
    fn new(policy: Policy, password: String) -> Self {
        Row { policy, password }
    }

    fn validate(&self) -> bool {
        self.policy.validate(&self.password)
    }

    fn validate_2(&self) -> bool {
        self.policy.validate_2(&self.password)
    }
}

impl std::str::FromStr for Row {
    type Err = std::num::ParseIntError;

    // "1-3 a: abcde"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref POLICY_RE: Regex =
                Regex::new(r"^(?P<policy>.*): (?P<password>[a-z]+)$").unwrap();
        }
        let caps = POLICY_RE.captures(s).expect("Regex didn't match");
        let policy: Policy = caps["policy"].parse::<Policy>()?;
        let password: String = caps["password"].to_string();
        Ok(Row::new(policy, password))
    }
}

fn parse_input(contents: &str) -> Vec<Row> {
    contents
        .lines()
        .map(|x| x.trim().parse::<Row>().expect("Couldn't parse line"))
        .collect::<Vec<_>>()
}

fn read_input(file: &str) -> Vec<Row> {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    parse_input(&contents)
}

fn solve_1(input: &[Row]) -> usize {
    input.iter().filter(|r| r.validate()).count()
}

pub fn part_1(file: &str) {
    println!("{}", solve_1(&read_input(file)));
}

fn solve_2(input: &[Row]) -> usize {
    input.iter().filter(|r| r.validate_2()).count()
}

pub fn part_2(file: &str) {
    println!("{}", solve_2(&read_input(file)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Row> {
        let input = "1-3 a: abcde
                     1-3 b: cdefg
                     2-9 c: ccccccccc";

        parse_input(&input)
    }

    #[test]
    fn validate() {
        assert_eq!(Policy::new('a', 1, 3).validate("abcde"), true);
        assert_eq!(Policy::new('b', 1, 3).validate("cdefg"), false);
        assert_eq!(Policy::new('c', 2, 9).validate("ccccccccc"), true);
    }

    #[test]
    fn part_1() {
        assert_eq!(solve_1(&input()), 2)
    }

    #[test]
    fn validate_2() {
        assert_eq!(Policy::new('a', 1, 3).validate_2("abcde"), true);
        assert_eq!(Policy::new('b', 1, 3).validate_2("cdefg"), false);
        assert_eq!(Policy::new('c', 2, 9).validate_2("ccccccccc"), false);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_2(&input()), 1)
    }
}
