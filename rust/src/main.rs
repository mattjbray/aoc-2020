use clap::{App, Arg};

fn main() {
    let matches = App::new("Advent of Code 2020")
        .author("Matt Bray")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PART")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    match (
        matches.value_of("day").unwrap(),
        matches.value_of("part").unwrap(),
    ) {
        ("1", "1") => day_1::part_1(),
        ("1", "2") => day_1::part_2(),
        _ => println!("Not solved yet"),
    }
}

mod day_1 {
    use itertools::Itertools;
    use std::fs;

    fn read_input() -> Vec<i32> {
        let contents =
            fs::read_to_string("../data/day_1.txt").expect("Something went wrong reading the file");

        contents
            .lines()
            .map(|x| x.parse::<i32>().expect("Couldn't parse line as i32"))
            .collect::<Vec<_>>()
    }

    fn solve(input: &[i32], n: usize) -> i32 {
        for c in input.iter().combinations(n) {
            let sum: i32 = c.clone().into_iter().sum();
            if sum == 2020 {
                return c.clone().into_iter().product();
            }
        }
        return 0;
    }

    pub fn part_1() {
        println!("{}", solve(&read_input(), 2));
    }

    pub fn part_2() {
        println!("{}", solve(&read_input(), 3));
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn input() -> Vec<i32> {
            return vec![1721, 979, 366, 299, 675, 1456];
        }

        #[test]
        fn part_1() {
            assert_eq!(solve(&input(), 2), 514579)
        }

        #[test]
        fn part_2() {
            assert_eq!(solve(&input(), 3), 241861950)
        }
    }
}
