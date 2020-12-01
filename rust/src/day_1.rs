use itertools::Itertools;
use std::fs;

fn read_input(file: &str) -> Vec<i32> {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|x| x.parse::<i32>().expect("Couldn't parse line as i32"))
        .collect::<Vec<_>>()
}

fn solve(input: &[i32], n: usize) -> i32 {
    input
        .iter()
        .copied()
        .combinations(n)
        .find(|c| c.iter().sum::<i32>() == 2020)
        .map(|c| c.iter().product())
        .expect("No items sum to 2020")
}

pub fn part_1(file: &str) {
    println!("{}", solve(&read_input(file), 2));
}

pub fn part_2(file: &str) {
    println!("{}", solve(&read_input(file), 3));
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
