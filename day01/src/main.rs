use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input = input
        .lines()
        .map(|n| n.parse::<i32>().expect("Cannot read integer"))
        .collect::<Vec<i32>>();

    println!("part one: {:?}", part_one(&input, 2020));
    println!("part two: {:?}", part_two(&input, 2020));

    Ok(())
}

fn part_one(input: &[i32], target: i32) -> Option<i32> {
    sum_two(&input, target).map(|(a, b)| a * b)
}

fn part_two(input: &[i32], target: i32) -> Option<i32> {
    sum_three(&input, target).map(|(a, b, c)| a * b * c)
}

fn sum_two(input: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut seen = HashSet::new();

    for &first in input {
        let second = target - first;
        if seen.contains(&second) {
            return Some((first, second));
        } else {
            seen.insert(first);
        }
    }
    None
}

fn sum_three(input: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    for &first in input {
        let rest = target - first;
        // To get the correct answer, looks like `first` does not need to be removed from the input list.
        if let Some((second, third)) = sum_two(input, rest) {
            return Some((first, second, third));
        }
    }

    None
}
