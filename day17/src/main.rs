use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input = input.parse::<Conway>().expect("Cannot parse input");

    println!("part one: {:?}", part_one(input.clone()));
    println!("part two: {:?}", part_two(input));

    Ok(())
}

fn part_one(conway: Conway) -> usize {
    (0..6).fold(conway, |conway, _| conway.next_3d()).alive()
}

fn part_two(conway: Conway) -> usize {
    (0..6).fold(conway, |conway, _| conway.next_4d()).alive()
}

#[derive(Clone)]
struct Conway {
    state: HashSet<Coordinate>,
    min: Coordinate,
    max: Coordinate,
}

impl Conway {
    fn alive(&self) -> usize {
        self.state.len()
    }

    fn is_active(&self, c: &Coordinate) -> bool {
        self.state.contains(c)
    }

    fn rules(
        &self,
        current: Coordinate,
        active_neighbors: usize,
        mut state: HashSet<Coordinate>,
    ) -> HashSet<Coordinate> {
        if self.is_active(&current) && (active_neighbors == 2 || active_neighbors == 3) {
            state.insert(current);
        } else {
            if active_neighbors == 3 {
                state.insert(current);
            }
        }
        state
    }

    fn next_3d(self) -> Conway {
        let mut state = HashSet::new();
        for z in self.min.z..=self.max.z {
            for y in self.min.y..=self.max.y {
                for x in self.min.x..=self.max.x {
                    let current = Coordinate { x, y, z, w: 0 };
                    let active_neighbors = Coordinate::neighbors(&current)
                        .iter()
                        .filter(|n| n.w == 0)
                        .filter(|n| self.is_active(n))
                        .count();
                    state = self.rules(current, active_neighbors, state);
                }
            }
        }
        Conway::new(state)
    }

    fn next_4d(self) -> Conway {
        let mut state = HashSet::new();
        for w in self.min.w..=self.max.w {
            for z in self.min.z..=self.max.z {
                for y in self.min.y..=self.max.y {
                    for x in self.min.x..=self.max.x {
                        let current = Coordinate { x, y, z, w };
                        let active_neighbors = Coordinate::neighbors(&current)
                            .iter()
                            .filter(|c| self.is_active(c))
                            .count();
                        state = self.rules(current, active_neighbors, state);
                    }
                }
            }
        }

        Conway::new(state)
    }

    fn new(state: HashSet<Coordinate>) -> Conway {
        let min = Conway::boundaries_min(&state);
        let max = Conway::boundaries_max(&state);
        Conway { state, min, max }
    }

    fn boundaries_min(state: &HashSet<Coordinate>) -> Coordinate {
        let x = state.iter().map(|c| c.x).min().unwrap() - 1;
        let y = state.iter().map(|c| c.y).min().unwrap() - 1;
        let z = state.iter().map(|c| c.z).min().unwrap() - 1;
        let w = state.iter().map(|c| c.w).min().unwrap() - 1;
        Coordinate { x, y, z, w }
    }

    fn boundaries_max(state: &HashSet<Coordinate>) -> Coordinate {
        let x = state.iter().map(|c| c.x).max().unwrap() + 1;
        let y = state.iter().map(|c| c.y).max().unwrap() + 1;
        let z = state.iter().map(|c| c.z).max().unwrap() + 1;
        let w = state.iter().map(|c| c.w).max().unwrap() + 1;
        Coordinate { x, y, z, w }
    }
}

impl FromStr for Conway {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Coordinate {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                        w: 0,
                    }),
                    _ => None,
                })
            })
            .collect::<HashSet<_>>();

        Ok(Conway::new(state))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Coordinate {
    fn neighbors(c: &Coordinate) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        for w in c.w - 1..=c.w + 1 {
            for z in c.z - 1..=c.z + 1 {
                for y in c.y - 1..=c.y + 1 {
                    for x in c.x - 1..=c.x + 1 {
                        let neighbor = Coordinate { x, y, z, w };
                        if *c != neighbor {
                            neighbors.push(neighbor)
                        }
                    }
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_tests() {
        let input = ".#.
..#
###";

        let input = input.parse::<Conway>().expect("Cannot parse input");
        assert_eq!(part_one(input), 112)
    }

    #[test]
    fn part_two_tests() {
        let input = ".#.
..#
###";

        let input = input.parse::<Conway>().expect("Cannot parse input");
        assert_eq!(part_two(input), 848)
    }
}
