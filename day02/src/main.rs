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
        .lines()
        .map(is_password_valid_old)
        .filter(|&b| b)
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(is_password_valid_new)
        .filter(|&b| b)
        .count()
}

fn parse(input: &str) -> (usize, usize, char, &str) {
    let mut input = input.split_whitespace();

    let mut rules = input
        .next()
        .expect("Cannot read rule")
        .split('-')
        .map(|n| n.parse::<usize>().expect("Cannot read number"));
    let lower = rules.next().expect("Cannot read lower rule");
    let upper = rules.next().expect("Cannot read upper rule");
    let char_rule = input
        .next()
        .expect("Cannot read char rule")
        .chars()
        .next()
        .expect("Cannot read first char");
    let password = input.next().expect("Cannot read password");

    (upper, lower, char_rule, password)
}

fn is_password_valid_old(input: &str) -> bool {
    let (upper, lower, char_rule, password) = parse(input);

    let count = password.chars().filter(|&c| c == char_rule).count();

    lower <= count && count <= upper
}

fn is_password_valid_new(input: &str) -> bool {
    let (upper, lower, char_rule, password) = parse(input);

    let c1 = password
        .chars()
        .nth(lower - 1)
        .expect("Cannot check first rule");

    let c2 = password
        .chars()
        .nth(upper - 1)
        .expect("Cannot check first rule");

    (c1 == char_rule) ^ (c2 == char_rule)
}

#[cfg(test)]
mod tests {
    use super::is_password_valid_old;

    #[test]
    fn is_password_valid_old_tests() {
        assert_eq!(is_password_valid_old("1-3 a: abcde"), true);
        assert_eq!(is_password_valid_old("1-3 b: cdefg"), false);
        assert_eq!(is_password_valid_old("2-9 c: ccccccccc"), true);
    }

    use super::is_password_valid_new;

    #[test]
    fn is_password_valid_new_tests() {
        assert_eq!(is_password_valid_new("1-3 a: abcde"), true);
        assert_eq!(is_password_valid_new("1-3 b: cdefg"), false);
        assert_eq!(is_password_valid_new("2-9 c: ccccccccc"), false);
    }
}
