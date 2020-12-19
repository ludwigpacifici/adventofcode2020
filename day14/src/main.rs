use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let instructions = parse(&input).expect("Cannot parse instructions");

    println!("part one: {:?}", part_one(&instructions));
    println!("part two: {:?}", part_two(&instructions));

    Ok(())
}

fn part_one(instructions: &[Instruction]) -> u64 {
    fn logic(mask: &[u8], mut memory: HashMap<u64, u64>, write: &Write) -> HashMap<u64, u64> {
        let value = mask1(mask, write.value);
        *memory.entry(write.address).or_insert(0) = value;
        memory
    }

    part_inner(instructions, &logic)
}

fn part_two(instructions: &[Instruction]) -> u64 {
    fn logic(mask: &[u8], memory: HashMap<u64, u64>, write: &Write) -> HashMap<u64, u64> {
        let memory = Mask2::new(mask, write.address)
            .iter()
            .fold(memory, |mut memory, address| {
                *memory.entry(address).or_insert(0) = write.value;
                memory
            });

        memory
    }

    part_inner(instructions, &logic)
}

fn part_inner(
    instructions: &[Instruction],
    part_fn: &dyn Fn(&[u8], HashMap<u64, u64>, &Write) -> HashMap<u64, u64>,
) -> u64 {
    instructions
        .into_iter()
        .fold((HashMap::new(), &Vec::new()), |(memory, mask), i| match i {
            Instruction::Mask(m) => (memory, m),
            Instruction::Write(w) => (part_fn(mask, memory, w), mask),
        })
        .0
        .values()
        .sum()
}

fn mask1(mask: &[u8], value: u64) -> u64 {
    mask.iter().rev().enumerate().fold(value, |v, (i, &bit)| {
        if bit == b'1' {
            1 << i | v
        } else if bit == b'0' {
            !(1 << i) & v
        } else {
            v
        }
    })
}

struct Mask2<'a> {
    mask: &'a [u8],
    address: u64,
}

impl<'a> Mask2<'a> {
    fn new(mask: &[u8], address: u64) -> Mask2 {
        Mask2 { mask, address }
    }

    fn iter(&self) -> Mask2Iter {
        let ternary_mask = self.mask.iter().rev().enumerate().fold(
            Vec::new(),
            |mut floating_address, (i, &bit)| {
                if bit == b'1' {
                    floating_address.push(b'1');
                    floating_address
                } else if bit == b'0' {
                    if self.address & (1 << i) == 0 {
                        floating_address.push(b'0');
                    } else {
                        floating_address.push(b'1');
                    }
                    floating_address
                } else if bit == b'X' {
                    floating_address.push(b'X');
                    floating_address
                } else {
                    panic!(format!(
                        "Incorrect mask {:?} at position {:?}",
                        self.mask, i
                    ));
                }
            },
        );

        let state_count = 1 << ternary_mask.iter().filter(|&&x| x == b'X').count();
        let state = 0;

        Mask2Iter {
            ternary_mask,
            state_count,
            state,
        }
    }
}

struct Mask2Iter {
    ternary_mask: Vec<u8>,
    state_count: u64,
    state: u64,
}

impl Iterator for Mask2Iter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state >= self.state_count {
            return None;
        }

        let address = self
            .ternary_mask
            .iter()
            .fold((0, 0), |(address, i_state), &bit| {
                if bit == b'1' {
                    let address = address << 1 | 1;
                    (address, i_state)
                } else if bit == b'0' {
                    let address = address << 1;
                    (address, i_state)
                } else {
                    let inc = (self.state & (1 << i_state) == 0) as u64;
                    let address = address << 1 | inc;
                    (address, i_state + 1)
                }
            })
            .0;

        self.state += 1;

        Some(address)
    }
}

fn parse(s: &str) -> Result<Vec<Instruction>, String> {
    s.lines().map(|l| l.parse()).collect()
}

#[derive(Debug)]
enum Instruction {
    Mask(Vec<u8>),
    Write(Write),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let m = s
                .split(" = ")
                .nth(1)
                .ok_or("Cannot read mask")?
                .bytes()
                .collect();

            Ok(Instruction::Mask(m))
        } else {
            s.parse::<Write>().map(Instruction::Write)
        }
    }
}

#[derive(Debug)]
struct Write {
    address: u64,
    value: u64,
}

impl FromStr for Write {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" = ");
        let address = it
            .next()
            .ok_or("Cannot read left hand side for write instruction")?
            .split("[")
            .skip(1)
            .next()
            .ok_or("Cannot read the begginning of the address for write instruction")?
            .split("]")
            .next()
            .ok_or("Cannot read the address for write instruction")?
            .parse::<u64>()
            .map_err(|e| e.to_string())?;

        let value = it
            .next()
            .expect("Cannot read value for write instruction")
            .parse()
            .expect("Cannot parse value for write instruction");

        Ok(Write { address, value })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_tests() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let instructions = parse(&input).unwrap();
        assert_eq!(part_one(&instructions), 165)
    }

    #[test]
    fn part_two_tests() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let instructions = parse(&input).unwrap();
        assert_eq!(part_two(&instructions), 208)
    }
}
