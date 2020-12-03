use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq)]
struct Map {
    width: usize,
    height: usize,
    trees: HashSet<(usize, usize)>,
}

fn parse(input: &str) -> Map {
    let width = input.lines().next().expect("No lines!").len();
    let height = input.lines().count();

    let trees = input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.trim()
                .char_indices()
                .filter_map(move |(i, c)| if c == '#' { Some((i, j)) } else { None })
        })
        .collect::<HashSet<_>>();
    Map {
        width,
        height,
        trees,
    }
}

fn read_input(file: &str) -> Map {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    parse(&contents)
}

fn count_trees(map: &Map, (right, down): &(usize, usize)) -> u32 {
    let mut x = 0;
    let mut y = 0;

    let mut count = 0;

    while y < map.height {
        if map.trees.contains(&(x, y)) {
            count += 1;
        }
        x = (x + right) % map.width;
        y = y + down;
    }

    count
}

pub fn part_1(file: &str) {
    let map = read_input(file);
    println!("{}", count_trees(&map, &(3, 1)));
}

fn slopes() -> Vec<(usize, usize)> {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
}

fn run_part_2(map: &Map) -> u32 {
    slopes()
        .iter()
        .map(|slope| count_trees(&map, &slope))
        .product()
}

pub fn part_2(file: &str) {
    let map = read_input(file);
    println!("{}", run_part_2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "..##.......
         #...#...#..
         .#....#..#.
         ..#.#...#.#
         .#...##..#.
         ..#.##.....
         .#.#.#....#
         .#........#
         #.##...#...
         #...##....#
         .#..#...#.#"
    }

    #[test]
    fn test_parse() {
        let parsed = parse(&input().lines().take(3).collect::<Vec<_>>().join("\n"));

        assert_eq!(
            parsed,
            Map {
                width: 11,
                height: 3,
                trees: [
                    (2, 0),
                    (3, 0),
                    (0, 1),
                    (4, 1),
                    (8, 1),
                    (1, 2),
                    (6, 2),
                    (9, 2)
                ]
                .iter()
                .copied()
                .collect::<HashSet<_>>()
            }
        );
    }

    #[test]
    fn test_run() {
        let parsed = parse(&input());
        assert_eq!(count_trees(&parsed, &(3, 1)), 7);
    }

    #[test]
    fn test_run_part_2() {
        let parsed = parse(&input());
        assert_eq!(run_part_2(&parsed), 336);
    }
}
