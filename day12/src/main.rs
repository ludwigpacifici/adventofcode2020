use std::io;
use std::io::prelude::*;
use std::{fs::File, str::FromStr};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input = parse(&input);

    println!("part one: {:?}", part_one(&input));
    println!("part two: {:?}", part_two(&input));

    Ok(())
}

fn part_one(input: &[Movement]) -> Result<u64, String> {
    let p = input
        .into_iter()
        .fold(Ok(DirectedPosition::default()), |p, m| {
            p.and_then(|p| p.goto(m))
        })?;

    Ok(norm1(p.x, p.y))
}

fn part_two(input: &[Movement]) -> Result<u64, String> {
    let w = Waypoint::default();
    let p = DirectedPosition::default();
    let p = input
        .into_iter()
        .fold(Ok((p, w)), |acc, m| acc.and_then(|(p, w)| w.goto(&p, m)))?
        .0;

    Ok(norm1(p.x, p.y))
}

fn norm1(x: isize, y: isize) -> u64 {
    (x.abs() + y.abs()) as u64
}

fn parse(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|l| l.parse::<Movement>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Cannot parse all the movement input.")
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cardinal {
    East,
    North,
    South,
    West,
}

impl Default for Cardinal {
    fn default() -> Self {
        Cardinal::East
    }
}

impl FromStr for Cardinal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E" => Ok(Cardinal::East),
            "N" => Ok(Cardinal::North),
            "S" => Ok(Cardinal::South),
            "W" => Ok(Cardinal::West),
            c => Err(format!("Unknow cardinal: {}", c)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left,
    Right,
}

impl Rotation {
    fn try_cardinal(&self, c: &Cardinal, degree: u64) -> Result<Cardinal, String> {
        match (self, degree) {
            (Rotation::Left, 90) | (Rotation::Right, 270) => match c {
                Cardinal::East => Ok(Cardinal::North),
                Cardinal::North => Ok(Cardinal::West),
                Cardinal::South => Ok(Cardinal::East),
                Cardinal::West => Ok(Cardinal::South),
            },
            (Rotation::Right, 90) | (Rotation::Left, 270) => match c {
                Cardinal::East => Ok(Cardinal::South),
                Cardinal::North => Ok(Cardinal::East),
                Cardinal::South => Ok(Cardinal::West),
                Cardinal::West => Ok(Cardinal::North),
            },
            (Rotation::Left, 180) | (Rotation::Right, 180) => match c {
                Cardinal::East => Ok(Cardinal::West),
                Cardinal::North => Ok(Cardinal::South),
                Cardinal::South => Ok(Cardinal::North),
                Cardinal::West => Ok(Cardinal::East),
            },
            (r, d) => Err(format!(
                "Unknow rotation to cardinal convertion: {:?} {:?}",
                r, d
            )),
        }
    }
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Rotation::Left),
            "R" => Ok(Rotation::Right),
            c => Err(format!("Unknow rotation: {}", c)),
        }
    }
}

#[derive(Debug)]
enum Movement {
    Absolute(Cardinal, u64),
    Rotation(Rotation, u64),
    Forward(u64),
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, n) = s.split_at(1);
        if let Ok(cardinal) = c.parse() {
            let n = n.parse::<u64>().map_err(|e| e.to_string())?;
            Ok(Movement::Absolute(cardinal, n))
        } else if let Ok(rotation) = c.parse() {
            let n = n.parse::<u64>().map_err(|e| e.to_string())?;
            Ok(Movement::Rotation(rotation, n))
        } else if let Ok('F') = c.parse::<char>() {
            let n = n.parse::<u64>().map_err(|e| e.to_string())?;
            Ok(Movement::Forward(n))
        } else {
            Err(format!("Unknow movement: {}", c))
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct DirectedPosition {
    x: isize,
    y: isize,
    c: Cardinal,
}

impl DirectedPosition {
    fn goto_coordinates(&self, c: &Cardinal, n: u64) -> DirectedPosition {
        let n = n as isize;
        match c {
            Cardinal::East => DirectedPosition {
                x: self.x + n,
                ..*self
            },
            Cardinal::North => DirectedPosition {
                y: self.y + n,
                ..*self
            },
            Cardinal::South => DirectedPosition {
                y: self.y - n,
                ..*self
            },
            Cardinal::West => DirectedPosition {
                x: self.x - n,
                ..*self
            },
        }
    }

    fn inc(&self, x: isize, y: isize) -> DirectedPosition {
        DirectedPosition {
            x: self.x + x,
            y: self.y + y,
            ..*self
        }
    }

    fn goto(&self, m: &Movement) -> Result<DirectedPosition, String> {
        match m {
            Movement::Absolute(cardinal, n) => Ok(self.goto_coordinates(cardinal, *n)),
            Movement::Forward(n) => Ok(self.goto_coordinates(&self.c, *n)),
            Movement::Rotation(rotation, degree) => {
                let c = rotation.try_cardinal(&self.c, *degree)?;
                let p = DirectedPosition { c, ..*self };
                Ok(p)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Waypoint {
    x: isize,
    y: isize,
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint { x: 10, y: 1 }
    }
}

impl Waypoint {
    fn goto_coordinates(&self, c: &Cardinal, n: isize) -> Waypoint {
        match c {
            Cardinal::East => Waypoint {
                x: self.x + n,
                ..*self
            },
            Cardinal::North => Waypoint {
                y: self.y + n,
                ..*self
            },
            Cardinal::South => Waypoint {
                y: self.y - n,
                ..*self
            },
            Cardinal::West => Waypoint {
                x: self.x - n,
                ..*self
            },
        }
    }

    fn rotate(&self, rotation: &Rotation, degree: u64) -> Result<Waypoint, String> {
        match (rotation, degree) {
            (Rotation::Left, 90) | (Rotation::Right, 270) => Ok(Waypoint {
                x: -self.y,
                y: self.x,
            }),
            (Rotation::Right, 90) | (Rotation::Left, 270) => Ok(Waypoint {
                x: self.y,
                y: -self.x,
            }),
            (Rotation::Left, 180) | (Rotation::Right, 180) => Ok(Waypoint {
                x: -self.x,
                y: -self.y,
            }),
            (r, d) => Err(format!(
                "Unknow rotation to cardinal convertion: {:?} {:?}",
                r, d
            )),
        }
    }

    fn goto(
        &self,
        p: &DirectedPosition,
        m: &Movement,
    ) -> Result<(DirectedPosition, Waypoint), String> {
        match m {
            Movement::Absolute(c, n) => {
                let n = *n as isize;
                let w = self.goto_coordinates(c, n);
                Ok((*p, w))
            }
            Movement::Forward(n) => {
                let n = *n as isize;
                let dx = self.x * n;
                let dy = self.y * n;
                let p = p.inc(dx, dy);

                Ok((p, *self))
            }
            Movement::Rotation(rotation, degree) => {
                let w = self.rotate(rotation, *degree)?;
                Ok((*p, w))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_tests() {
        let input = "F10
N3
F7
R90
F11";
        let input = parse(&input);
        assert_eq!(part_one(&input), Ok(25));
    }

    #[test]
    fn part_two_tests() {
        let input = "F10
N3
F7
R90
F11";
        let input = parse(&input);
        assert_eq!(part_two(&input), Ok(286));
    }
}
