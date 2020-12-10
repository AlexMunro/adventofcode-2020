use aoc2020::parse;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref COMMAND: Regex = Regex::new(r"^([a-z]+) ([\+\-]\d+)$").unwrap();
}

struct Command {
    operator: String,
    operand: isize,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Err("No entries to parse".to_string());
        }

        let captures = COMMAND.captures(s).unwrap();
        Ok(Self {
            operator: captures
                .get(1)
                .expect("Could not identify operator")
                .as_str()
                .to_string(),
            operand: captures
                .get(2)
                .expect("Could not identify operand")
                .as_str()
                .parse::<isize>()
                .unwrap(),
        })
    }
}

fn execute(program: Vec<Command>) -> isize {
    let mut instruction_ptr = 0;
    let mut acc = 0;

    let mut executed_indices = HashSet::<usize>::new();

    loop {
        if executed_indices.contains(&instruction_ptr) {
            return acc;
        }

        executed_indices.insert(instruction_ptr);

        let command = &program[instruction_ptr];
        match command.operator.as_str() {
            "acc" => acc += command.operand,
            "jmp" => instruction_ptr = (command.operand + instruction_ptr as isize) as usize - 1,
            "nop" => (),
            _ => panic!("Invalid operator"),
        }
        instruction_ptr += 1;
    }
}

fn execute_with_flip(program: &Vec<Command>, flipped_instr: usize) -> Option<isize> {
    if program[flipped_instr].operator == "acc" {
        return None;
    }

    let mut instruction_ptr = 0;
    let mut acc = 0;

    let mut executed_indices = HashSet::<usize>::new();

    loop {
        if instruction_ptr == program.len() {
            return Some(acc);
        }

        if executed_indices.contains(&(instruction_ptr as usize)) {
            return None;
        }

        executed_indices.insert(instruction_ptr as usize);

        let command = &program[instruction_ptr as usize];
        match command.operator.as_str() {
            "acc" => acc += command.operand,
            "jmp" => {
                if instruction_ptr as usize != flipped_instr {
                    instruction_ptr = (command.operand + instruction_ptr as isize) as usize - 1
                }
            }
            "nop" => {
                if instruction_ptr as usize == flipped_instr {
                    instruction_ptr = (command.operand + instruction_ptr as isize) as usize - 1
                }
            }
            _ => panic!("Invalid operator"),
        }
        instruction_ptr += 1;
    }
}

fn find_corrupted_instr(program: Vec<Command>) -> isize {
    (0..program.len())
        .find_map(|n| execute_with_flip(&program, n))
        .expect("Could not find corrupted instruction")
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let program = parse::<Command>(input)?.collect();
    println!("The answer to part one is {}", execute(program));
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let program = parse::<Command>(input)?.collect();
    println!(
        "The answer to part two is {}",
        find_corrupted_instr(program)
    );
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
#[test]
fn test_execute() {
    let example: Vec<Command> = [
        "acc +1", "nop +0", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ]
    .iter()
    .map(|s| Command::from_str(s).unwrap())
    .collect();

    assert_eq!(execute(example), 5);
}

#[test]
fn test_find_corrupted_instr() {
    let example: Vec<Command> = [
        "acc +1", "nop +0", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ]
    .iter()
    .map(|s| Command::from_str(s).unwrap())
    .collect();

    assert_eq!(find_corrupted_instr(example), 8);
}
