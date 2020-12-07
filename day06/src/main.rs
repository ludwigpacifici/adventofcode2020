use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("part one: {:?}", part_one(&input));
    println!("part two: {:?}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .fold(HashSet::new(), |any_yes, yes| {
                    any_yes.union(&yes).cloned().collect()
                })
                .len()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect::<HashSet<_>>())
                .into_iter()
                .fold(('a'..='z').collect::<HashSet<_>>(), |all_yes, yes| {
                    all_yes.intersection(&yes).cloned().collect()
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(
            part_one(
                "abc

a
b
c

ab
ac

a
a
a
a

b
"
            ),
            11
        );
    }

    #[test]
    fn part_two_test() {
        assert_eq!(
            part_two(
                "abc

a
b
c

ab
ac

a
a
a
a

b
"
            ),
            6
        );
    }
}
