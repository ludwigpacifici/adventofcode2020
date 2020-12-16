use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let (earliest_timestamp, buses) = parse(&input);

    println!("part one: {:?}", part_one(earliest_timestamp, &buses));
    println!("part two: {:?}", part_two(&buses));

    Ok(())
}

fn part_one(earliest_timestamp: u64, buses: &[Option<u64>]) -> Option<u64> {
    buses
        .into_iter()
        .filter_map(|&b| b)
        .map(|bus| (bus, bus - (earliest_timestamp % bus)))
        .min_by_key(|&(_, time)| time)
        .map(|(bus, time)| bus * time)
}

fn part_two(buses: &[Option<u64>]) -> u64 {
    let buses = buses
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if let Some(b) = b {
                Some((i as u64, *b))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Dummy parallel solution that takes too much time for my i5-7300 CPU

    let mut i = 0;
    loop {
        i += 1;
        if buses.par_iter().all(|(t, b)| (i + t) % b == 0) {
            return i;
        }
    }

    // Hint: use Chinese Remainder - https://www.dcode.fr/chinese-remainder
}

fn parse(s: &str) -> (u64, Vec<Option<u64>>) {
    let mut lines = s.lines();

    let earliest_timestamp = lines
        .next()
        .expect("Cannot read first line")
        .parse::<u64>()
        .expect("Cannot parse first line");

    let buses = lines
        .next()
        .expect("Cannot read second line")
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse().expect("Cannot read bus"))
            }
        })
        .collect();

    (earliest_timestamp, buses)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_tests() {
        let input = "939
7,13,x,x,59,x,31,19";
        let (earliest_timestamp, buses) = parse(&input);
        assert_eq!(part_one(earliest_timestamp, &buses), Some(295));
    }

    #[test]
    fn part_two_tests() {
        let input = "939
7,13,x,x,59,x,31,19";
        let (_, buses) = parse(&input);
        assert_eq!(part_two(&buses), 1068788);

        let input = "939
17,x,13,19";
        let (_, buses) = parse(&input);
        assert_eq!(part_two(&buses), 3417);

        let input = "939
67,7,59,61";
        let (_, buses) = parse(&input);
        assert_eq!(part_two(&buses), 754018);

        let input = "939
67,x,7,59,61";
        let (_, buses) = parse(&input);
        assert_eq!(part_two(&buses), 779210);

        let input = "939
67,7,x,59,61";
        let (_, buses) = parse(&input);
        assert_eq!(part_two(&buses), 1261476);

        let input = "939
1789,37,47,1889";
        let (_, buses) = parse(&input);
        assert_eq!(part_two(&buses), 1202161486);
    }
}
