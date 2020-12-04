use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input = parse(&input);
    println!("part one: {:?}", part_one(&input));
    println!("part two: {:?}", part_two(&input));

    Ok(())
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    count_trees(input, 3, 1)
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn count_trees(input: &Vec<Vec<char>>, dx: usize, dy: usize) -> usize {
    let w = input[0].len();
    input
        .iter()
        .step_by(dy)
        .enumerate()
        .filter(|(i, v)| v[(i * dx) % w] == '#')
        .count()
}

#[cfg(test)]
mod tests {

    use super::{count_trees, parse};

    #[test]
    fn count_trees_test() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let input = parse(&input);
        assert_eq!(count_trees(&input, 1, 1), 2);
        assert_eq!(count_trees(&input, 3, 1), 7);
        assert_eq!(count_trees(&input, 5, 1), 3);
        assert_eq!(count_trees(&input, 7, 1), 4);
        assert_eq!(count_trees(&input, 1, 2), 2);
    }
}
