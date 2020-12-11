use std::fs::File;
use std::io::prelude::*;
use std::{collections::HashSet, io};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let numbers = parse(&input);

    let invalid_number = part_one(&numbers, 25).expect("Part one is incorrect.");
    println!("part one: {:?}", invalid_number);
    println!("part two: {:?}", part_two(&numbers, invalid_number));

    Ok(())
}

fn part_one(numbers: &[u64], window_length: usize) -> Option<u64> {
    for (i, window) in numbers.windows(window_length).enumerate() {
        let target = numbers[window_length + i];
        if sum_two(window, target).is_none() {
            return Some(target);
        }
    }

    None
}

fn part_two(numbers: &[u64], target: u64) -> Option<u64> {
    let (start, end) = continuous_sum(numbers, target)?;
    let numbers = &numbers[start..=end];
    let min = numbers.iter().min()?;
    let max = numbers.iter().max()?;
    Some(min + max)
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|n| n.parse::<u64>().expect("Cannot parse input number"))
        .collect::<Vec<_>>()
}

fn continuous_sum(numbers: &[u64], target: u64) -> Option<(usize, usize)> {
    let mut start = 0;
    let mut sum_start_end = numbers[start];

    for end in 1..numbers.len() {
        sum_start_end += numbers[end];

        while sum_start_end > target && start < end {
            sum_start_end -= numbers[start];
            start += 1;
        }

        if sum_start_end == target {
            return Some((start, end));
        }
    }

    None
}

fn sum_two(numbers: &[u64], target: u64) -> Option<(u64, u64)> {
    let mut seen = HashSet::new();

    for &n in numbers {
        seen.insert(n);
        if target > n && seen.contains(&(target - n)) {
            return Some((n, target - n));
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(part_one(&parse(&input), 5), Some(127));
    }

    #[test]
    fn part_two_test() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(part_two(&parse(&input), 127), Some(62));
    }
}
