use std::io;
use std::io::prelude::*;
use std::{fs::File, str::FromStr};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let ids = input
        .lines()
        .map(|l| l.parse::<Seat>().expect("Cannot read sea"))
        .map(|s| Seat::id(&s))
        .collect::<Vec<_>>();

    println!("part one: {:?}", part_one(&ids));
    println!("part two: {:?}", part_two(&ids));

    Ok(())
}

fn part_one(ids: &[u64]) -> Option<&u64> {
    ids.iter().max()
}

fn part_two(ids: &[u64]) -> Option<u64> {
    let min = ids.iter().min()?;
    let max = ids.iter().max()?;
    let partial_sum: u64 = ids.iter().sum();

    let min_sum = (min - 1) * min / 2;
    let sum = max * (max + 1) / 2;
    Some(sum - (partial_sum + min_sum))
}

#[derive(Debug, PartialEq)]
struct Seat {
    row: u64,
    col: u64,
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let row = s[..7].iter().fold(Ok(0), |row, c| match c {
            b'F' => row.map(|x| x << 1),
            b'B' => row.map(|x| x << 1 | 1),
            c => Err(format!("Unknown row char: {:?}", c)),
        })?;

        let col = s[7..].iter().fold(Ok(0), |col, c| match c {
            b'L' => col.map(|x| x << 1),
            b'R' => col.map(|x| x << 1 | 1),
            c => Err(format!("Unknown row char: {:?}", c)),
        })?;

        Ok(Seat { row, col })
    }
}

impl Seat {
    fn id(s: &Seat) -> u64 {
        s.row * 8 + s.col
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn seat_from_str_test() {
        assert_eq!(Seat::from_str("FBFBBFFRLR"), Ok(Seat { row: 44, col: 5 }));
        assert_eq!(Seat::from_str("BFFFBBFRRR"), Ok(Seat { row: 70, col: 7 }));
        assert_eq!(Seat::from_str("FFFBBBFRRR"), Ok(Seat { row: 14, col: 7 }));
        assert_eq!(Seat::from_str("BBFFBBFRLL"), Ok(Seat { row: 102, col: 4 }));
    }

    #[test]
    fn seat_id_test() {
        assert_eq!(Seat::id(&Seat { row: 44, col: 5 }), 357);
        assert_eq!(Seat::id(&Seat { row: 70, col: 7 }), 567);
        assert_eq!(Seat::id(&Seat { row: 14, col: 7 }), 119);
        assert_eq!(Seat::id(&Seat { row: 102, col: 4 }), 820);
    }
}
