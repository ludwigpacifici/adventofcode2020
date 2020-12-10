use std::io::prelude::*;
use std::{collections::HashSet, io};
use std::{fs::File, str::FromStr};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let instructions = input
        .parse::<Instructions>()
        .expect("Cannot read instructions");

    println!("part one: {:?}", part_one(&instructions));
    println!("part two: {:?}", part_two(instructions));

    Ok(())
}

fn part_one(instructions: &Instructions) -> i64 {
    let (is_looping, program_state) = is_looping(instructions);

    debug_assert!(is_looping);

    program_state.accumulator
}

fn part_two(mut instructions: Instructions) -> i64 {
    for modify_index in 0..instructions.0.len() {
        match instructions.0[modify_index] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(n) => {
                instructions.0[modify_index] = Instruction::Nop(n);
                let (is_looping, program_state) = is_looping(&instructions);
                if is_looping {
                    instructions.0[modify_index] = Instruction::Jmp(n);
                } else {
                    return program_state.accumulator;
                }
            }
            Instruction::Nop(n) => {
                instructions.0[modify_index] = Instruction::Jmp(n);
                let (is_looping, program_state) = is_looping(&instructions);
                if is_looping {
                    instructions.0[modify_index] = Instruction::Nop(n);
                } else {
                    return program_state.accumulator;
                }
            }
        }
    }

    unreachable!();
}

fn is_looping(instructions: &Instructions) -> (bool, ProgramState) {
    let mut program_state = ProgramState::default();
    let mut instructions_seen = HashSet::new();

    while !instructions_seen.contains(&program_state.pc) && program_state.pc < instructions.0.len()
    {
        instructions_seen.insert(program_state.pc);
        program_state = step(&program_state, &instructions);
    }

    (program_state.pc < instructions.0.len(), program_state)
}

enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let instruction = it.next().ok_or("Cannot read instruction".to_string())?;
        let number = it
            .next()
            .ok_or("Cannot read number".to_string())?
            .parse::<i64>()
            .map_err(|e| e.to_string())?;

        match instruction {
            "acc" => Ok(Instruction::Acc(number)),
            "jmp" => Ok(Instruction::Jmp(number)),
            "nop" => Ok(Instruction::Nop(number)),
            other_instruction => Err(format!("Unknown instruction: {:?}", other_instruction)),
        }
    }
}

struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|l| l.parse())
            .collect::<Result<_, _>>()
            .map(|x| Instructions(x))
    }
}

#[derive(Default)]
struct ProgramState {
    pc: usize,
    accumulator: i64,
}

fn step(p: &ProgramState, instructions: &Instructions) -> ProgramState {
    match instructions.0[p.pc] {
        Instruction::Acc(n) => ProgramState {
            accumulator: p.accumulator + n,
            pc: p.pc + 1,
        },
        Instruction::Jmp(n) => ProgramState {
            pc: ((p.pc as i64 + n) as usize),
            ..*p
        },
        Instruction::Nop(_) => ProgramState { pc: p.pc + 1, ..*p },
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instructions = input
            .parse::<Instructions>()
            .expect("Cannot read instructions");

        assert_eq!(part_one(&instructions), 5)
    }

    #[test]
    fn part_two_test() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instructions = input
            .parse::<Instructions>()
            .expect("Cannot read instructions");

        assert_eq!(part_two(instructions), 8)
    }
}
