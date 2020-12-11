use std::io;
use std::io::prelude::*;
use std::{collections::HashMap, fs::File};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let adapters = parse(&input);

    println!("part one: {:?}", part_one(&adapters));
    println!("part two: {:?}", part_two(&adapters));

    Ok(())
}

fn part_one(adapters: &[usize]) -> u64 {
    let jolt_differences = adapters
        .windows(2)
        .map(|adapters| adapters[1] - adapters[0] - 1)
        .fold(
            vec![
                0, 0,
                // built-in joltage adapter rated for 3 jolts higher than the highest-rated adapter in your bag.
                1,
            ],
            |mut jolt_differences, diff| {
                jolt_differences[diff] += 1;
                jolt_differences
            },
        );

    jolt_differences[0] * jolt_differences[2]
}

fn part_two(adapters: &[usize]) -> Option<u64> {
    arrangement_count(adapters)
}

fn arrangement_count(adapters: &[usize]) -> Option<u64> {
    let mut counts: HashMap<i64, u64> = HashMap::new();
    counts.insert(0, 1);

    for &adapter in
        // Skip the charging outlet
        &adapters[1..]
    {
        let c1 = *counts.entry(adapter as i64 - 1).or_insert(0);
        let c2 = *counts.entry(adapter as i64 - 2).or_insert(0);
        let c3 = *counts.entry(adapter as i64 - 3).or_insert(0);
        counts.insert(adapter as i64, c1 + c2 + c3);
    }

    let last = adapters.last().map(|&n| n as i64)?;

    counts.get(&last).cloned()
}

fn parse(input: &str) -> Vec<usize> {
    let mut adapters = input
        .lines()
        .map(|n| n.parse().expect("Cannot parse input number"))
        .collect::<Vec<_>>();

    adapters.push(0);
    adapters.sort();
    adapters
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(part_one(&parse(input)), 35);

        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(part_one(&parse(input)), 220);
    }

    #[test]
    fn part_two_test() {
        let input = "16
10
15
5
1
11
7
19
6
12
4
";
        assert_eq!(part_two(&parse(input)), Some(8));

        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(part_two(&parse(input)), Some(19208));
    }
}
