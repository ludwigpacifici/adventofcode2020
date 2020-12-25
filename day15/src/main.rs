use std::collections::HashMap;

fn main() {
    let numbers = parse("13,0,10,12,1,5,8");

    println!("part one: {:?}", part_one(&numbers, 2020));
    println!("part two: {:?}", part_one(&numbers, 30_000_000));
}

fn part_one(numbers: &[usize], n: usize) -> usize {
    let mut game = numbers
        .into_iter()
        .enumerate()
        .map(|(i, &n)| (n, i + 1))
        .collect::<HashMap<_, _>>();

    let mut last_spoke = *numbers.last().expect("There is no input numbers.");

    for i in numbers.len() + 1..=n {
        match game.get_mut(&last_spoke) {
            None => {
                game.insert(last_spoke, i - 1);
                last_spoke = 0;
            }
            Some(turns) => {
                last_spoke = i - 1 - *turns;
                *turns = i - 1;
            }
        }
    }

    last_spoke
}

fn parse(s: &str) -> Vec<usize> {
    s.split(',')
        .map(|n| n.parse().expect("Cannot parse number"))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_tests() {
        assert_eq!(part_one(&parse("0,3,6"), 10), 0);
        assert_eq!(part_one(&parse("0,3,6"), 2020), 436);
        assert_eq!(part_one(&parse("1,3,2"), 2020), 1);
        assert_eq!(part_one(&parse("2,1,3"), 2020), 10);
        assert_eq!(part_one(&parse("1,2,3"), 2020), 27);
        assert_eq!(part_one(&parse("2,3,1"), 2020), 78);
        assert_eq!(part_one(&parse("3,2,1"), 2020), 438);
        assert_eq!(part_one(&parse("3,1,2"), 2020), 1836);
    }

    #[test]
    fn part_two_tests() {
        assert_eq!(part_one(&parse("0,3,6"), 30_000_000), 175594);
        assert_eq!(part_one(&parse("1,3,2"), 30_000_000), 2578);
        assert_eq!(part_one(&parse("2,1,3"), 30_000_000), 3544142);
        assert_eq!(part_one(&parse("1,2,3"), 30_000_000), 261214);
        assert_eq!(part_one(&parse("2,3,1"), 30_000_000), 6895259);
        assert_eq!(part_one(&parse("3,2,1"), 30_000_000), 18);
        assert_eq!(part_one(&parse("3,1,2"), 30_000_000), 362);
    }
}
