use aoc2020::parse;

use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

lazy_static! {
    static ref COMMAND: Regex = Regex::new(r"^([a-z])+ ([\+\-]\d+)$").unwrap();
}

fn execute(program: Vec<Command>)  -> isize {
    let instructionPtr = 0;
    let acc = 0;

    while instructionPtr < program.length() {
        let command = program[instructionPtr];
        match (command.operator.as_str()){
            "acc" => acc += command.operand,
            "jmp" => instructionPtr += command.operand as usize,
            "nop" => continue,
        }
    }
    acc
}

struct Command {
    operator: String,
    operand: isize,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = COMMAND.captures(s);
        Ok(Self {
            operator: captures.get(1).expect("Could not identify operator"),
            operand: from_str::<isize>(captures.get(2).expect("Could not identify operand")),
        })
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let program: Vec<String> = parse::<Command>(input)?.collect();
    println!("The answer to part one is {}", execute(program));
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
#[test]
fn test_execute(){
    let example = [
        "acc +1",
        "nop +0",
        "jmp +4",
        "acc +3",
        "jmp -3",
        "acc -99",
        "acc +1",
        "jmp -4",
        "acc +6",
    ].iter().map(|s| Command::from_string(s)).collect();

    assert_eq!(execute(example), 5)
}
